#[cfg(feature = "ssr")]
mod auth;
#[cfg(feature = "ssr")]
mod db;
#[cfg(feature = "ssr")]
mod domain;

pub mod server_fn;

#[cfg(feature = "ssr")]
type Pool = sqlx::Pool<sqlx::Sqlite>;

use std::{fmt::Display, str::FromStr};

#[cfg(feature = "ssr")]
use axum::extract::FromRef;
use leptos::{
    prelude::{FromServerFnError, ServerFnError, ServerFnErrorErr},
    server,
    server_fn::codec::JsonEncoding,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool,
    pub user_repository: db::UserRepository,
}
#[cfg(feature = "ssr")]
impl AppState {
    pub async fn new(database_url: &str) -> Self {
        let pool = Pool::connect(database_url).await.unwrap();

        Self {
            pool: pool.clone(),
            user_repository: db::UserRepository::new(pool.clone()),
        }
    }
}

#[server]
pub async fn get_pow() -> Result<String, ServerFnError> {
    use leptos_captcha::spow::pow::Pow;
    const DEV_MODE: bool = true;

    if DEV_MODE {
        Ok(Pow::with_difficulty(10, 10)?.to_string())
    } else {
        Ok(Pow::new(20)?.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum AuthError {
    PasswordMismatch,
    UsernameTaken,
    InvalidCredentials,
    ServerFnError(ServerFnErrorErr),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::PasswordMismatch => write!(f, "Password mismatch"),
            AuthError::UsernameTaken => write!(f, "Username taken"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::ServerFnError(err) => write!(f, "Server error: {err}"),
        }
    }
}

impl FromStr for AuthError {
    type Err = ServerFnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PasswordMismatch" => Ok(AuthError::PasswordMismatch),
            "UsernameTaken" => Ok(AuthError::UsernameTaken),
            "InvalidCredentials" => Ok(AuthError::InvalidCredentials),
            _ => Err(ServerFnError::ServerError(s.into())),
        }
    }
}

impl FromServerFnError for AuthError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AuthError::ServerFnError(value)
    }
}

#[cfg(feature = "ssr")]
pub async fn get_session_store(
    pool: Pool,
) -> axum_session::SessionStore<axum_session_sqlx::SessionSqlitePool> {
    use chrono::Duration;

    let session_config = axum_session::SessionConfig::default()
        .with_lifetime(Duration::days(365)) // short session
        .with_max_lifetime(Duration::days(365)) // max in DB
        .with_max_age(Some(Duration::days(365)))
        .with_always_save(false)
        .with_table_name("sessions_table");

    // create SessionStore and initiate the database tables

    axum_session::SessionStore::<axum_session_sqlx::SessionSqlitePool>::new(
        Some(pool.clone().into()),
        session_config,
    )
    .await
    .unwrap()
}
#[cfg(feature = "ssr")]
pub async fn get_auth_session(
    pool: Pool,
) -> axum_session_auth::AuthSessionLayer<
    crate::domain::user::User,
    uuid::Uuid,
    axum_session_sqlx::SessionSqlitePool,
    Pool,
> {
    use chrono::Duration;

    let auth_config =
        axum_session_auth::AuthConfig::<uuid::Uuid>::default().with_max_age(Duration::hours(6));
    axum_session_auth::AuthSessionLayer::<
        crate::domain::user::User,
        uuid::Uuid,
        axum_session_sqlx::SessionSqlitePool,
        sqlx::SqlitePool,
    >::new(Some(pool))
    .with_config(auth_config)
}

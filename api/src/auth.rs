use std::{fmt::Display, str::FromStr};

use crate::{Pool, domain::user::User};

use crate::{AppState, AuthError};
use axum_session_auth::Rights;
pub use axum_session_sqlx::SessionSqlitePool;
use leptos::prelude::*;
use leptos::prelude::{ServerFnError, use_context};
use leptos_axum::extract;
use uuid::Uuid;
pub async fn require_permission(permission: &Rights) -> bool {
    if let Some(api_user) = auth()
        .await
        .map(|auth| auth.current_user.clone())
        .ok()
        .flatten()
    {
        permission.evaluate(&api_user, &None).await
    } else {
        false
    }
}
/// Get the database connection pool
/// # Errors
/// Will return an error if the context is not available
pub fn pool() -> Result<Pool, ServerFnError> {
    use_context::<AppState>()
        .map(|x| x.pool.clone())
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}
/// Get the authentication session
/// # Errors
/// Will return an error if the context is not available
pub async fn auth() -> Result<AuthSession, ServerFnError> {
    extract::<AuthSession>()
        .await
        .or(Err(ServerFnError::ServerError(
            "Auth session missing.".into(),
        )))
}
pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionSqlitePool, Pool>;

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

#[cfg(feature = "ssr")]
/// Get the current user
/// # Errors
/// Will return an error if the context is not available
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use self::auth;
    let auth = auth().await?;

    Ok(auth.current_user)
}

#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    use crate::auth::get_user;
    let Some(user) = get_user().await? else {
        return Err(ServerFnError::ServerError("Not Logged in".into()));
    };
    Ok(user.permissions.contains("Admin") || user.permissions.contains("admin"))
}

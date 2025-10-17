use leptos::prelude::*;

use crate::AuthError;
#[server]
pub async fn signup(
    pow: String,
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
    next: Option<String>,
) -> Result<(), ServerFnError<AuthError>> {
    use crate::AppState;
    use crate::auth::auth;
    use leptos_captcha::spow::pow::Pow;
    use password_auth::generate_hash;
    Pow::validate(&pow).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let app_state = use_context::<AppState>()
        .ok_or_else(|| ServerFnError::ServerError("AppState not found".into()))?;
    let user_repo = app_state.user_repository;
    let pool = app_state.pool;
    let auth = auth()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    if password != password_confirmation {
        return Err(AuthError::PasswordMismatch.into());
    }

    // Check if username already exists
    let existing_user = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    if existing_user.is_some() {
        return Err(AuthError::UsernameTaken.into());
    }

    let password_hashed = generate_hash(password);
    let new_user = crate::domain::user::User::new(username.clone(), password_hashed.clone());
    let new_user_id = new_user.id;
    user_repo
        .create(new_user)
        .await
        .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

    auth.login_user(new_user_id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect(next.unwrap_or("/".to_string()).as_str());

    Ok(())
}

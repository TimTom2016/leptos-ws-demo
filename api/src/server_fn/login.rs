use crate::AuthError;
use leptos::prelude::*;

#[server]
pub async fn login(
    pow: String,
    username: String,
    password: String,
    remember: Option<String>,
    next: Option<String>,
) -> Result<(), ServerFnError<AuthError>> {
    use crate::AppState;
    use crate::auth::auth;
    use crate::db::UserRepository;
    use leptos_captcha::spow::pow::Pow;
    use password_auth::verify_password;
    Pow::validate(&pow).map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let app_state = use_context::<AppState>()
        .ok_or_else(|| ServerFnError::ServerError("AppState not found".into()))?;
    let user_repo = UserRepository::new(app_state.pool.clone());
    let auth = auth()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = user_repo
        .get_by_username(username)
        .await
        .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

    match verify_password(password, &user.password) {
        Ok(()) => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(next.unwrap_or("/".to_string()).as_str());
            Ok(())
        }
        Err(error) => Err(ServerFnError::ServerError(error.to_string())),
    }
}

use leptos::prelude::*;
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::auth::auth;

    let auth = auth().await?;

    auth.logout_user();
    Ok(())
}

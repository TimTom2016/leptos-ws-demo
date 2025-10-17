use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggedIn {
    pub username: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Account {
    NotLoggedIn,
    LoggedIn(LoggedIn),
}

impl Account {
    pub fn is_logged_in(&self) -> bool {
        matches!(self, Account::LoggedIn(_))
    }

    pub fn username(&self) -> Option<&str> {
        match self {
            Account::LoggedIn(logged_in) => Some(&logged_in.username),
            Account::NotLoggedIn => None,
        }
    }
}

#[server]
pub async fn get_account() -> Result<Account, ServerFnError> {
    use crate::auth::get_user;
    let user = get_user().await?;
    if let Some(user) = user {
        Ok(Account::LoggedIn(LoggedIn {
            username: user.username,
        }))
    } else {
        Ok(Account::NotLoggedIn)
    }
}

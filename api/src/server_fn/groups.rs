use std::str::FromStr;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
    pub last_message: String,
    pub join_code: String,
}

#[server]
pub async fn create_group(name: String, avatar: String) -> Result<(), ServerFnError> {
    use crate::AppState;
    use crate::domain::group::Group;
    let state = use_context::<AppState>().expect("AppState not found");
    use crate::auth::get_user;
    let user = get_user().await?;
    let Some(user) = user else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    };
    let group = Group::new_with_avatar(name, avatar);
    let group_id = state.group_repository.create_group(group).await?;
    state.group_repository.add_member(group_id, user.id).await?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinCode(String);

#[derive(thiserror::Error, Debug)]
pub enum JoinCodeError {
    #[error("Join code is too short")]
    TooShort,
    #[error("Join code is too long")]
    TooLong,
}

impl FromStr for JoinCode {
    type Err = JoinCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            Err(JoinCodeError::TooShort)
        } else if s.len() > 16 {
            Err(JoinCodeError::TooLong)
        } else {
            Ok(JoinCode(s.to_string()))
        }
    }
}

impl TryFrom<String> for JoinCode {
    type Error = JoinCodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err(JoinCodeError::TooShort)
        } else if value.len() > 8 {
            Err(JoinCodeError::TooLong)
        } else {
            Ok(JoinCode(value))
        }
    }
}

impl AsRef<str> for JoinCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[server]
pub async fn join_group(join_code: JoinCode) -> Result<(), ServerFnError> {
    use crate::AppState;
    let state = use_context::<AppState>().expect("AppState not found");
    use crate::auth::get_user;
    let user = get_user().await?;
    let Some(user) = user else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    };
    let Some(group) = state
        .group_repository
        .get_by_join_code(join_code.as_ref())
        .await?
    else {
        return Err(ServerFnError::ServerError("Group not found".to_string()));
    };
    if state.group_repository.is_member(group.id, user.id).await? {
        return Err(ServerFnError::ServerError(
            "Already a member, username: ".to_string() + &user.username,
        ));
    }
    state.group_repository.add_member(group.id, user.id).await?;
    Ok(())
}

#[server]
pub async fn get_groups() -> Result<Vec<Group>, ServerFnError> {
    use crate::AppState;
    let state = use_context::<AppState>().expect("AppState not found");
    use crate::auth::get_user;
    let user = get_user().await?;
    let Some(user) = user else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    };
    let groups = state
        .group_repository
        .list_user_groups_with_last_message(user.id)
        .await?;
    Ok(groups
        .into_iter()
        .map(|v| Group {
            id: v.group.id.to_string(),
            name: v.group.name,
            avatar_url: v
                .group
                .avatar
                .unwrap_or("https://api.dicebear.com/9.x/glass/svg".to_string()),
            join_code: v.group.join_code.clone(),
            last_message: v
                .last_message
                .map(|m| m.content)
                .unwrap_or("No messages yet".to_string()),
        })
        .collect())
}

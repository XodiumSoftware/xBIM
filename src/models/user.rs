/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::routes::github::GitHubUser;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub github_id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Option<UserPreferences>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct UserPreferences {
    pub theme: Option<String>,
}

impl From<GitHubUser> for User {
    fn from(github_user: GitHubUser) -> Self {
        Self {
            id: None,
            github_id: github_user.id,
            login: github_user.login,
            name: github_user.name,
            email: github_user.email,
            avatar_url: github_user.avatar_url,
            preferences: Some(UserPreferences {
                theme: Some("light".to_string()),
            }),
        }
    }
}

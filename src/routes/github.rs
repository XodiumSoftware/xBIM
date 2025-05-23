/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::database::Database;
use crate::models::user::User;
use reqwest::Client as HttpClient;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket_oauth2::{OAuth2, TokenResponse};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[get("/auth/github/login")]
pub fn github_login(oauth2: OAuth2<GitHubUser>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(cookies, &["user:email", "read:user"])
        .unwrap()
}

#[get("/auth/github/callback")]
pub async fn github_callback(
    token: TokenResponse<GitHubUser>,
    cookies: &CookieJar<'_>,
    db: &State<Database>,
) -> Result<Redirect, Flash<Redirect>> {
    // Get GitHub user data
    let github_user: GitHubUser = HttpClient::new()
        .get("https://api.github.com/user")
        .header("User-Agent", "xBIM-App")
        .bearer_auth(token.access_token())
        .send()
        .await
        .map_err(|_| Flash::error(Redirect::to("/"), "Failed to get GitHub user data"))?
        .json()
        .await
        .map_err(|_| Flash::error(Redirect::to("/"), "Failed to parse GitHub user data"))?;

    let github_id = github_user.id;

    // Create or update user record
    let user = User::from(github_user);
    let user_clone = user.clone();
    let saved_user = match db.create("users", user).await {
        Ok(user) => user,
        Err(_) => db
            .update("users", &format!("github_id:{}", github_id), user_clone)
            .await
            .map_err(|_| Flash::error(Redirect::to("/"), "Failed to save user data"))?,
    };

    // Store just the session ID in cookie
    let user_id = saved_user.id.unwrap().to_string();
    cookies.add_private(
        Cookie::build(("user_session", user_id))
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::from_secs(86400).try_into().unwrap())
            .build(),
    );

    Ok(Redirect::to("/"))
}

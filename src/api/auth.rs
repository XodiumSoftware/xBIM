/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use reqwest::Error;
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

pub const COOKIE_TOKEN: &str = "token";

/// Represents the GitHub OAuth2 provider.
pub struct GitHub;

/// Initiates the GitHub OAuth2 login process.
///
/// This function generates a redirect URL for GitHub authentication
/// with the required scope ("user:read"). It stores necessary state information
/// in cookies before redirecting the user to GitHub's login page.
///
/// # Arguments
///
/// * `oauth2` - The GitHub OAuth2 provider wrapper for handling OAuth flow.
/// * `cookies` - The cookie jar used to set or read cookies.
///
/// # Returns
///
/// * `Ok(Redirect)` if the redirect URL is successfully generated.
/// * `Err(String)` if there is an error generating the redirect URL.
#[get("/login")]
pub fn login(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Result<Redirect, String> {
    oauth2
        .get_redirect(cookies, &["user:read"])
        .map_err(|e| e.to_string())
}

/// Handles the callback from the GitHub OAuth2 login process.
///
/// This asynchronous endpoint retrieves the OAuth2 token provided by GitHub,
/// stores the access token in a private cookie, and then redirects the user
/// to the home page.
///
/// # Arguments
///
/// * `token` - The OAuth2 token response containing the access token.
/// * `cookies` - The cookie jar used for storing the authentication token.
///
/// # Returns
///
/// * `Ok(Redirect)` directing the user to the dashboard page after successful authentication.
/// * `Err(Debug<Error>)` if an error occurs during token retrieval or cookie manipulation.
#[get("/auth")]
pub async fn callback(
    token: TokenResponse<GitHub>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    cookies.add_private(
        Cookie::build((COOKIE_TOKEN, token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(true),
    );
    Ok(Redirect::to("/dashboard"))
}

/// Logs the user out by clearing authentication-related cookies.
///
/// This endpoint removes the private authentication token cookie, effectively logging out
/// the user, and then redirects them to the home page.
///
/// # Arguments
///
/// * `cookies` - The cookie jar from which the authentication cookie will be removed.
///
/// # Returns
///
/// * `Ok(Redirect)` directing the user to the home page after logging out.
/// * `Err(Debug<Error>)` if an error occurs during cookie manipulation.
#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Result<Redirect, Debug<Error>> {
    cookies.remove_private(Cookie::build(COOKIE_TOKEN));
    Ok(Redirect::to("/"))
}

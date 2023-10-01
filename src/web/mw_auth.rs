use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_requier<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.to_string());
    auth_token.ok_or(Error::AuthFailNoAuthToken)?;
    Ok(next.run(req).await)
}

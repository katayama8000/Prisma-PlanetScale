use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_requier<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.to_string());
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)?;
    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let parts: Vec<&str> = token.split('=').collect();
    println!("{}", parts[1]);
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &parts[1]
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

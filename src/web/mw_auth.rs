use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::response::Response;
use axum::middleware::Next;
use axum::http::Request;
use tower_cookies::Cookies;
use axum::body::Body;

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
    ) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Todo: Real auth-token parsing
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)

}
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_cookies::{Cookies, cookie};


// Check session cookie is valid for a user
pub async fn auth_user(cookies:Cookies,req:Request, next:Next) -> Result<Response, Response> {

    // No cookie, redirect to login page
    let Some(cookie) = cookies.get("session") else {
        return Err(Redirect::to("/login").into_response());
    };

    // TODO: Validate session cookie

    Ok(next.run(req).await)

}
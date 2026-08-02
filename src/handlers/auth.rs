use serde::Deserialize;
use axum::{
    response::{Html, Redirect},
    extract::Form,
};
use askama::Template;
use tower_cookies::{Cookie, Cookies, cookie};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate {
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

// display login page
pub async fn login_page() -> Html<String>{
    //askama template
    let template = LoginTemplate {
        error: None,
    };

    Html(template.render().unwrap())
}

// check credentials and create session cookie if valid
pub async fn login(cookies: Cookies,
    Form(form): Form<LoginForm>,) -> Redirect {
    println!("Login Detected");
    println!("Username {}", form.username);

    // check login valid
    // if not stay on /login and display error


    // PLACEHOLDER: grant session cookie
    let session_id = Uuid::new_v4();
    let session = Cookie::build(("session", session_id.to_string()))
    .path("/")
    .http_only(true)
    .secure(false)    // NOTE: only secure as false while local
    .same_site(cookie::SameSite::Lax)
    .max_age(cookie::time::Duration::days(7))
    .build();
    cookies.add(session);

    Redirect::to("/")
}

// remove session cookie
pub async fn logout(cookies: Cookies) -> Redirect {
    
    //TODO:: remove from db also
    // remove cookie from browser
    cookies.remove(Cookie::build(("session", ""))
        .path("/")
        .build(),
    );

    Redirect::to("/login")
}
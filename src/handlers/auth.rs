use serde::Deserialize;
use axum::{
    response::{Html, Redirect},
    extract::Form,
};
use askama::Template;

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
pub async fn login(
    Form(form): Form<LoginForm>,) -> Redirect {
    println!("Login Detected");
    println!("Username {}", form.username);


    Redirect::to("/")
}

// remove session cookie
pub async fn logout() {

}
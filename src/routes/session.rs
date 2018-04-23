use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use models::user::User;
use rocket::response::status::NotFound;
use rocket::http::Cookies;
use db_conn::DbConn;
use rocket::http::Cookie;
use models::user::AUTH_COOKIE;

#[get("/login")]
fn new() -> Template {
    Template::render("session/login", HashMap::<String, String>::new())
}

#[derive(FromForm)]
struct LoginForm {
    email_or_name: String,
    password: String
}

#[post("/login", data = "<data>")]
fn create(conn: DbConn, data: Form<LoginForm>, mut cookies: Cookies) -> Result<Redirect, NotFound<String>> {
    let form = data.get();
    let user = match User::find_by_email(&*conn, form.email_or_name.to_string()) {
        Some(usr) => Ok(usr),
        None => match User::find_by_name(&*conn, form.email_or_name.to_string()) {
            Some(usr) => Ok(usr),
            None => Err("Invalid username or password")
        }
    };

    match user {
        Ok(usr) => {
            if usr.auth(form.password.to_string()) {
                cookies.add_private(Cookie::new(AUTH_COOKIE, usr.id.to_string()));
                Ok(Redirect::to("/"))
            } else {
                Err(NotFound(String::from("Invalid username or password")))
            }
        },
        Err(e) => Err(NotFound(String::from(e)))
    }
}

#[get("/logout")]
fn delete(mut cookies: Cookies) -> Redirect {
    let cookie = cookies.get_private(AUTH_COOKIE).unwrap();
    cookies.remove_private(cookie);
    Redirect::to("/")
}

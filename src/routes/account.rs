use crate::model::{Session, User};
use crate::render;
use crate::ructe::Ructe;
use crate::Database;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;

pub const SESSION_COOKIE: &str = "nuit-info:session";

#[get("/login")]
pub fn login() -> Ructe {
    render!(account::login())
}

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<data>")]
pub async fn login_post(
    db: Database,
    cookies: &CookieJar<'_>,
    data: Form<LoginForm>,
) -> Result<Redirect, String> {
    let data = data.into_inner();
    let email = data.email;

    let user = db
        .run(move |c| User::by_email(c, &email))
        .await
        .ok_or_else(|| String::from("Compte inconnu"))?;

    if bcrypt::verify(data.password, &user.password).ok() == Some(true) {
        let session = db
            .run(move |c| Session::insert(c, user.id))
            .await
            .ok_or_else(|| String::from("Cannot create session"))?;

        cookies.add_private(Cookie::new(SESSION_COOKIE, session.to_string()));

        Ok(Redirect::to("/admin"))
    } else {
        Err(String::from("Mot de passe invalide"))
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named(SESSION_COOKIE));
    Redirect::to("/")
}

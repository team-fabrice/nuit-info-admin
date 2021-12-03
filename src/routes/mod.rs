use crate::model::{Session, User};
use crate::routes::account::SESSION_COOKIE;
use crate::Database;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub mod account;
pub mod admin;

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = match request.guard::<Database>().await {
            Outcome::Forward(f) => return Outcome::Forward(f),
            Outcome::Failure(_) => {
                return Outcome::Failure((Status::InternalServerError, String::from("no database")))
            }
            Outcome::Success(db) => db,
        };

        let cookies = request.cookies();

        let session = cookies
            .get_private(SESSION_COOKIE)
            .map(|c| c.value().to_owned())
            .unwrap_or_default();

        match db.run(move |c| Session::query(c, &session)).await {
            None => Outcome::Failure((Status::Unauthorized, String::from("not logged in"))),
            Some(user) => Outcome::Success(user),
        }
    }
}

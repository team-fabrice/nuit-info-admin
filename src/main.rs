#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod schema;

use diesel::expression::count::count_star;
use crate::diesel::{QueryDsl, RunQueryDsl};

#[database("nuit-info")]
struct Database(rocket_sync_db_pools::diesel::PgConnection);

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![index])
}

#[get("/")]
async fn index(db: Database) -> String {
    db.run(|c| {
        use crate::schema::users::dsl::users;

        users.select(count_star()).first::<i64>(c)
    }).await.unwrap().to_string()
}


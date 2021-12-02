#[macro_use]
extern crate diesel;
#[cfg(not(debug_assertions))]
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod schema;

use crate::diesel::{QueryDsl, RunQueryDsl};
use diesel::expression::count::count_star;

#[cfg(not(debug_assertions))]
embed_migrations!();

#[database("nuit-info")]
struct Database(rocket_sync_db_pools::diesel::PgConnection);

#[rocket::launch]
fn launch() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .attach(Database::fairing());

    #[cfg(debug_assertions)]
    {
        rocket
    }

    #[cfg(not(debug_assertions))]
    {
        use rocket::fairing::AdHoc;
        rocket.attach(AdHoc::on_liftoff("migration runner", |rocket| {
            Box::pin(async move {
                let conn = Database::get_one(rocket)
                    .await
                    .expect("no database available for running migrations");

                conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout()))
                    .await
                    .unwrap();
            })
        }))
    }
}

#[get("/")]
async fn index(db: Database) -> String {
    db.run(|c| {
        use crate::schema::users::dsl::users;

        users.select(count_star()).first::<i64>(c)
    })
    .await
    .unwrap()
    .to_string()
}

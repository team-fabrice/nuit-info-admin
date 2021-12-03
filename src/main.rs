#[macro_use]
extern crate diesel;
#[cfg(not(debug_assertions))]
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod model;
mod routes;
mod ructe;
mod schema;

#[cfg(not(debug_assertions))]
embed_migrations!();

pub type Connection = rocket_sync_db_pools::diesel::PgConnection;
#[database("nuit-info")]
pub struct Database(Connection);

#[rocket::launch]
fn launch() -> _ {
    let rocket = rocket::build()
        .mount(
            "/admin",
            routes![
                routes::account::login,
                routes::account::login_post,
                routes::account::logout,
                routes::admin::articles,
                routes::admin::articles_delete,
                routes::admin::articles_new,
                routes::admin::articles_edit,
                routes::admin::contributions,
                routes::admin::dashboard,
            ],
        )
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

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

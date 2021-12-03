use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
    pub is_admin: bool,
}

impl User {
    pub fn by_id(c: &crate::Connection, user_id: Uuid) -> Option<User> {
        use crate::schema::users::dsl as u_dsl;

        u_dsl::users
            .filter(u_dsl::id.eq(user_id))
            .first::<User>(c)
            .ok()
    }

    pub fn by_email(c: &crate::Connection, email: &str) -> Option<User> {
        use crate::schema::users::dsl as u_dsl;

        u_dsl::users
            .filter(u_dsl::email.eq(email))
            .first::<User>(c)
            .ok()
    }
}

#[derive(Debug, Queryable)]
pub struct Session {
    session: Uuid,
    account: Uuid,
    expires: NaiveDateTime,
}

impl Session {
    pub fn insert(c: &crate::Connection, account: Uuid) -> Option<Uuid> {
        use crate::schema::sessions;
        use crate::schema::sessions::dsl as s_dsl;

        #[derive(Insertable)]
        #[table_name = "sessions"]
        struct NewSession {
            account: Uuid,
        }

        diesel::insert_into(s_dsl::sessions)
            .values(NewSession { account })
            .get_result::<Self>(c)
            .ok()
            .map(|s| s.session)
    }

    pub fn query(c: &crate::Connection, session: &str) -> Option<User> {
        use crate::schema::sessions::dsl as s_dsl;

        let session = session.parse::<Uuid>().ok()?;

        let now = Utc::now().naive_utc();

        let session = s_dsl::sessions
            .filter(s_dsl::session.eq(session).and(s_dsl::expires.gt(now)))
            .first::<Self>(c)
            .ok()?;

        User::by_id(c, session.account)
    }
}

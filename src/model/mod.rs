use chrono::{NaiveDate, NaiveDateTime, Utc};
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

#[derive(Debug, Queryable)]
pub struct ArticleRev {
    pub revision_id: Uuid,
    pub article_id: Uuid,
    pub title: String,
    pub contents: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub modification_author: Option<Uuid>,

    meta_class: Option<String>,
    meta_person_first_name: Option<String>,
    meta_person_last_name: Option<String>,
    meta_person_birth: Option<NaiveDate>,
    meta_person_death: Option<NaiveDate>,
    meta_event_date: Option<NaiveDate>,
    meta_location: Option<String>,
}

impl ArticleRev {
    pub fn list(c: &crate::Connection) -> Option<Vec<Self>> {
        use crate::schema::article_rev::dsl as a_dsl;

        a_dsl::article_rev
            .filter(a_dsl::modification_author.is_null())
            .load::<Self>(c)
            .ok()
    }

    pub fn list_contributions(c: &crate::Connection, for_user: Option<Uuid>) -> Option<Vec<Self>> {
        use crate::schema::article_rev::dsl as a_dsl;

        if let Some(for_user) = for_user {
            a_dsl::article_rev
                .filter(a_dsl::modification_author.eq(for_user))
                .load::<Self>(c)
        } else {
            a_dsl::article_rev
                .filter(a_dsl::modification_author.is_not_null())
                .load::<Self>(c)
        }
        .ok()
    }

    pub fn edit(c: &crate::Connection, id: Uuid, title: String, contents: String) -> Option<()> {
        use crate::schema::article_rev::dsl as a_dsl;

        diesel::update(a_dsl::article_rev)
            .filter(a_dsl::article_id.eq(id))
            .set((a_dsl::title.eq(title), a_dsl::contents.eq(contents)))
            .execute(c)
            .ok()
            .map(|_| ())
    }

    pub fn insert(
        c: &crate::Connection,
        title: String,
        contents: String,
        article: Option<Uuid>,
        author: Option<Uuid>,
    ) -> Option<()> {
        use crate::schema::article_rev;
        use crate::schema::article_rev::dsl as a_dsl;

        let article = article.unwrap_or_else(|| Uuid::new_v4());

        #[derive(Insertable)]
        #[table_name = "article_rev"]
        struct NewArticle {
            article_id: Uuid,
            title: String,
            contents: String,
            created_at: NaiveDate,
            updated_at: NaiveDate,
            modification_author: Option<Uuid>,
        }

        diesel::insert_into(a_dsl::article_rev)
            .values(NewArticle {
                title,
                article_id: article,
                contents,
                created_at: Utc::now().date().naive_utc(),
                updated_at: Utc::now().date().naive_utc(),
                modification_author: author,
            })
            .execute(c)
            .ok()
            .map(|_| ())
    }

    pub fn delete(c: &crate::Connection, id: Uuid) -> Option<()> {
        use crate::schema::article_rev::dsl as a_dsl;

        diesel::delete(a_dsl::article_rev)
            .filter(a_dsl::article_id.eq(id))
            .execute(c)
            .ok()
            .map(|_| ())
    }

    pub fn delete_by_rev(c: &crate::Connection, rev_id: Uuid) -> diesel::result::QueryResult<()> {
        use crate::schema::article_rev::dsl as a_dsl;

        diesel::delete(a_dsl::article_rev)
            .filter(a_dsl::revision_id.eq(rev_id))
            .execute(c)
            .map(|_| ())
    }

    pub fn accept(c: &crate::Connection, rev_id: Uuid) -> Option<()> {
        use crate::schema::article_rev::dsl as a_dsl;

        c.transaction(|| {
            let art = a_dsl::article_rev
                .filter(a_dsl::revision_id.eq(rev_id))
                .select(a_dsl::article_id)
                .first::<Uuid>(c)?;
            Self::delete_by_rev(c, art)?;

            diesel::update(a_dsl::article_rev)
                .filter(a_dsl::revision_id.eq(rev_id))
                .set((a_dsl::modification_author.eq(Option::<Uuid>::None),))
                .execute(c)
        })
        .ok()
        .map(|_| ())
    }

    pub fn class(&self) -> Option<&'static str> {
        match self.meta_class.as_deref() {
            Some("person") => Some("Personne"),
            Some("event") => Some("Événement"),
            _ => None,
        }
    }
}

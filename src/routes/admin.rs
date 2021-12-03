use crate::model::{ArticleRev, User};
use crate::ructe::Ructe;
use crate::{render, Database};
use rocket::form::Form;
use rocket::response::Redirect;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Section {
    Dashboard,
    Articles,
    Media,
    Contributions,
    Users,
}

impl Section {
    pub fn short_name(self) -> &'static str {
        match self {
            Self::Dashboard => "Tableau de bord",
            Self::Articles => "Articles",
            Self::Media => "Fichiers",
            Self::Contributions => "Contributions",
            Self::Users => "Utilisateurs",
        }
    }

    pub fn long_name(self) -> &'static str {
        match self {
            Self::Media => "Fichiers mis en ligne",
            Self::Contributions => "Contributions en attente",
            Self::Users => "Gestion des utilisateur·ice·s",
            o => o.short_name(),
        }
    }

    pub fn selected(self, o: Self) -> &'static str {
        if self == o {
            " selected"
        } else {
            ""
        }
    }
}

#[get("/")]
pub fn dashboard(user: User) -> Ructe {
    render!(sidebar::dashboard(&user))
}

#[get("/articles")]
pub async fn articles(db: Database, user: User) -> Ructe {
    let articles = db.run(|c| ArticleRev::list(c)).await.unwrap();
    render!(sidebar::articles(&user, &articles))
}

#[derive(FromForm)]
pub struct DeleteForm {
    delete: Option<String>,
}

#[post("/admin/articles/<id>", data = "<data>")]
pub async fn articles_delete(
    db: Database,
    data: Form<DeleteForm>,
    user: User,
    id: Uuid,
) -> Redirect {
    if data.delete.is_some() && user.is_admin {
        db.run(move |c| ArticleRev::delete(c, id)).await.unwrap();
    }

    Redirect::to("/admin/articles")
}

#[derive(FromForm)]
pub struct EditForm {
    title: String,
    contents: String,
}

#[post("/admin/articles/new", data = "<data>")]
pub async fn articles_new(db: Database, user: User, data: Form<EditForm>) -> Redirect {
    let data = data.into_inner();
    let is_admin = user.is_admin;

    db.run(move |c| {
        if is_admin {
            ArticleRev::insert(c, data.title, data.contents, None, None)
        } else {
            ArticleRev::insert(c, data.title, data.contents, None, Some(user.id))
        }
    })
    .await;

    if is_admin {
        Redirect::to("/admin/articles")
    } else {
        Redirect::to("/admin/contributions")
    }
}

#[post("/admin/articles/<id>/edit", data = "<data>")]
pub async fn articles_edit(db: Database, id: Uuid, user: User, data: Form<EditForm>) -> Redirect {
    let data = data.into_inner();
    let is_admin = user.is_admin;

    db.run(move |c| {
        if is_admin {
            ArticleRev::edit(c, id, data.title, data.contents)
        } else {
            ArticleRev::insert(c, data.title, data.contents, Some(id), Some(user.id)).map(|_| ())
        }
    })
    .await;

    if is_admin {
        Redirect::to("/admin/articles")
    } else {
        Redirect::to("/admin/contributions")
    }
}

#[get("/contributions")]
pub async fn contributions(db: Database, user: User) -> Ructe {
    let filter = match user.is_admin {
        true => None,
        false => Some(user.id),
    };

    let contributions = db
        .run(move |c| ArticleRev::list_contributions(c, filter))
        .await
        .unwrap();

    render!(sidebar::contributions(&user, &contributions))
}

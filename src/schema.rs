table! {
    article_rev (revision_id) {
        revision_id -> Uuid,
        article_id -> Uuid,
        title -> Varchar,
        contents -> Varchar,
        created_at -> Date,
        updated_at -> Date,
        modification_author -> Nullable<Uuid>,
        meta_class -> Nullable<Varchar>,
        meta_person_first_name -> Nullable<Varchar>,
        meta_person_last_name -> Nullable<Varchar>,
        meta_person_birth -> Nullable<Date>,
        meta_person_death -> Nullable<Date>,
        meta_event_date -> Nullable<Date>,
        meta_location -> Nullable<Varchar>,
    }
}

table! {
    media (hash) {
        hash -> Varchar,
        mime -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Nullable<Varchar>,
        password -> Varchar,
        is_admin -> Bool,
    }
}

joinable!(article_rev -> users (modification_author));

allow_tables_to_appear_in_same_query!(
    article_rev,
    media,
    users,
);

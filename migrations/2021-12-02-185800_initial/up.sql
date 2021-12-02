create table users (
    id uuid not null default uuid_generate_v4 (),
    email varchar(255) not null,
    name varchar (32),
    -- BCrypted password
    password varchar(60) not null,
    -- User can accept contribution from others and manage accounts
    is_admin bool not null default false,

    primary key (id)
);

create table media (
    hash varchar(32) not null,
    mime varchar(64) not null,
    title varchar(255) not null,
    description varchar,

    primary key (hash)
);

create table article_rev (
    -- Globally unique ID for this revision
    revision_id uuid not null default uuid_generate_v4 (),
    -- Non-unique ID shared by all revisions referring to the same article
    article_id uuid not null,

    title varchar(255) not null,
    contents varchar not null,
    created_at date not null,
    updated_at date not null,
    modification_author uuid references users (id) on delete cascade,

    -- Type of article, if any (sauveteur, bateau, événement)
    meta_class varchar(32),

    meta_person_first_name varchar,
    meta_person_last_name varchar,
    meta_person_birth date,
    meta_person_death date,
    meta_event_date date,
    meta_location varchar,

    primary key (revision_id)
);

create unique index on article_rev (article_id) where modification_author is null;

select diesel_manage_updated_at ('article_rev');

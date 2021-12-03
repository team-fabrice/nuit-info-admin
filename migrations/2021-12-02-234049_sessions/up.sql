create table sessions (
    session uuid not null default uuid_generate_v4 (),
    account uuid not null references users (id) on delete cascade,
    expires timestamp with time zone not null default now() + interval '30 days',

    unique (session, account),

    primary key (session)
);

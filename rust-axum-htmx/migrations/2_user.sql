create table "users"(
    github_id int not null,
    github_login text,
    access_token text,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT trigger_updated_at('"users"');

create table if not exists Blogs (
    id serial primary key,
    title varchar(255) not null,
    subtitle text,
    author varchar(255),
    slug varchar(255) not null,
    body text not null,
    published boolean not null default 'f',
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
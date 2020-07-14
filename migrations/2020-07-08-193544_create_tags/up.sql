create table if not exists Tags (
    id serial primary key,
    name varchar(255) unique not null
)
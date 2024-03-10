-- Your SQL goes here
create table todo (
    id serial primary key,
    text text default '',
    status varchar not null
)
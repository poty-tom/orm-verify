-- Your SQL goes here
create table todos (
    id serial primary key,
    title varchar(255) not null,
    complete boolean not null default false,
    created_at timestamp not null default current_timestamp
);
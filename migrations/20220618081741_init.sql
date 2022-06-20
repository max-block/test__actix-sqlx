create table data1 (
    id serial primary key,
    name text not null,
    value integer not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
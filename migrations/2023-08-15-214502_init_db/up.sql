-- Your SQL goes here
create type gender as enum('male', 'female');

create table cable_types (
	id serial primary key,
	name varchar not null,
	cable_gender gender not null,
	image bytea not null
);

create table cables (
	id serial primary key,
	end_a serial references cable_types(id) on delete cascade,
	end_b serial references cable_types(id) on delete cascade,
	cable_length real not null
);
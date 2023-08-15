-- Your SQL goes here
create type gender as enum('Male', 'Female');

create table cable_types (
	id serial primary key,
	name varchar not null,
	cable_gender gender not null,
	image bytea
);

create table cables (
	id serial primary key,
	end_a serial references cable_types(id),
	end_b serial references cable_types(id),
	cable_length real not null
);
-- Your SQL goes here

CREATE TABLE users (
	user_id varchar NOT NULL,
	first_name varchar NOT NULL,
	last_name varchar NOT NULL,
	e_mail varchar NOT NULL,
	"password" varchar NOT NULL,
	created_at int8 NOT NULL,
	CONSTRAINT users_pkey PRIMARY KEY (user_id)
);
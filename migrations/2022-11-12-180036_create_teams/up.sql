-- Your SQL goes here

CREATE TABLE teams (
	team_id varchar NOT NULL,
	team_name varchar NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT teams_pkey PRIMARY KEY (team_id) 
);

CREATE TABLE teammember (
	team_member_id varchar NOT NULL,
	user_id varchar NOT NULL,
	team_id varchar NOT NULL,
	role_id int4 NOT NULL,
	CONSTRAINT teammember_pkey PRIMARY KEY (team_member_id),
	CONSTRAINT fk_teammember_team FOREIGN KEY (team_id) REFERENCES teams(team_id),
	CONSTRAINT fk_teammember_role FOREIGN KEY (role_id) REFERENCES systemrole(role_id),
	CONSTRAINT fk_teammember_users FOREIGN KEY (user_id) REFERENCES users(user_id)
);

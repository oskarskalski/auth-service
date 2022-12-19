-- Your SQL goes here

CREATE TABLE teamroles (
	id serial4 NOT NULL,
	role_name varchar NOT NULL,
	CONSTRAINT roles_pkey PRIMARY KEY (id)
);

INSERT INTO teamroles (id, role_name)
VALUES
    (1, 'U_READ'),
    (2, 'U_CREATE'),
    (3, 'U_EDIT'),
    (4, 'U_DELETE'),
    (5, 'U_SOFT_DELETE'),
    (6, 'T_EDIT'),
    (7, 'T_ADD_USER'),
    (8, 'T_DELETE'),
    (9, 'T_SOFT_DELETE'),
    (10, 'T_CREATE');

CREATE TABLE teams (
	team_id varchar NOT NULL,
	team_name varchar NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT teams_pkey PRIMARY KEY (team_id) 
);

CREATE TABLE teamrole (
	team_role_id varchar NOT NULL,
	role_name varchar NOT NULL,
	team_id varchar NOT NULL,
	CONSTRAINT teamrole_pkey PRIMARY KEY (team_role_id),
	CONSTRAINT fk_teamrole_teams FOREIGN KEY (team_id) REFERENCES teams(team_id)
);


CREATE TABLE teamrolemap (
	id serial4 NOT NULL,
	role_id int4 NOT NULL,
	team_role_id varchar NOT NULL,
	CONSTRAINT teamrolemap_pkey PRIMARY KEY (id),
	CONSTRAINT fk_teamrolemap_teamrole FOREIGN KEY (team_role_id) REFERENCES teamrole(team_role_id),
	CONSTRAINT fk_teamrolemap_teamroles FOREIGN KEY (role_id) REFERENCES teamroles(id)
);

CREATE TABLE teammember (
	team_member_id varchar NOT NULL,
	user_id varchar NOT NULL,
	team_id varchar NOT NULL,
	role_id varchar NOT NULL,
	CONSTRAINT teammember_pkey PRIMARY KEY (team_member_id),
	CONSTRAINT fk_teammember_team FOREIGN KEY (team_id) REFERENCES teams(team_id),
	CONSTRAINT fk_teammember_teamrole FOREIGN KEY (role_id) REFERENCES teamrole(team_role_id),
	CONSTRAINT fk_teammember_users FOREIGN KEY (user_id) REFERENCES users(user_id)
);

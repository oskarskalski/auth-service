-- Your SQL goes here

CREATE TABLE grouproles (
	id serial4 NOT NULL,
	role_name varchar NOT NULL,
	CONSTRAINT grouproles_pkey PRIMARY KEY (id)
);

CREATE TABLE groups (
	group_id varchar PRIMARY KEY NOT NULL,
	team_id varchar NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT fk_subgroup_team FOREIGN KEY (team_id) REFERENCES teams(team_id)
);

CREATE TABLE grouprole (
	group_role_id varchar PRIMARY KEY NOT NULL,
	group_id varchar NOT NULL,
	role_name varchar NOT NULL,
	CONSTRAINT fk_grouprole_groups FOREIGN KEY (group_id) REFERENCES groups(group_id)
);

CREATE TABLE grouprolemap (
	id serial4 NOT NULL,
	group_role_id varchar NOT NULL,
	group_roles_id int4 NOT NULL,
	CONSTRAINT grouprolemap_pkey PRIMARY KEY (id),
	CONSTRAINT fk_grouprolemap_grouproles FOREIGN KEY (group_roles_id) REFERENCES grouproles(id),
	CONSTRAINT fk_grouprolemap_grouprole FOREIGN KEY (group_role_id) REFERENCES grouprole(group_role_id)
);

CREATE TABLE groupuser (
	user_id varchar PRIMARY KEY NOT NULL,
	group_id varchar NOT NULL,
	group_role_id varchar NOT NULL,
	CONSTRAINT fk_groupuser_group FOREIGN KEY (group_id) REFERENCES groups(group_id),
	CONSTRAINT fk_groupuser_user FOREIGN KEY (user_id) REFERENCES users(user_id),
	CONSTRAINT fk_groupuser_grouprole FOREIGN KEY (group_role_id) REFERENCES grouprole(group_role_id)
);



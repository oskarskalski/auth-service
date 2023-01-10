-- Your SQL goes here

CREATE TABLE groups (
	group_id varchar PRIMARY KEY NOT NULL,
	team_id varchar NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT fk_subgroup_team FOREIGN KEY (team_id) REFERENCES teams(team_id)
);

CREATE TABLE groupuser (
	user_id varchar PRIMARY KEY NOT NULL,
	group_id varchar NOT NULL,
	role_id int4 NOT NULL,
	CONSTRAINT fk_groupuser_group FOREIGN KEY (group_id) REFERENCES groups(group_id),
	CONSTRAINT fk_groupuser_user FOREIGN KEY (user_id) REFERENCES users(user_id),
	CONSTRAINT fk_groupuser_role FOREIGN KEY (role_id) REFERENCES systemrole(role_id)
);



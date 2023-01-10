-- Your SQL goes here


CREATE TABLE roles (
	id serial4 NOT NULL,
	role_name varchar NOT NULL,
    role_description varchar, 
	CONSTRAINT roles_pkey PRIMARY KEY (id)
);

INSERT INTO roles (id, role_name)
VALUES
    (1, 'T_ADD_USER'),
    (2, 'T_READ'),
    (3, 'T_EDIT'),
    (4, 'T_DELETE'),
    (5, 'T_SOFT_DELETE'),
    (6, 'T_SOFT_DELETE_USER'),
    (7, 'T_GROUP_CREATOR'),

    (8, 'G_DELETE'),
    (9, 'G_SOFT_DELETE'),
    (10, 'G_ADD_USER'),
    (11, 'G_DELETE_USER'),
    (12, 'G_EDIT'),
    (13, 'G_READ'),
    (14, 'G_SOFT_DELETE_USER');

CREATE TABLE systemrole (
	role_id serial4 NOT NULL,
	role_name varchar NOT NULL,
    role_type varchar NOT NULL,
    role_description varchar,
	CONSTRAINT teamrole_pkey PRIMARY KEY (role_id)
);

INSERT INTO systemrole (role_id, role_name, role_type)
VALUES
    (1, 'TEAM_OWNER', 'TEAM'),
    (2, 'TEAM_MANAGER', 'TEAM'),
    (3, 'TEAM_MEMBER', 'TEAM'),
    (4, 'TEAM_GUEST', 'TEAM'),
    (5, 'G_OWNER', 'GROUP'),
    (6, 'G_MEMBER', 'GROUP'),
    (7, 'G_GUEST', 'GROUP'),
    (8, 'G_CREATOR', 'GROUP'); 

CREATE TABLE rolesmap (
    id serial4 NOT NULL, 
    role_id int4 NOT NULL,
    roles_id int4 NOT NULL,
	CONSTRAINT rolesmap_pkey PRIMARY KEY (id),
	CONSTRAINT fk_rolesmap_roles FOREIGN KEY (roles_id) REFERENCES roles(id),
	CONSTRAINT fk_rolesmap_role FOREIGN KEY (role_id) REFERENCES systemrole(role_id)
);

-- TEAM OWNER role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (1,1,1),
        (2,1,2),
        (3,1,3),
        (4,1,4),
        (5,1,5),
        (6,1,6),
        (7,1,7);

-- TEAM MANAGER role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (8,2,1),
        (9,2,2),
        (10,2,3),
        (11,2,6),
        (12,2,7);

-- TEAM MEMBER role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (13,3,1),
        (14,3,2),
        (30,3,7);

-- TEAM GUEST role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (15,4,2);

-- GROUP OWNER role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (16,5,8),
        (17,5,9),
        (18,5,10),
        (19,5,11),
        (20,5,12),
        (21,5,13),
        (22,5,14);

-- GROUP MEMBER role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (23,6,12),
        (24,6,13);

-- GROUP GUEST role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (25,7,13);

-- GROUP CREATOR role
INSERT INTO rolesmap (id, role_id, roles_id)
VALUES
        (26,8,9),
        (27,8,10),
        (28,8,12),
        (29,8,13),
        (31,8,14);
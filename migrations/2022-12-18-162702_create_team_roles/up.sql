-- Your SQL goes here


CREATE TABLE teamroles2 (
	id serial4 NOT NULL,
	role_name varchar NOT NULL,
	CONSTRAINT roles2_pkey PRIMARY KEY (id)
);

INSERT INTO teamroles2 (id, role_name)
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

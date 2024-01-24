-- UPDATE users SET is_active = TRUE WHERE id = 1;

INSERT INTO users (username, hashed_password, nickname, avatar_uri, is_active) VALUES (
    'root',
    '$argon2id$v=19$m=19456,t=2,p=1$dM/kaD0QQq9uDgU0XqmtNA$/+1qV1BsYJ22TH9Yk7UaOM+OUBZ6QU3Ky23fd8GC8pE',
    'root',
    'default',
    TRUE
);

INSERT INTO groups (name) VALUES ('root');

INSERT INTO permissions (name) VALUES ('users.read_all');
INSERT INTO permissions (name) VALUES ('users.update_all');
INSERT INTO permissions (name) VALUES ('users.delete_all');
INSERT INTO permissions (name) VALUES ('users.activate');

INSERT INTO permissions (name) VALUES ('datasets.create');
INSERT INTO permissions (name) VALUES ('datasets.read');
INSERT INTO permissions (name) VALUES ('datasets.update');
INSERT INTO permissions (name) VALUES ('datasets.delete');

INSERT INTO permissions (name) VALUES ('datasets.items.create');
INSERT INTO permissions (name) VALUES ('datasets.items.read');
INSERT INTO permissions (name) VALUES ('datasets.items.update');
INSERT INTO permissions (name) VALUES ('datasets.items.delete');

INSERT INTO permissions (name) VALUES ('datasets.shards.create');
INSERT INTO permissions (name) VALUES ('datasets.shards.read');
INSERT INTO permissions (name) VALUES ('datasets.shards.update');
INSERT INTO permissions (name) VALUES ('datasets.shards.delete');

INSERT INTO permissions (name) VALUES ('groups.create');
INSERT INTO permissions (name) VALUES ('groups.read');
INSERT INTO permissions (name) VALUES ('groups.delete');

INSERT INTO permissions (name) VALUES ('permissions.create');
INSERT INTO permissions (name) VALUES ('permissions.read');
INSERT INTO permissions (name) VALUES ('permissions.delete');

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 17);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 18);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 19);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 20);

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 21);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 22);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 23);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 24);

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 25);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 26);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 27);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 28);

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 29);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 30);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 31);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 32);

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 33);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 34);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 35);

INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 36);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 37);
INSERT INTO groups_permissions_rel (group_id, permission_id) VALUES (5, 38);

INSERT INTO users_groups_rel (user_id, group_id) VALUES (1, 5);

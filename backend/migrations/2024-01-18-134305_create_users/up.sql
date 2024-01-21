CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL,
    nickname VARCHAR(255) NOT NULL,
    avatar_uri VARCHAR(255) NOT NULL DEFAULT 'default',
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('groups');

CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('permissions');

-- relationship between users and groups
CREATE TABLE users_groups_rel (
    user_id INTEGER NOT NULL REFERENCES users(id),
    group_id INTEGER NOT NULL REFERENCES groups(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(user_id, group_id)
);

-- relationship between groups and permissions
CREATE TABLE groups_permissions_rel (
    group_id INTEGER NOT NULL REFERENCES groups(id),
    permission_id INTEGER NOT NULL REFERENCES permissions(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(group_id, permission_id)
);

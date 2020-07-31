CREATE TABLE user
(
--     "uid"           INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "username"      VARCHAR NOT NULL PRIMARY KEY,
    "first_name"    VARCHAR NOT NULL,
    "last_name"     VARCHAR NOT NULL,
    "password_hash" VARCHAR NOT NULL
);

-- CREATE UNIQUE INDEX idx_user_username
--     ON user (username);

INSERT INTO user (username, first_name, last_name, password_hash)
VALUES ('yvi', 'Yvi', 'Best', 'pwhash'),
       ('daniel', 'Daniel', 'Corn', 'pwhash');

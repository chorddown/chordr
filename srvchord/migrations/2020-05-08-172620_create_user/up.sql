CREATE TABLE user
(
    "id"         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "username"   VARCHAR NOT NULL UNIQUE,
    "first_name" VARCHAR NOT NULL,
    "last_name"  VARCHAR NOT NULL,
    "password"   VARCHAR NOT NULL
);

INSERT INTO user (username, first_name, last_name, password)
VALUES ('yvi', 'Yvi', 'Best', 'pwhash'),
       ('daniel', 'Daniel', 'Corn', 'pwhash');

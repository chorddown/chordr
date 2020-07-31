CREATE TABLE team
(
    "id"    VARCHAR NOT NULL PRIMARY KEY,
    "name"  VARCHAR NOT NULL UNIQUE,
    "users" VARCHAR NOT NULL
);

-- CREATE UNIQUE INDEX idx_team_id
--     ON team (id);

INSERT INTO team (id, name, users)
VALUES ('super-team', 'Allstars', 'yvi,daniel'),
       ('dan-team', 'Only me', 'daniel');

CREATE TABLE setlist
(
    "id"                INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name"              TEXT      NOT NULL,
    "sorting"           INTEGER   NOT NULL,
    "owner"             VARCHAR   NOT NULL,
    "team"              VARCHAR,
    "gig_date"          TIMESTAMP,
    "creation_date"     TIMESTAMP NOT NULL,
    "modification_date" TIMESTAMP NOT NULL
);

CREATE TABLE setlist_entry
(
    "id"            INTEGER PRIMARY KEY AUTOINCREMENT,
    "song_id"       VARCHAR     NOT NULL,
    "file_type"     VARCHAR     NOT NULL,
    "title"         TEXT,
    "settings"      TEXT,
    "setlist_db_id" INTEGER KEY NOT NULL,
    CONSTRAINT fk_setlist
        FOREIGN KEY (setlist_db_id)
            REFERENCES setlist (id)
            ON DELETE CASCADE
);

INSERT INTO setlist (id, name, sorting, owner, team, gig_date, creation_date, modification_date)
VALUES (1, 'Yvi`s setlist', 100, 'yvi', null, '2020-07-05 00:00:00', '2020-06-25 00:00:00', '2020-06-28 13:37:40'),
       (2, 'Daniel`s setlist', 200, 'daniel', null, '2020-07-05 00:00:00', '2020-06-25 00:00:00',
        '2020-06-28 13:37:40');

INSERT INTO setlist_entry (song_id, title, setlist_db_id, file_type)
VALUES ('bubbles', 'song 1', 1, 'chorddown'),
       ('great', 'song 2', 2, 'chorddown');

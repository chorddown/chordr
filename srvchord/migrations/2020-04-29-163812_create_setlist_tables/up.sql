CREATE TABLE setlist
(
    "id"        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "user"      INTEGER NOT NULL,
    "user_name" VARCHAR NOT NULL
);

CREATE TABLE setlist_entry
(
    "id"            INTEGER PRIMARY KEY AUTOINCREMENT,
    "song_id"       VARCHAR     NOT NULL,
    "file_type"     VARCHAR     NOT NULL,
    "title"         TEXT,
    "setlist_db_id" INTEGER KEY NOT NULL,
    CONSTRAINT fk_setlist
        FOREIGN KEY (setlist_db_id)
            REFERENCES setlist (id)
            ON DELETE CASCADE
);

INSERT INTO setlist (user, user_name)
VALUES (1, "yvi");
INSERT INTO setlist (user, user_name)
VALUES (2, "daniel");

INSERT INTO setlist_entry (song_id, title, setlist_db_id, file_type)
VALUES ("bubbles", "song 1", 1, "chorddown"),
       ("great", "song 2", 2, "chorddown");

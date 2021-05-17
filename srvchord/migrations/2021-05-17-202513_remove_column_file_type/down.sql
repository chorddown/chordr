CREATE TABLE setlist_entry_new
(
    "id"                INTEGER PRIMARY KEY AUTOINCREMENT,
    "song_id"           VARCHAR     NOT NULL,
    "file_type"         VARCHAR     NOT NULL,
    "title"             TEXT,
    "settings"          TEXT,
    "setlist_db_id"     INTEGER KEY NOT NULL,
    "modification_date" TIMESTAMP,
    CONSTRAINT fk_setlist
        FOREIGN KEY (setlist_db_id)
            REFERENCES setlist (id)
            ON DELETE CASCADE
);

INSERT INTO setlist_entry_new
SELECT id, song_id, file_type, title, settings, setlist_db_id, modification_date
FROM setlist_entry;
DROP TABLE setlist_entry;
ALTER TABLE setlist_entry_new
    RENAME TO setlist_entry;

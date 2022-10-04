CREATE TABLE setlist
(
    "uid"               INTEGER   NOT NULL PRIMARY KEY,
    "id"                INTEGER   NOT NULL,
    "name"              TEXT      NOT NULL,
    "sorting"           INTEGER   NOT NULL,
    "owner"             VARCHAR   NOT NULL,
    "team"              VARCHAR,
    "gig_date"          TIMESTAMP,          -- INTEGER,
    "creation_date"     TIMESTAMP NOT NULL, -- INTEGER NOT NULL,
    "modification_date" TIMESTAMP NOT NULL  -- INTEGER NOT NULL,
);

CREATE INDEX idx_setlist_id ON setlist (id);
CREATE INDEX idx_setlist_owner ON setlist (owner);

CREATE TABLE setlist_entry
(
    "uid"           INTEGER PRIMARY KEY AUTOINCREMENT,
    "song_id"       VARCHAR     NOT NULL,
    "file_type"     VARCHAR     NOT NULL,
    "title"         TEXT,
    "settings"      TEXT,
    "setlist_db_id" INTEGER KEY NOT NULL,
    CONSTRAINT fk_setlist
        FOREIGN KEY (setlist_db_id)
            REFERENCES setlist (uid)
            ON DELETE CASCADE
);

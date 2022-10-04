CREATE TEMPORARY TABLE setlist_entry_backup
(
    uid,
    song_id,
    file_type,
    title,
    settings,
    setlist_db_id
);
INSERT INTO setlist_entry_backup
SELECT uid, song_id, file_type, title, settings, setlist_db_id
FROM setlist_entry;
DROP TABLE setlist_entry;
ALTER TABLE setlist_entry_backup
    RENAME TO setlist_entry;

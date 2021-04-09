ALTER TABLE setlist_entry
    ADD COLUMN "modification_date" TIMESTAMP;

UPDATE setlist_entry
SET modification_date=CURRENT_TIMESTAMP
WHERE modification_date is null;

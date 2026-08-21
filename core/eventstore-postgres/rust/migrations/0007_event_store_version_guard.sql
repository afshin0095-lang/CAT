CREATE OR REPLACE FUNCTION cat_event_version_guard()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.event_version <= 0 THEN
        RAISE EXCEPTION 'CAT event version must be positive';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_cat_event_version_guard ON cat_events;
CREATE TRIGGER trg_cat_event_version_guard
BEFORE INSERT OR UPDATE ON cat_events
FOR EACH ROW EXECUTE FUNCTION cat_event_version_guard();

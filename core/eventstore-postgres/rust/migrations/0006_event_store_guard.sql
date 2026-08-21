CREATE OR REPLACE FUNCTION cat_event_stream_guard()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.current_sequence < OLD.current_sequence THEN
        RAISE EXCEPTION 'CAT event stream sequence cannot move backwards';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_cat_event_stream_guard ON cat_event_streams;
CREATE TRIGGER trg_cat_event_stream_guard
BEFORE UPDATE ON cat_event_streams
FOR EACH ROW EXECUTE FUNCTION cat_event_stream_guard();

CREATE OR REPLACE FUNCTION cat_event_stream_tenant_guard()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM cat_events
        WHERE stream_id = NEW.stream_id
          AND tenant_id <> NEW.tenant_id
    ) THEN
        RAISE EXCEPTION 'CAT event stream cannot cross tenant boundaries';
    END IF;
    RETURN NEW;
END;
$$;

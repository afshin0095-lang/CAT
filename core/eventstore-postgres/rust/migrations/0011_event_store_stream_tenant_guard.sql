CREATE OR REPLACE FUNCTION cat_event_stream_tenant_guard()
RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    first_tenant UUID;
BEGIN
    SELECT tenant_id INTO first_tenant
    FROM cat_events
    WHERE stream_id = NEW.stream_id
    ORDER BY sequence ASC
    LIMIT 1;

    IF first_tenant IS NOT NULL AND first_tenant <> NEW.tenant_id THEN
        RAISE EXCEPTION 'CAT event stream cannot cross tenant boundaries';
    END IF;
    RETURN NEW;
END;
$$;

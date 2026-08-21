DROP TRIGGER IF EXISTS trg_cat_event_stream_tenant_guard ON cat_events;
CREATE TRIGGER trg_cat_event_stream_tenant_guard
BEFORE INSERT ON cat_events
FOR EACH ROW EXECUTE FUNCTION cat_event_stream_tenant_guard();

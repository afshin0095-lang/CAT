# O13 — Backup and Restore

Backups protect durable business state and configuration required to reconstruct CAT.

```text
Primary Data
   ↓
Consistent Backup
   ↓
Encrypted Storage
   ↓
Retention Policy
   ↓
Restore Test
   ↓
Verified Recovery Point
```

Backup policy must define scope, frequency, retention, encryption, ownership, and recovery objectives. A backup that has never been restored is not considered operationally proven.

Restore procedures must account for schema version, event/state consistency, secrets re-provisioning, provider credentials, and replay/idempotency behavior.

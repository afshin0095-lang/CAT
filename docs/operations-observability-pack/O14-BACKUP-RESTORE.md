# O14 — Backup & Restore

Backups are useful only when restore is tested.

```text
Primary Data
    ↓
Consistent Snapshot
    ↓
Encrypted Backup
    ↓
Independent Storage
    ↓
Periodic Restore Test
```

Backups require defined retention, encryption, access controls, integrity checks, and recovery-point objectives. Restore procedures must verify schema compatibility, application compatibility, and data integrity before production cutover.
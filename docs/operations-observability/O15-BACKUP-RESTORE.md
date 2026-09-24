# O15 — Backup & Restore

Backups protect against data loss, corruption, operator error, and destructive incidents.

```mermaid
flowchart LR
D[Primary Data] --> B[Encrypted Backup]
B --> V[Restore Verification]
V --> R[Recovery Environment]
R --> P[Production Restore]
```

Define RPO (acceptable data loss) and RTO (acceptable recovery time) per data class. Backups must be encrypted, access-controlled, retention-limited, and periodically restored in a separate environment.

A backup that has never been restored successfully is not treated as proven recoverability evidence.

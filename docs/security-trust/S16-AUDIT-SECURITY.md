# S16 — Security Audit Architecture

Security audit records answer: **who/what acted, on which resource, under which policy, with what result?**

```text
Principal → Action → Policy Decision → System Operation → Outcome
                         │
                         └──────→ Audit Record
```

Audit evidence is append-oriented and correlation-aware. It must not contain raw secrets. Failed authorization attempts may be recorded according to security and privacy policy. Financially meaningful affiliate operations require stronger audit coverage than ordinary reads.

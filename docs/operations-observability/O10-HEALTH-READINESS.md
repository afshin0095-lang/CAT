# O10 — Health, Liveness and Readiness

Health endpoints answer different questions.

| Signal | Question |
|---|---|
| liveness | Is the process able to continue running? |
| readiness | Can this instance safely receive traffic/work? |
| dependency health | Are required dependencies usable? |
| business health | Is the business capability functioning? |

A dependency outage should not automatically make every process non-live. Readiness is capability-specific and should avoid cascading failure caused by over-broad health checks.

Health endpoints expose bounded diagnostic state and never return secrets, credentials, or unrestricted internal exception details.

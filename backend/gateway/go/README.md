# CAT Go Gateway

The Go gateway is the north-south HTTP boundary for CAT. This first implementation slice intentionally stays transport-focused: it owns HTTP lifecycle, request metadata, health/readiness probes, and security response headers without embedding domain or treasury logic.

## Runtime contract

- `GET /health` — process health and version.
- `GET /ready` — readiness boundary; future adapters will be checked here.
- `GET /live` — liveness-compatible probe.
- `GET /` — minimal service identity response.
- Unknown routes return JSON `404`.
- `X-Request-ID` is preserved when supplied and generated otherwise.
- Basic browser-facing security headers are emitted at the boundary.

## Configuration

| Variable | Default |
|---|---|
| `CAT_GATEWAY_ADDR` | `:8080` |
| `CAT_GATEWAY_READ_HEADER_TIMEOUT_MS` | `5000` |
| `CAT_GATEWAY_READ_TIMEOUT_MS` | `15000` |
| `CAT_GATEWAY_WRITE_TIMEOUT_MS` | `15000` |
| `CAT_GATEWAY_IDLE_TIMEOUT_MS` | `60000` |
| `CAT_GATEWAY_SHUTDOWN_TIMEOUT_MS` | `10000` |

The gateway uses only the Go standard library in this slice so the boundary remains small and dependency-light. Domain commands, authentication, rate limiting, policy enforcement, and downstream adapters belong to later layers rather than the HTTP transport itself.

## Local verification

```bash
go test ./...
go run .
```

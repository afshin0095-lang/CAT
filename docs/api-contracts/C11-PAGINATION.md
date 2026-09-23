# C11 — Pagination Contract

CAT list APIs use explicit bounded pagination.

## Offset form

`limit` is bounded by endpoint policy; `offset` must be non-negative and checked for overflow.

```json
{
  "data": [],
  "meta": {"limit": 50, "offset": 0, "has_more": true}
}
```

## Ordering

Every pageable query has a deterministic total order. Ties are resolved by a stable unique identifier. Clients must never rely on database default ordering.

## Future cursor form

Cursor pagination may replace or supplement offset pagination for high-volume streams. A cursor is opaque and must not be parsed by clients.

# Affiliate Opportunity Observability P0

## Purpose

A neutral observability contract for the Affiliate Opportunity platform: canonical metric names, minimal samples with labels, a push sink, and conversions from Sprint 0 domain results. The domain never couples to Prometheus, OTLP, or any specific backend.

## Scope

- `core/affiliate/rust/src/observability.rs` — names, samples, sink, conversion helpers.
- `core/affiliate/rust/src/opportunity_ingestion.rs` — report fields feeding discovery metrics.
- `core/affiliate/rust/tests/observability.rs` — contract tests.

## Architecture

```text
Domain results (reports, batches, statuses)
        │ conversion helpers (pure)
        ▼
MetricSample { name: canonical const, value: u64, labels }
        │ MetricSink::record
        ▼
Any backend (Prometheus, OTLP, in-memory dashboards)
```

## Data model

Metric namespace (`observability::metric_names`):

| Name | Kind | Source |
|---|---|---|
| `affiliate.discovery.candidates_received` | counter | ingestion report |
| `affiliate.discovery.candidates_rejected` | counter | ingestion report |
| `affiliate.discovery.opportunities_created` | counter | ingestion report |
| `affiliate.discovery.opportunities_changed` | counter | ingestion report |
| `affiliate.discovery.source_failures` | counter | ingestion report |
| `affiliate.opportunity.active` | gauge | lifecycle batch |
| `affiliate.opportunity.stale` | gauge | lifecycle batch |
| `affiliate.opportunity.expired` | gauge | lifecycle batch |
| `affiliate.opportunity.revalidation_requested` | counter | request store / Sprint 1 boundary |
| `affiliate.opportunity.revalidation_succeeded` | counter | execution boundary (Sprint 1) |
| `affiliate.opportunity.revalidation_failed` | counter | execution boundary (Sprint 1) |

Label keys: `source`, `batch_id`, `evaluated_at_ms`.

## API contracts

- `MetricSink::record(sample)` — infallible push boundary; implementations must not panic on unknown names.
- `InMemoryMetricSink` — captures samples, aggregates `totals()` deterministically (ordered by name).
- `ingestion_report_metrics(&OpportunityIngestionReport) -> Vec<MetricSample>` (tagged with `batch_id`).
- `lifecycle_batch_metrics(&LifecycleEvaluationBatch) -> Vec<MetricSample>` (tagged with `evaluated_at_ms`).
- `revalidation_status_metrics(RevalidationStatus, count)` — maps statuses to metric families.

## State transitions

Not applicable. Metrics observe facts produced by boundaries.

## Invariants

1. Metric names are unique and owned by the canonical constants module.
2. Emission belongs to the boundary that produces the fact. Read-only derivations (a query computing `Expired`) never emit lifecycle metrics.
3. Values are unsigned integers; counters and gauges are distinguished by name convention, matching backend registries.
4. The domain imports no telemetry dependency.

## Error handling

Sinks are infallible by contract; backends translate internal failures into their own error channels.

## Persistence behavior

`InMemoryMetricSink` is process-local; durable telemetry is the backend's responsibility.

## Concurrency behavior

Sinks take `&mut self` for push; concurrent emission requires an adapter with internal synchronization (same pattern as `MetricSink` implementations elsewhere in CAT).

## Idempotency

Conversion helpers are pure; emitting the same fact twice produces two samples (counting semantics), so boundaries must emit once per fact — the same discipline as events.

## Testing strategy

Namespace uniqueness, conversion completeness for ingestion reports and lifecycle batches, status→metric-family mapping, deterministic aggregation, and the ingestion→metrics integration round trip.

## Security considerations

Labels carry identifiers only (`source`, `batch_id`); no credentials, headers, or provider payloads can enter samples through the provided helpers.

## Future extension points

- Latency histograms for source interactions once `SourceHealthStore` gains a durable adapter.
- Sprint 1 execution metrics (attempt durations, dead-letter rates).

## Known limitations

- Counters/gauges only; no histograms in Sprint 0.
- No default backend wiring; deployments choose and configure sinks.

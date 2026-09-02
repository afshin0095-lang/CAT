# Provider Selection Engine V1

## Purpose

Select the best eligible external provider after capability filtering.

## V1 scoring

| Signal | Weight |
|---|---:|
| Commission potential | 30% |
| Reliability | 25% |
| Historical conversion | 25% |
| Latency | 10% |
| Geographic fit | 10% |

The engine first filters providers by mandatory safety/operation capabilities. Only then does it compare scores.

## Important boundary

V1 uses supplied normalized scores. It does not yet collect live affiliate economics itself. Future analytics and learning cores will feed real measurements into this engine.

## Determinism

If scores tie, selection remains deterministic through provider-name ordering.

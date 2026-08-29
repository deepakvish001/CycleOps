# CycleOps observability runbook

## Service objectives

- Pickup command availability: 99.9% per calendar month.
- Dispatch decision latency: p95 below 750 ms.
- Outbox publication lag: p95 below 30 seconds.
- Route event loss: zero accepted events.

## Required signals

Trace HTTP requests through command handling, PostgreSQL transactions and NATS publication. Tag metrics with service and operation, never customer identifiers. Log correlation IDs, tenant IDs and aggregate IDs; redact addresses and contact data.

## Triage

1. Confirm health probes and deployment revision.
2. Compare request error rate, database saturation and outbox lag.
3. Pause route consumers when poison events repeat.
4. Replay only idempotent events from the last confirmed sequence.
5. Record customer impact and recovery timestamps.

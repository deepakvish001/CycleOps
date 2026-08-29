# CycleOps

CycleOps is a multi-tenant urban waste and recycling operations platform for municipalities, housing societies, material recovery facilities, transport partners, and recyclers.

It coordinates service zones, household and commercial pickups, reusable containers, route execution, weighbridge tickets, contamination audits, material recovery, recycler contracts, diversion reporting, and transparent settlements.

## Technology

- Rust 1.85 with Axum and Tokio
- SvelteKit 2 with TypeScript
- PostgreSQL 17 with PostGIS
- SQLx migrations and compile-time queries
- NATS JetStream for durable operational events
- Redis for idempotency and short-lived dispatch state
- OpenTelemetry for traces and metrics
- Docker Compose for local infrastructure

## Architecture

The backend follows domain, application, ports, adapters, and HTTP boundaries. Every operational record is scoped by `tenant_id`; PostgreSQL row-level security provides defense in depth. Commands use idempotency keys and integrations publish through a transactional outbox.

## Development

Copy `.env.example` to `.env`, start infrastructure with `docker compose up -d`, then run `cargo test --workspace`.

Delivery is split into 120 independently reviewable pull requests. Merge by the three-digit branch prefix.

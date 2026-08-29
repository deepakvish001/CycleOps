# CycleOps architecture and merge guide

CycleOps connects waste generators, pickup fleets, recovery facilities and recyclers while preserving tenant isolation and material traceability.

## Non-negotiable invariants

- Every operational record carries `tenant_id`.
- PostgreSQL row-level security is forced for tenant tables.
- Pickup commands require idempotency keys.
- Material weights never become negative and retain source tickets.
- External events are emitted through a transactional outbox.
- Settlement calculations are deterministic and auditable.
- Logs never contain addresses, contact details or credentials.

## Merge order

Merge by branch prefix rather than GitHub PR number:

1. `001–024`: domain models
2. `025–044`: operational policies
3. `045–054`: repository ports
4. `055–066`: application services
5. `067–078`: Axum HTTP boundaries
6. `079–090`: Svelte components
7. `091–104`: test coverage
8. `105–108`: PostGIS, RLS and outbox migrations
9. `109–116`: infrastructure adapters
10. `117–120`: API contract, CI and operational documentation

After every group, rebase the next branches on current `main` and run `cargo fmt --all --check && cargo test --workspace`. Run frontend checks before merging UI or CI groups.

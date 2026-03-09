===============================================================================
RS-022 — Add asset route regression tests and README troubleshooting
===============================================================================

Title:
RS-022 Add regression coverage for static asset routes and troubleshooting guide

Description:
Document the most likely failure modes and automate checks for them.

Acceptance criteria:
- README includes troubleshooting section
- static route tests cover required wasm URLs
- curl commands documented
- common errors and fixes listed

Implementation steps:

Files:
- README.md
- server-rs/tests/static_assets.rs

README troubleshooting topics:
- wasm 404
- zero-byte wasm
- incorrect static path layout
- websocket upgrade failure
- NATS DNS/container connectivity issue
- build slowdown due to dependency invalidation

Commands documented:
- cargo test
- docker compose up --build
- curl -I for all wasm assets
- logs for server/producer/nats

Done checklist:
- [ ] troubleshooting section added
- [ ] static route tests present
- [ ] wasm curl checks documented

===============================================================================
RS-023 — Improve Docker build caching with cargo-chef or equivalent
===============================================================================

Title:
RS-023 Optimize container build speed with stronger Rust dependency caching

Description:
Reduce rebuild times by introducing cargo-chef or an equivalent multi-stage
caching pattern.

Acceptance criteria:
- rebuilding after src-only changes is significantly faster
- dependency build layers reused
- docs explain the cache strategy

Implementation steps:

Files:
- producer/Dockerfile
- server-rs/Dockerfile
- README.md

Approach:
Option A:
- introduce cargo-chef planner/builder pattern

Option B:
- stay with manual Cargo.toml/Cargo.lock-first layering if good enough

Recommendation:
- implement only after the baseline works
- do not risk destabilizing early tickets

Done checklist:
- [ ] cache strategy improved
- [ ] Dockerfiles still readable
- [ ] rebuild timing improved

===============================================================================
RS-024 — Final end-to-end smoke pack and handoff docs
===============================================================================

Title:
RS-024 Add final smoke commands, acceptance checklist, and handoff notes

Description:
Create a clean final handoff doc for future development phases.

Acceptance criteria:
- end-to-end startup documented
- tests documented by category
- known follow-on enhancements listed
- final acceptance checklist present

Implementation steps:

Files:
- README.md

README should include:
1. Quickstart
2. Local development
3. Running tests
4. Static asset verification
5. Troubleshooting
6. Future improvements:
   - auth
   - multiple symbols
   - richer table schema
   - replay mode
   - metrics/telemetry

Done checklist:
- [ ] docs complete
- [ ] startup verified from clean clone
- [ ] smoke checklist complete
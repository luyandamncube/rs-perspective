===============================================================================
RS-018 — Add Perspective server bootstrap module
===============================================================================

Title:
RS-018 Introduce Perspective server/table bootstrap in Rust

Description:
Create the Rust-side Perspective bootstrap code that initializes the server/table
state and prepares for websocket connections.

Acceptance criteria:
- Perspective server state initializes on app startup
- schema/table creation succeeds
- module isolated behind server-rs/src/perspective.rs
- unit test covers schema/bootstrap path where feasible

Implementation steps:

Files:
- server-rs/src/perspective.rs
- server-rs/src/lib.rs
- server-rs/src/main.rs

Code notes:
- pin exact perspective_server / perspective_client crate versions
- define schema matching tick rows:
  - symbol
  - price
  - size
  - ts
- expose app state holding perspective server/table handles

Tests:
- bootstrap test creates state successfully
- schema field names/types match expected row keys

Commands:
- cd server-rs && cargo test

Done checklist:
- [ ] Perspective state boots
- [ ] schema fixed and documented
- [ ] crate versions pinned exactly

===============================================================================
RS-019 — Wire NATS consumer into Perspective table updates
===============================================================================

Title:
RS-019 Update Perspective table from validated NATS ticks

Description:
Instead of only logging validated ticks, update the Perspective table with each
new row.

Acceptance criteria:
- valid ticks update Perspective table
- malformed ticks skipped safely
- update path isolated in helper function
- tests cover row wrapping/update request construction

Implementation steps:

Files:
- server-rs/src/nats_consumer.rs
- server-rs/src/perspective.rs
- server-rs/src/ticks.rs

Code notes:
- careful version-specific imports for TableInitOptions / UpdateOptions
- isolate version-sensitive calls
- use helper:
  - async fn apply_tick(table, tick) -> Result<()>

Tests:
- unit test row payload builder
- unit test/integration-lite apply_tick helper if practical

Commands:
- cd server-rs && cargo test
- docker compose up --build

Done checklist:
- [ ] NATS updates table
- [ ] helper isolated from main loop
- [ ] tests protect row/update path

===============================================================================
RS-020 — Add websocket endpoint for Perspective sessions
===============================================================================

Title:
RS-020 Expose Perspective websocket endpoint through Axum

Description:
Add the websocket route used by the browser to connect to the Perspective server.

Acceptance criteria:
- websocket route exists
- route uses ConnectInfo-compatible Axum startup
- handshake returns upgrade response
- integration test verifies websocket upgrade works

Implementation steps:

Files:
- server-rs/src/app.rs
- server-rs/src/handlers.rs
- server-rs/tests/websocket.rs

Code notes:
- app must be served with:
  app.into_make_service_with_connect_info::<SocketAddr>()
- add route such as /ws or Perspective-required route shape
- handler bridges websocket to Perspective session manager

Tests:
- spawn server on ephemeral port
- send websocket handshake
- assert HTTP 101 Switching Protocols or successful websocket connect

Commands:
- cd server-rs && cargo test --test websocket -- --nocapture

Done checklist:
- [ ] websocket route present
- [ ] ConnectInfo startup preserved
- [ ] websocket integration test passes

===============================================================================
RS-021 — Replace placeholder index.html with live Perspective viewer page
===============================================================================

Title:
RS-021 Serve browser page that connects to websocket and renders live table

Description:
Replace the placeholder HTML with a working Perspective page using vendored JS
assets and correct wasm paths.

Acceptance criteria:
- browser page loads without missing asset errors
- Perspective viewer initializes
- connects to websocket endpoint
- live rows appear from producer
- no “Missing perspective-client.wasm” error

Implementation steps:

Files:
- server-rs/static/index.html
- server-rs/static/js/[supporting scripts if needed]

Page responsibilities:
- import vendored Perspective JS
- configure wasm locations exactly as expected by that bundle version
- create viewer element
- connect to websocket
- open table/view
- display live data

Manual validation:
- open browser at http://127.0.0.1:8080/
- confirm table visible
- observe new rows arriving

Additional guardrail:
- browser console should not show 404s for wasm or worker assets

Done checklist:
- [ ] page loads
- [ ] no wasm 404s
- [ ] no zero-byte wasm issue
- [ ] live table visible
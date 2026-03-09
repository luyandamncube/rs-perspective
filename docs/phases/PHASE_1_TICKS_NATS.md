===============================================================================
RS-015 — Define shared tick contract in server and validate incoming rows
===============================================================================

Title:
RS-015 Add server-side Tick DTO and row validation helpers

Description:
Create the server-side representation of incoming ticks and helpers that validate
and transform them into row objects for Perspective updates later.

Acceptance criteria:
- server has Tick DTO matching producer JSON
- invalid payloads are rejected clearly
- valid ticks convert to row objects
- unit tests cover happy path and error cases

Implementation steps:

Files:
- server-rs/src/ticks.rs
- server-rs/src/lib.rs

Code notes:
- define Tick struct with serde Deserialize/Serialize
- fn parse_tick(bytes: &[u8]) -> Result<Tick>
- fn tick_to_row(t: &Tick) -> serde_json::Value

Tests:
- parse valid JSON
- reject malformed JSON
- reject negative/non-finite price if enforced
- row contains expected keys

Commands:
- cd server-rs && cargo test

Done checklist:
- [ ] Tick DTO created
- [ ] validation helpers tested
- [ ] row conversion tested

===============================================================================
RS-016 — Subscribe server to NATS and log validated ticks
===============================================================================

Title:
RS-016 Add NATS consumer task in server

Description:
Teach the server to connect to NATS, subscribe to the configured subject, parse
incoming ticks, and log validated rows. No Perspective updates yet.

Acceptance criteria:
- server connects to NATS on startup
- subscribes to configured subject
- valid ticks are parsed and logged
- invalid ticks are logged and skipped
- application stays up if malformed messages appear

Implementation steps:

Files:
- server-rs/src/nats_consumer.rs
- server-rs/src/main.rs
- server-rs/src/config.rs

Code notes:
- spawn background tokio task
- async_nats::connect
- client.subscribe(subject)
- loop messages
- parse_tick(msg.payload.as_ref())

Tests:
- unit tests for consumer helpers if extracted
- manual docker compose smoke:
  - producer publishes
  - server logs validated ticks

Commands:
- docker compose up --build
- docker compose logs -f server-rs
- docker compose logs -f producer

Done checklist:
- [ ] server subscribes to NATS
- [ ] valid ticks logged
- [ ] invalid messages do not crash server

===============================================================================
RS-017 — Add integration test with testcontainers + NATS
===============================================================================

Title:
RS-017 Add NATS-backed integration test for server startup and health

Description:
Use testcontainers to spin up a real NATS instance during tests, then start the
server on an ephemeral port and verify health plus basic NATS connectivity path.

Acceptance criteria:
- integration test starts NATS container automatically
- server boots against test NATS URL
- /health returns 200
- test publishes one valid tick to NATS
- server remains healthy after receiving tick

Implementation steps:

Files:
- server-rs/tests/nats_integration.rs

Dependencies:
- testcontainers
- reqwest (for HTTP assertions)
- tokio-tungstenite may come later for websocket tests

Test flow:
1. Start NATS container
2. Determine mapped port
3. Build AppConfig with that URL
4. Bind server to 127.0.0.1:0
5. Spawn server
6. Assert GET /health == 200
7. Publish valid tick
8. Optionally wait briefly
9. Assert server still healthy

Commands:
- cd server-rs && cargo test --test nats_integration -- --nocapture

Done checklist:
- [ ] testcontainers integration working
- [ ] health asserted
- [ ] tick publish path exercised

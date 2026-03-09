Rust demo:

- producer/: Rust binary publishing JSON ticks to NATS
- server-rs/: Rust Axum server hosting Perspective UI and websocket endpoint
- docker-compose.yml at repo root
- README.md with quickstart, troubleshooting, and test commands

Primary runtime flow:
Producer -> NATS subject `ticks` -> Server subscribes -> Perspective table updates -> Browser renders live table

--------------------------------------------------------------------------------
0. Recommended design decisions
--------------------------------------------------------------------------------

1. Pin versions aggressively
   - Rust Docker image: rust:1.88
   - NATS image: nats:2.10
   - Axum: 0.8
   - Tokio: 1.x
   - async-nats: pinned exact/minor compatible version
   - perspective crates: pin exact versions that are known compatible with vendored JS/wasm assets

2. Keep server testable
   - server-rs should expose a library module:
     - config
     - router builder
     - health handler
     - tick validation/wrapping helpers
     - optional NATS consumer bootstrap
   - main.rs should only wire config + app + listener

3. Use vendored Perspective static assets
   Simplest reliable approach:
   - vendor the exact Perspective dist assets under server-rs/static/
   - do NOT depend on a CDN at runtime
   - add a script/check that verifies:
     - perspective-client.wasm exists and non-zero
     - perspective-viewer.wasm exists and non-zero
     - perspective-server.wasm exists and non-zero
   - ensure the JS bundle you serve matches the wasm path layout you vendor

4. Optimize Docker builds early
   - copy Cargo.toml/Cargo.lock first
   - cargo fetch before copying src
   - install build deps once:
     - cmake
     - pkg-config
     - build-essential
     - ca-certificates
   - cargo-chef can be phase 4 if needed; don’t block Phase 1 on it

5. Tests from the start
   - Producer:
     - unit test tick generation
     - unit test JSON serialization
   - Server:
     - unit test row validation/wrapping
     - router health endpoint test
     - static wasm existence/serving test
   - Integration:
     - use testcontainers to start NATS
     - boot Axum on ephemeral port
     - assert /health
     - assert websocket upgrade endpoint responds correctly

--------------------------------------------------------------------------------
1. Target repo layout
--------------------------------------------------------------------------------

repo-root/
  docker-compose.yml
  README.md
  .env.example
  producer/
    Cargo.toml
    Cargo.lock
    Dockerfile
    src/
      main.rs
      lib.rs
      tick.rs
  server-rs/
    Cargo.toml
    Cargo.lock
    Dockerfile
    src/
      main.rs
      lib.rs
      config.rs
      app.rs
      handlers.rs
      ticks.rs
      nats_consumer.rs
      perspective.rs
    static/
      index.html
      js/
        perspective.js
        [other vendored js/worker files if required]
      wasm/
        perspective-client.wasm
        perspective-viewer.wasm
      server/
        dist/
          wasm/
            perspective-server.wasm
    tests/
      health.rs
      websocket.rs
      nats_integration.rs
      static_assets.rs
  scripts/
    verify_perspective_assets.sh
    vendor_perspective_assets.sh

--------------------------------------------------------------------------------
2. Phase overview
--------------------------------------------------------------------------------

Phase 1: Workspace bootstrap and deterministic local startup
Outcome:
- Repo boots with docker compose
- NATS reachable
- Server responds on /health
- Static files served
- Producer can publish demo ticks
- Initial unit tests pass

Phase 2: Tick contract and NATS ingestion
Outcome:
- Shared/consistent tick JSON format
- Server subscribes to NATS
- Server validates incoming ticks
- Integration test proves NATS -> server path exists

Phase 3: Perspective live table wiring
Outcome:
- Perspective assets served correctly
- Websocket endpoint works
- Browser connects and shows live rows
- Integration/static checks prevent wasm regressions

Phase 4: Hardening and developer experience
Outcome:
- Faster cached builds
- Stronger troubleshooting docs
- Asset verification script
- Final smoke/integration commands documented
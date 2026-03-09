===============================================================================
RS-010 — Create workspace scaffold and pinned dependencies
===============================================================================

Title:
RS-010 Create baseline repo scaffold with pinned Rust/Axum/NATS versions

Description:
Create the initial project layout with separate producer and server-rs crates,
pinned dependency versions, and a root README. The purpose is to establish a
stable baseline with minimal moving parts.

Acceptance criteria:
- producer/ and server-rs/ compile locally
- root README explains startup flow
- dependency versions are pinned explicitly
- no Perspective runtime yet, only static placeholder capability
- cargo test passes in both crates

Implementation steps:

Files to create:
- README.md
- .env.example
- producer/Cargo.toml
- producer/src/main.rs
- producer/src/lib.rs
- producer/src/tick.rs
- server-rs/Cargo.toml
- server-rs/src/main.rs
- server-rs/src/lib.rs
- server-rs/src/config.rs
- server-rs/src/app.rs
- server-rs/src/handlers.rs
- server-rs/static/index.html

Recommended dependency baseline:

producer/Cargo.toml
- tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
- async-nats = "0.39"   (or exact compatible version selected at implementation time)
- serde = { version = "1", features = ["derive"] }
- serde_json = "1"
- rand = "0.8"
- chrono = { version = "0.4", features = ["serde", "clock"] }
- anyhow = "1"

server-rs/Cargo.toml
- axum = { version = "0.8", features = ["ws"] }
- tokio = { version = "1", features = ["macros", "rt-multi-thread", "time", "net"] }
- tower = "0.5"
- tower-http = { version = "0.6", features = ["fs", "trace"] }
- serde = { version = "1", features = ["derive"] }
- serde_json = "1"
- anyhow = "1"
- tracing = "0.1"
- tracing-subscriber = { version = "0.3", features = ["env-filter"] }

Drop-in starter code notes:

producer/src/tick.rs
- define Tick struct:
  - symbol: String
  - price: f64
  - size: u64
  - ts: DateTime<Utc>
- fn sample_tick(symbol: &str) -> Tick

producer/src/lib.rs
- export tick module
- add serialize_tick(&Tick) -> String

producer/src/main.rs
- read NATS_URL and NATS_SUBJECT defaults
- connect to NATS
- publish one sample tick every second

server-rs/src/config.rs
- define AppConfig:
  - http_port: u16
  - nats_url: String
  - nats_subject: String
- impl from_env()

server-rs/src/handlers.rs
- health() -> "ok"

server-rs/src/app.rs
- build_router() with:
  - GET /health
  - static ServeDir on /

server-rs/src/main.rs
- init tracing
- read config
- bind 0.0.0.0:HTTP_PORT
- app.into_make_service_with_connect_info::<SocketAddr>()

server-rs/static/index.html
- minimal page:
  - title
  - “server alive” message
  - mention that Perspective wiring comes in Phase 3

Tests to add/run:
Producer:
- unit: sample_tick creates positive price/size and correct symbol
- unit: serialize_tick outputs valid JSON with required keys

Server:
- unit/integration-lite: GET /health returns 200 and "ok"

Commands:
- cd producer && cargo test && cargo run
- cd server-rs && cargo test && cargo run
- curl http://127.0.0.1:8080/health

Done checklist:
- [ ] crates compile
- [ ] tests pass
- [ ] /health works
- [ ] producer publishes ticks without panic
- [ ] versions pinned in Cargo.toml

===============================================================================
RS-011 — Add Dockerfiles and docker-compose baseline
===============================================================================

Title:
RS-011 Add cache-friendly Dockerfiles and docker-compose local stack

Description:
Containerize NATS, producer, and server-rs with build-layer caching and the
required native build dependencies.

Acceptance criteria:
- docker compose up --build starts all services
- server accessible on localhost:8080
- producer connects to NATS
- Dockerfiles use rust:1.88 and install required build packages
- build layers are structured for cache reuse

Implementation steps:

Files:
- docker-compose.yml
- producer/Dockerfile
- server-rs/Dockerfile

docker-compose.yml services:
- nats
  - image: nats:2.10
  - ports: 4222:4222
- server-rs
  - build: ./server-rs
  - environment:
    - HTTP_PORT=8080
    - NATS_URL=nats://nats:4222
    - NATS_SUBJECT=ticks
  - depends_on: [nats]
  - ports: 8080:8080
- producer
  - build: ./producer
  - environment:
    - NATS_URL=nats://nats:4222
    - NATS_SUBJECT=ticks
  - depends_on: [nats]

Dockerfile pattern:
1. FROM rust:1.88 AS builder
2. apt-get install:
   - cmake
   - pkg-config
   - build-essential
   - ca-certificates
3. copy Cargo.toml/Cargo.lock
4. create dummy src/main.rs or copy minimal src for dependency caching
5. cargo fetch
6. copy full src
7. cargo build --release
8. runtime image can be debian bookworm-slim or similar
9. copy binary and static dir if needed

Tests to add/run:
- no new Rust tests required
- smoke test:
  - docker compose up --build
  - curl /health succeeds

Commands:
- docker compose up --build
- curl http://127.0.0.1:8080/health
- docker compose logs -f producer
- docker compose logs -f server-rs

Done checklist:
- [ ] compose starts successfully
- [ ] build uses rust:1.88
- [ ] build deps present
- [ ] /health reachable from host
- [ ] logs show producer connected to NATS

===============================================================================
RS-012 — Add producer unit-tested tick generator
===============================================================================

Title:
RS-012 Add deterministic tick generator and JSON publish helper

Description:
Move producer logic into testable library code. The binary should become a thin
loop over a reusable tick-generation function.

Acceptance criteria:
- tick generation lives in library code
- producer main uses library code only
- unit tests cover generation and encoding
- JSON payload matches server expectations

Implementation steps:

Files:
- producer/src/lib.rs
- producer/src/tick.rs
- producer/src/main.rs

Code notes:
- Tick struct:
  - symbol
  - price
  - size
  - ts
- fn next_tick(prev_price: f64, symbol: &str) -> Tick
- fn encode_tick_json(tick: &Tick) -> Result<String>

Testing ideas:
- symbol preserved
- price finite and > 0
- size > 0
- serde_json round-trip works
- timestamps serialize

Commands:
- cd producer && cargo test
- docker compose up --build producer

Done checklist:
- [ ] library-based generator exists
- [ ] binary is thin wrapper
- [ ] JSON tests pass
- [ ] output payload stable and readable

===============================================================================
RS-013 — Add server router module and health/static tests
===============================================================================

Title:
RS-013 Refactor server into library router with testable health and static serving

Description:
Make the Axum app testable by moving router construction into library code.
Add tests for /health and for serving index.html from static/.

Acceptance criteria:
- build_router() exposed from library
- /health returns 200
- / returns index.html
- server binary only handles config and listener binding

Implementation steps:

Files:
- server-rs/src/lib.rs
- server-rs/src/app.rs
- server-rs/src/handlers.rs
- server-rs/tests/health.rs

Code notes:
- pub fn build_router(...) -> Router
- use tower::ServiceExt or axum test request pattern
- static served via ServeDir::new("static").append_index_html_on_directories(true)

Tests:
- GET /health -> 200, body "ok"
- GET / -> 200, contains basic html text

Commands:
- cd server-rs && cargo test

Done checklist:
- [ ] router extracted
- [ ] health test passes
- [ ] index route test passes
- [ ] main is wiring-only

===============================================================================
RS-014 — Vendor Perspective assets with verification guardrails
===============================================================================

Title:
RS-014 Vendor Perspective JS/WASM assets and add non-zero verification script

Description:
Before live Perspective wiring, establish the static asset layout and add a
verification script that prevents broken zero-byte wasm regressions.

Acceptance criteria:
- required Perspective files exist under server-rs/static
- script fails if any required wasm is missing or zero bytes
- server can serve those files with 200
- curl -I shows non-zero content-length for wasm assets

Implementation steps:

Files:
- scripts/vendor_perspective_assets.sh
- scripts/verify_perspective_assets.sh
- server-rs/static/js/[vendored assets]
- server-rs/static/wasm/perspective-client.wasm
- server-rs/static/wasm/perspective-viewer.wasm
- server-rs/static/server/dist/wasm/perspective-server.wasm
- server-rs/tests/static_assets.rs

Approach:
- simplest reliable method: vendor exact distribution files from the matching
  npm/crate-compatible Perspective release into the repo
- keep directory structure stable
- do not rely on CDN paths

verify_perspective_assets.sh should:
- exit non-zero if file missing
- exit non-zero if size <= 0
- print file sizes

Rust static asset test:
- GET /wasm/perspective-client.wasm -> 200
- GET /wasm/perspective-viewer.wasm -> 200
- GET /server/dist/wasm/perspective-server.wasm -> 200

Manual smoke:
- curl -I http://127.0.0.1:8080/wasm/perspective-client.wasm
- curl -I http://127.0.0.1:8080/wasm/perspective-viewer.wasm
- curl -I http://127.0.0.1:8080/server/dist/wasm/perspective-server.wasm

Done checklist:
- [ ] assets vendored
- [ ] verify script works
- [ ] wasm routes return 200
- [ ] content-length non-zero
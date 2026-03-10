## Root worksapce manifest structure
```bash
tree -L 5 -I ".git|node_modules|__pycache__|target"

rust-perspective-streaming/
├── Cargo.toml          # workspace only
├── producer/           # crate
└── server-rs/          # crate
```


## Scaffhold Order
build these in order : 

1. Root (workspace)
```bash
Cargo.toml
.gitignore
.env.example
README.md
```

2. Producer first (crate 1)
```bash
producer/Cargo.toml
producer/src/tick.rs
producer/src/lib.rs
producer/src/main.rs
```

3. Then server (crate 2)

```bash
server-rs/Cargo.toml
server-rs/src/config.rs
server-rs/src/handlers.rs
server-rs/src/app.rs
server-rs/src/lib.rs
server-rs/src/main.rs
server-rs/static/index.html
server-rs/tests/health.rs
```

## Run crates
```bash


# run server
cargo test -p server-rs
cargo run -p server-rs

# run producer
cargo test -p producer
cargo run -p producer

# run nats
nats-server -m 8222

# health check

curl http://127.0.0.1:8080/health   
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8222/varz     # nats 
# run together 
docker compose build --up
```

## Vendor Assets
```bash
chmod +x scripts/vendor_perspective_assets.sh scripts/verify_perspective_assets.sh
./scripts/vendor_perspective_assets.sh 4.2.0
./scripts/verify_perspective_assets.sh
```

## Runtime Flow

```bash
NATS -> ingest::nats_consumer -> mpsc channel -> pipeline::batcher
     -> transforms::demo_rows -> sinks::perspective
     -> hosted table -> browser viewer

NATS message
-> ingest::nats_consumer parses to Tick
-> tokio::mpsc::Sender<Tick>
-> pipeline::batcher buffers Tick values
-> transforms::demo_rows converts batch to DemoRow values
-> sinks::perspective applies one batch update
-> hosted table visible to frontend     
```

## Data Contracts

1. Module responsibilities and signatures
```bash
server-rs/src/models/mod.rs
server-rs/src/models/rows.rs
server-rs/src/models/tick.rs
```
2. Transform layer
```bash
server-rs/src/transforms/mod.rs
server-rs/src/transforms/demo_rows.rs
```
3. Sink contract
```bash
server-rs/src/sinks/mod.rs
server-rs/src/sinks/perspective.rs
```
4. Ingest contract
```bash
server-rs/src/ingest/mod.rs
server-rs/src/ingest/nats_consumer.rs
```
5. Pipeline contract
```bash
server-rs/src/pipeline/mod.rs
server-rs/src/pipeline/backpressure.rs
server-rs/src/pipeline/batcher.rs
server-rs/src/pipeline/flush_policy.rs
```
6. Config contract
```bash
server-rs/src/config.rs
```
7. Main orchestration
```bash
server-rs/src/main.rs
```
8. lib.rs
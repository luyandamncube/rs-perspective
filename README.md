## Root worksapce manifest structure
```bash
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
# run producer
cargo test -p producer
cargo run -p producer

# run server
cargo test -p server-rs
cargo run -p server-rs

# health check

curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/

# run together 
docker compose build --up
```
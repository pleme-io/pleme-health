# pleme-health

pleme-health library

## Installation

```toml
[dependencies]
pleme-health = "0.1"
```

## Usage

```rust
use pleme_health::HealthChecker;

let health = HealthChecker::new()
    .add_postgres(pool.clone())
    .add_redis(redis.clone())
    .build();

// Mount at /health
let app = Router::new().merge(health.router());
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible builds:

```bash
nix develop            # Dev shell with Rust toolchain
nix run .#check-all    # cargo fmt + clippy + test
nix run .#publish      # Publish to crates.io (--dry-run supported)
nix run .#regenerate   # Regenerate Cargo.nix
```

## License

MIT - see [LICENSE](LICENSE) for details.

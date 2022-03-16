# rust-playground

Playground for rust

## Notes

### Run

```bash
cargo run -- myArg
```

### Build

```bash
cargo build
/target/debug/calculator myArg
```

or

```bash
rustc ./src/main.rs
./main myArg
```

### Release

```bash
cargo build --release
./target/release/calculator myArg
```

### Watch and run

```bash
cargo install cargo-watch
cargo watch -x run
// watch and send arguments
cargo watch -x 'run -- myArg'
// watch and clear console:
cargo watch -c -w src -x run
```

### Generate doc (also generate doc for dependencies)

```bash
cargo doc --open
```

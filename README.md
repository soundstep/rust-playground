# rust-playground

Playground for rust

## Notes

### Run

```
cargo run -- myArg
```

### Build

```
cargo build
/target/debug/calculator myArg
```

or

```
rustc ./src/main.rs
./main myArg
```

### Release

```
cargo build --release
./target/release/calculator myArg
```

### Watch and run

```
cargo install cargo-watch
cargo watch -x run
// watch and send arguments
cargo watch -x 'run -- myArg'
// watch and clear console:
cargo watch -c -w src -x run
```

### Generate doc (also generate doc for dependencies)

```
cargo doc --open
```

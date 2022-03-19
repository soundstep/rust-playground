# rust-playground

Playground for rust

## Notes

### Run

```bash
cargo run -- myArg
```

Run one file only:

```bash
cargo install cargo-script
cargo script src/struct.rs
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

### Create a lib inside a project

```
cargo new --lib my_lib
```

### Run test

```
// run all
cargo test
// run serially
cargo test -- --test-threads=1
// run one
cargo test -- my_test
cargo test --test integration_test
```

Ignore one test:

```
#[test]
#[ignore]
```

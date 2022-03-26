# rust-playground

Playground for rust

## Notes

### Create project

```bash
cargo new my_project
```

Create a lib inside a project:

```
cargo new --lib my_lib
```

Create a derive macro inside a my_project:

```
cargo new hello_macro --lib
cd hello_macro
cargo new hello_macro_derive --lib
```

### Run

```bash
cargo run -- myArg
```

Run one file only:

```bash
cargo install cargo-script
cargo script src/struct.rs
```

Run a package in workspace:

```bash
cargo run -p my_package
```

### Check the code without running

```
cargo check
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

### Run test

```
// run all
cargo test
// run serially
cargo test -- --test-threads=1
// run one
cargo test -- my_test
cargo test --test integration_test
// show print even if successful
cargo test -- --nocapture
// run tests of a specific package in a workspace
cargo test -p my_package
```

Ignore one test:

```
#[test]
#[ignore]
```

### Monitor threads

Name the thread:

```
let builder = thread::Builder::new().name("foo".into());
```

Filter threads using [htop](https://htop.dev/).

```
htop
```

Show custom name in htop:

F2 (Setup) > Display Options > Show custom thread names

Filter thread name:

F4 (Filter) > foo

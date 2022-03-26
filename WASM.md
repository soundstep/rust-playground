# WASM

https://rustwasm.github.io/docs/book/introduction.html

## Install

https://rustwasm.github.io/

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install cargo-generate
```

## Create new project

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

## Build

```
wasm-pack build
npm init wasm-app www
```

## Run tests

```
wasm-pack test --chrome --headless
```

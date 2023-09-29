# README

Rust wasm demo project built with [Yew](https://yew.rs/), [Trunk](https://trunkrs.dev/) and [TailwindCSS](https://tailwindcss.com/).

## Prerequisites

### Rust Target

Add build support for WebAssembly platform.

```sh
rustup target add wasm32-unknown-unknown
```

### Trunk

[Trunk](https://trunkrs.dev/) is a WASM web application bundler for Rust.

```sh
cargo install --locked trunk  # install trunk
trunk --help  # check if successfully installed
```

### pnpm

[pnpm](https://pnpm.io/) is highly recommended as a replacement of [npm](https://www.npmjs.com/).

## Dependencies

```sh
pnpm install  # install dependencies specified in package.json
```

### TailwindCSS

[TailwindCSS](https://tailwindcss.com/) is a utility-first CSS framework and the CLI is used for css generation.

```sh
# check if tailwindcss has been successfully installed
pnpm exec tailwindcss --help
```

### Binaryen

[Binaryen](https://github.com/WebAssembly/binaryen) contains optimizer and toolchain for WebAssembly.

```sh
# check if binaryen has been successfully installed
pnpm exec wasm-opt --help
```

## Getting Started

```sh
trunk serve  # build debug version and serve locally with hot reload
trunk build --release  # build release version
```

## Todo

- [ ] apply trunk post hook only when build release version.

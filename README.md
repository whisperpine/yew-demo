# README

Rust wasm demo project with [Yew](https://yew.rs/), [Trunk](https://trunkrs.dev/) and [TailwindCSS](https://tailwindcss.com/).

### Pre-request

#### Rust Target
Add build support for WebAssembly platform.
```sh
rustup target add wasm32-unknown-unknown
```

#### Trunk
[Trunk](https://trunkrs.dev/) is a WASM web application bundler for Rust.
```sh
cargo install --locked trunk    # install trunk
trunk --help        # check if successfully installed
```

#### TailwindCSS
[TailwindCSS](https://tailwindcss.com/) is a utility-first CSS framework and the CLI is used for css generation.
```sh
npm install -g tailwindcss      # install tailwind CLI
tailwindcss --help  # check if successfully installed
```

#### Binaryen
[Binaryen](https://github.com/WebAssembly/binaryen) contains optimizer and toolchain for WebAssembly.

Download executables in the [release page](https://github.com/WebAssembly/binaryen/releases).

Append decompressed binaryen-xxx/bin directory to `PATH` environment variable.

```sh
wasm-opt --help     # make sure this command is available
```

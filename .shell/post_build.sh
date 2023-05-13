#! /usr/bin/sh

# 配合 Trunk.toml 中的 hook, 参与打包流程
pnpm exec wasm-opt -Oz -o dist/.stage/*.wasm dist/.stage/*.wasm

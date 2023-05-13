#! /usr/bin/sh

# 配合 Trunk.toml 中的 hook, 参与打包流程

NODE_ENV=production \
    pnpm exec tailwindcss \
    --config tailwind.config.js \
    --input ./assets/tailwind_input.css \
    --output ./assets/tailwind_output.css

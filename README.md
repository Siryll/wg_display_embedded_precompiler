# wg_display_embedded_precompiler
Precompiler and GitHub Action to build widgets for the [WG Display embedded port](https://github.com/Siryll/wg_display_embedded_multi).

RISV-32 is not supported by Cranelift, so widgets are precompiled with Wasmtime for the `pulley32` target.

## Use as GitHub Action

In the widget repository (see [wg_display_embedded_widget_template](https://github.com/Siryll/wg_display_embedded_widget_template)):

```yaml
on:
  push:
    tags: ["v*"]
permissions:
  contents: write
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Siryll/wg_display_embedded_precompiler@v0.1.0
```

This builds `widget/`, creates `widget.precompiled.wasm` and uploads it to the release of the tag.

| Input | Default | |
|---|---|---|
| `widget-dir` | `widget` | Widget crate directory, its library must be named `widget` |
| `use-wasm-opt` | `true` | Run `wasm-opt -Oz` |
| `release` | `true` | Upload to the release of the current tag |

## Use manually

This requires a wasm binary compiled for the `wasm32-unknown-unknown` target that was converted to a component with [wasm-tools](https://github.com/bytecodealliance/wasm-tools):
```bash
wasm-tools component new widget.wasm -o widget.component.wasm
```

## Usage
Build pre-compile script:
```bash
cargo build --release
```

Precompile WASM Component
```bash
./target/release/wg-display-embedded-precompiler widget.component.wasm widget.precompiled.wasm
```

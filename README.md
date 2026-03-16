# wg_display_embedded_precompiler
Precompiler script to create widgets for the wg_display embedded port

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
./target/release/wasm-precompiler widget.component.wasm widget.compiled
```
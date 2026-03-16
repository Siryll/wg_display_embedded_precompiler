use anyhow::{Context, Result};
use std::path::PathBuf;
use wasmtime::*;

// Precompile WASM components to run on the ESP32-S3
// Used by the wg_display_embedded_widget template to precompile and automatically provide a built binary via github actions
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.wasm> <output.compiled>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} add.wasm add.compiled", args[0]);
        std::process::exit(1);
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  ESP32 WASM Precompiler (Wasmtime 41.0.2 + Pulley)    ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();
    println!("Input:  {}", input.display());
    println!("Output: {}", output.display());
    println!();

    // Settings must match those in wg_display_embedded/embedded_app/src/runtime/mod.rs
    let mut config = Config::new();
    
    // Use the pulley32 target since this is the only supported no_std target
    config.target("pulley32")?;
    
    config.wasm_component_model(true);
    
    // disable many optional features: https://github.com/bytecodealliance/wasmtime/blob/main/examples/min-platform/embedding/wasmtime-platform.h
    config.wasm_bulk_memory(true);
    config.wasm_simd(false);
    config.wasm_relaxed_simd(false);
    config.wasm_multi_memory(false);
    config.gc_support(false);

    config.signals_based_traps(false);
    config.wasm_multi_value(false);
    config.wasm_tail_call(false);
    
    config.memory_reservation(0);
    config.memory_guard_size(0);
    config.memory_init_cow(false);
    config.concurrency_support(false);

    println!("Creating Wasmtime engine with ESP32-compatible config...");
    println!("  Target: pulley32 (32-bit little-endian for ESP32-S3)");
    let engine = Engine::new(&config)?;

    // Load the WASM module
    println!("Loading WASM module...");
    let wasm_bytes = std::fs::read(&input)
        .context("Failed to read input WASM file")?;
    
    println!("  Input size: {} bytes ({:.2} KB)", wasm_bytes.len(), wasm_bytes.len() as f64 / 1024.0);
    
    println!("Compiling module to pulley32...");
    let compiled = engine.precompile_component(&wasm_bytes)?;

    println!("Module compiled successfully!");
    println!("  Compiled size: {} bytes ({:.2} KB)", compiled.len(), compiled.len() as f64 / 1024.0);
    println!("  Compression ratio: {:.1}%", (compiled.len() as f64 / wasm_bytes.len() as f64) * 100.0);

    std::fs::write(&output, &compiled).context("Failed to write output file")?;

    println!();
    println!("✓ Successfully compiled to {}", output.display());
    println!();
    
    Ok(())
}

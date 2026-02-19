# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pyrkafka is a Python extension module written in Rust using PyO3. It wraps the `rdkafka` Rust crate to provide a Pythonic interface to Apache Kafka. The library exposes two classes: `PyrKafkaProducer` and `PyrKafkaConsumer`.

## Build Commands

```bash
# Development build (installs into current Python venv)
maturin develop

# Release wheel build
maturin build --release

# Rust-only build (no Python wheel)
cargo build

# Format and lint Rust code
cargo fmt
cargo clippy
```

Requires a Rust toolchain and `maturin` (`pip install maturin`). The build also needs CMake installed (used by rdkafka's cmake-build feature).

## Architecture

- **`src/lib.rs`** — PyO3 module entry point using the Bound API, registers `PyrKafkaConsumer` and `PyrKafkaProducer`
- **`src/producer.rs`** — `PyrKafkaProducer` wrapping rdkafka's `BaseProducer`. Supports `produce` (keyless), `produce_with_key`, and `flush`. Accepts optional config dict. Auto-flushes on drop with 10s timeout.
- **`src/consumer.rs`** — `PyrKafkaConsumer` wrapping rdkafka's `BaseConsumer`. Implements Python iterator protocol (`__iter__`/`__next__`). Releases the GIL during Kafka polling. Uses `Arc<Mutex<>>` for thread-safe stop signaling. Accepts optional config dict that overrides defaults.

## Key Dependencies

- **PyO3 0.24** with ABI3 stable API (Python 3.13+), using the Bound API
- **rdkafka 0.36** with features: `cmake-build`, `ssl-vendored`, `gssapi-vendored`, `libz-static`, `zstd`
- **Maturin** as the Python build backend

## CI/CD

`.github/workflows/release.yaml` builds wheels on version tags (e.g., `1.0.0`, `1.0.0a1`). Builds for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x64), then publishes to PyPI via trusted publishing.

## Notes

- `.gitignore` excludes `*.py`, `*.sh`, and `*.yml` files — test scripts and examples are local-only
- No test suite in the repository; testing requires a running Kafka broker
- The `crate-type` is `cdylib` (C-compatible dynamic library for Python loading)

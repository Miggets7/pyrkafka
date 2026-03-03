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
- **`src/consumer.rs`** — `PyrKafkaConsumer` wrapping rdkafka's `BaseConsumer`. Implements Python iterator protocol (`__iter__`/`__next__`). Uses `py.detach()` to release the GIL during Kafka polling so other Python threads can run. Uses `Arc<Mutex<PyrKafkaConsumerState>>` for thread-safe stop signaling. Validates broker connectivity at construction via `fetch_metadata`. Consumer defaults: `auto.offset.reset=earliest`, `allow.auto.create.topics=true` (overridable via config dict).

### Python API

```python
PyrKafkaProducer(broker: str, config: dict[str, str] | None = None)
PyrKafkaConsumer(broker: str, topic: str, group_id: str, config: dict[str, str] | None = None)
```

Both accept an optional `config` dict for [librdkafka configuration](https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md) overrides.

## Key Dependencies

- **Rust edition 2024**, `cdylib` crate type
- **PyO3 0.28** with ABI3 stable API (Python 3.13+), using the Bound API and `IntoPyObject`
- **rdkafka 0.39** with features: `cmake-build`, `ssl-vendored`, `libz-static`, `zstd`
- **Maturin** as the Python build backend

## CI/CD

`.github/workflows/release.yaml` builds wheels on version tags matching `X.Y.Z`, `X.Y.ZaN`, `X.Y.ZbN`, `X.Y.ZrcN`. Builds for Linux (x86_64, aarch64 via manylinux_2_28), macOS (x86_64, aarch64), and Windows (x64), then publishes to PyPI via trusted publishing.

Linux builds include a workaround for librdkafka 2.12.1 bug (confluentinc/librdkafka#5204): a stub `curl/curl.h` header is injected via `C_INCLUDE_PATH` because `rdkafka_conf.c` unconditionally includes it despite curl not being used.

## Notes

- `.gitignore` excludes `*.py`, `*.sh`, and `*.yml` files — test scripts and examples are local-only
- No test suite in the repository; testing requires a running Kafka broker

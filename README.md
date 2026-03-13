# pyrkafka: Fast Python Kafka Client Powered by Rust

[![PyPI version](https://badge.fury.io/py/pyrkafka.svg)](https://pypi.org/project/pyrkafka/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

`pyrkafka` is a fast, lightweight Python client for [Apache Kafka](https://kafka.apache.org/), built on top of [librdkafka](https://github.com/confluentinc/librdkafka) via the Rust [rdkafka](https://docs.rs/rdkafka) crate. It provides a simple, Pythonic API for producing and consuming Kafka messages with native Rust performance through [PyO3](https://pyo3.rs).

## Features

- **Rust-powered performance** — wraps librdkafka through Rust, avoiding the overhead of pure-Python implementations
- **GIL release during polling** — consumer polling releases the Python GIL, allowing other threads to run while waiting for messages
- **Simple Pythonic API** — producer and consumer with familiar Python patterns (`for` and `async for` for consuming)
- **Full librdkafka configuration** — pass any [librdkafka config option](https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md) via a dict
- **SSL and compression built-in** — compiled with SSL (vendored OpenSSL) and zstd compression support
- **Cross-platform wheels** — pre-built for Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon), and Windows

## Installation

```bash
pip install pyrkafka
```

Requires Python 3.13+. Pre-built wheels are available for all major platforms — no Rust toolchain needed for installation.

## Getting Started

### Producing Messages

```python
from pyrkafka import PyrKafkaProducer

producer = PyrKafkaProducer("localhost:9092")

# Send a message (partitioned round-robin)
producer.produce("my_topic", b"Hello, Kafka!")

# Send with a key (messages with the same key go to the same partition)
producer.produce_with_key("my_topic", b"Hello, Kafka!", "my_key")

# Flush pending messages (also happens automatically when the producer is dropped)
producer.flush()
```

### Consuming Messages

```python
from pyrkafka import PyrKafkaConsumer

consumer = PyrKafkaConsumer("localhost:9092", "my_topic", "my_group")

for message in consumer:
    print(message.decode())
```

### Async Consuming

The consumer supports `async for` for use in asyncio applications. The blocking Kafka poll is automatically offloaded to a thread so the event loop stays responsive:

```python
import asyncio
from pyrkafka import PyrKafkaConsumer

async def main():
    consumer = PyrKafkaConsumer("localhost:9092", "my_topic", "my_group")
    async for message in consumer:
        print(message.decode())

asyncio.run(main())
```

### Custom Configuration

Both producer and consumer accept an optional `config` dict for additional [librdkafka configuration](https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md):

```python
producer = PyrKafkaProducer("localhost:9092", config={
    "message.timeout.ms": "5000",
    "compression.type": "zstd",
})

consumer = PyrKafkaConsumer("localhost:9092", "my_topic", "my_group", config={
    "auto.offset.reset": "latest",
    "enable.auto.commit": "false",
})
```

## License

MIT

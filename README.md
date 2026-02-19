# pyrkafka: Python Wrapper for rdkafka (Rust)

[![PyPI version](https://badge.fury.io/py/pyrkafka.svg)](https://pypi.org/project/pyrkafka/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Python Library for Apache Kafka

`pyrkafka` is a Python wrapper around the `rdkafka` Rust library. It provides a high-level Pythonic API for producing and consuming Kafka messages, leveraging the performance and safety of Rust via [PyO3](https://pyo3.rs).

## Installation

```bash
pip install pyrkafka
```

Requires Python 3.13+.

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

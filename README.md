# rdfkafka

[![PyPI version](https://badge.fury.io/py/pyrkafka.svg)](https://pypi.org/project/pyrkafka/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Description

`rdfkafka` is a Rust library for working with Kafka in Python. It provides a high-level interface for producing and consuming RDF data using the Kafka messaging system.

## Features

- Easy integration with Kafka for RDF data processing.
- High-level API for producing and consuming RDF messages.
- Support for various serialization formats (e.g., JSON, Avro, Protobuf).
- Efficient and scalable processing of RDF data.

## Installation

You can install `rdfkafka` using pip:
```
pip install rdfkafka
```

## Producer Example
```python
from rdfkafka import KafkaProducer


producer = KafkaProducer(bootstrap_servers='localhost:9092')

topic = 'my_topic'
message = 'Hello, Kafka!'
producer.send(topic, message.encode())

```

##### Note: producer gets closed when dropped

## Consumer Example
```python
from rdfkafka import KafkaConsumer

consumer = KafkaConsumer(bootstrap_servers='localhost:9092')
topic = 'my_topic'


consumer.subscribe(topic)

for message in consumer:
    print(message.value.decode())
```
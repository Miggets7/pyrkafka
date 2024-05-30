# pyrkakfa: Python Wrapper for rdfkafka (Rust)

[![PyPI version](https://badge.fury.io/py/pyrkafka.svg)](https://pypi.org/project/pyrkafka/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Python Library for Apache Kafka and RDF Data Processing

`pyrkafka` is a Python wrapper around the `rdfkafka` library, which is written in Rust. It simplifies the integration with Apache Kafka for processing RDF (Resource Description Framework) data. `pyrkafka` provides a high-level Pythonic API for producing and consuming RDF messages, supporting various serialization formats such as JSON, Avro, and Protobuf. With `pyrkafka`, you can efficiently and scalably process RDF data in your Python applications, leveraging the performance and safety of Rust.

## Key Features of pyrkafka

- **Pythonic Interface to rdfkafka**: `pyrkafka` provides a Pythonic interface to the `rdfkafka` Rust library, making it easy to integrate Apache Kafka with Python applications.
- **High-Level API for RDF Messages**: `pyrkafka` offers a high-level API for producing and consuming RDF messages in Python.
- **Support for Various Serialization Formats**: `pyrkafka` supports various serialization formats, including JSON, Avro, and Protobuf.
- **Efficient RDF Data Processing**: `pyrkafka` enables efficient and scalable processing of RDF data, leveraging the performance of Rust.

## Installation

You can install `pyrkakfa` using pip:
```bash
pip install pyrkakfa
```

# Getting Started with pyrkafka

## Sending Messages to Kafka with pyrkafka
Here's a simple example of how to send a message to a Kafka topic using `pyrkafka`:

```python
from pyrkakfa import KafkaProducer

# Initialize a producer
producer = KafkaProducer(bootstrap_servers='localhost:9092')

# Specify the topic
topic = 'my_topic'

# Create a message
message = 'Hello, Kafka!'

# Send the message
producer.send(topic, message.encode())
```

## Consuming Messages from Kafka with pyrkafka
Here's how to consume messages from a Kafka topic using `pyrkafka`:
```python
from pyrkakfa import KafkaConsumer

# Initialize a consumer
consumer = KafkaConsumer(bootstrap_servers='localhost:9092')

# Specify the topic
topic = 'my_topic'

# Subscribe to the topic
consumer.subscribe(topic)

# Listen for messages
for message in consumer:
    print(message.value.decode())
```
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use pyo3::prelude::*;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::{ClientConfig, Message};

#[derive(PartialEq)]
enum PyrKafkaConsumerState {
    Polling,
    Stopped,
}

#[pyclass]
pub struct PyrKafkaConsumer {
    consumer: BaseConsumer,
    state: Arc<Mutex<PyrKafkaConsumerState>>,
}

#[pymethods]
impl PyrKafkaConsumer {
    #[new]
    #[pyo3(signature = (broker, topic, group_id, config=None))]
    fn new(
        broker: &str,
        topic: &str,
        group_id: &str,
        config: Option<HashMap<String, String>>,
    ) -> PyResult<Self> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("group.id", group_id)
            .set("bootstrap.servers", broker)
            .set("auto.offset.reset", "earliest")
            .set("allow.auto.create.topics", "true");

        if let Some(config) = config {
            for (key, value) in config {
                client_config.set(&key, &value);
            }
        }

        let consumer: BaseConsumer = client_config.create().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyConnectionError, _>(format!(
                "Failed to create consumer: {}",
                e
            ))
        })?;

        consumer.subscribe(&[topic]).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to subscribe to topic: {}",
                e
            ))
        })?;

        consumer
            .fetch_metadata(Some(topic), Duration::from_secs(1))
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyConnectionError, _>(format!(
                    "Failed to fetch metadata for topic: {}",
                    e
                ))
            })?;

        Ok(PyrKafkaConsumer {
            consumer,
            state: Arc::new(Mutex::new(PyrKafkaConsumerState::Polling)),
        })
    }

    fn stop(&self) -> PyResult<()> {
        let mut state = self.state.lock().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Consumer state lock poisoned")
        })?;
        *state = PyrKafkaConsumerState::Stopped;
        Ok(())
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Vec<u8>>> {
        loop {
            {
                let state = self.state.lock().map_err(|_| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        "Consumer state lock poisoned",
                    )
                })?;
                if *state == PyrKafkaConsumerState::Stopped {
                    return Ok(None);
                }
            }

            // Release the GIL during the blocking Kafka poll so other Python
            // threads can run while we wait for messages.
            let consumer = &self.consumer;
            let poll_result = py.detach(move || {
                consumer.poll(Duration::from_secs(1)).map(|res| {
                    res.map(|m| m.payload().map(|p| p.to_vec()))
                        .map_err(|e| format!("{e}"))
                })
            });

            match poll_result {
                None => continue,
                Some(Ok(Some(payload))) => return Ok(Some(payload)),
                Some(Ok(None)) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                        "Received message with empty payload",
                    ));
                }
                Some(Err(e)) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyException, _>(e));
                }
            }
        }
    }
}

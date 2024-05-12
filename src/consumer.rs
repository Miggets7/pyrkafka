use std::{sync::{Arc, Mutex}, time::Duration};

use pyo3::prelude::*;
use rdkafka::{consumer::{BaseConsumer, Consumer}, ClientConfig, Message};

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
    fn new(broker: &str, topic: &str, group_id: &str) -> PyResult<Self> {
        let consumer: BaseConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", broker)
            .set("auto.offset.reset", "earliest")
            .set("allow.auto.create.topics", "true")
            .create()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
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

        consumer.fetch_metadata(Some(topic), Duration::from_secs(1)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
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
        let mut state = self.state.lock().unwrap();
        *state = PyrKafkaConsumerState::Stopped;
        Ok(())
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python) -> PyResult<PyObject> {
        let state = self.state.clone();
        loop {
            let run = state.lock().unwrap();
            if *run == PyrKafkaConsumerState::Stopped {
                // Signal to Python to stop the iteration
                break Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(
                    "No more messages",
                ));
            }
            drop(run); // Release the lock before polling

            match self.consumer.poll(Duration::from_secs(1)) {
                None => continue,
                Some(Err(e)) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                        "Error polling: {}",
                        e
                    )))
                }
                Some(Ok(m)) => {
                    if let Some(payload) = m.payload() {
                        return Ok(payload.into_py(py).to_object(py));
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                            "Received message with empty payload",
                        ));
                    }
                }
            }
        }
    }
}
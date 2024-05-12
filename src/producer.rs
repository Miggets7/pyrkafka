use std::time::Duration;

use pyo3::prelude::*;
use rdkafka::{producer::{BaseProducer, BaseRecord, Producer}, ClientConfig};

#[pyclass]
pub struct PyrKafkaProducer {
    producer: BaseProducer,
}

#[pymethods]
impl PyrKafkaProducer {
    #[new]
    fn new(broker: &str) -> PyResult<Self> {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .create()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Failed to create producer: {}",
                    e
                ))
            })?;

        Ok(PyrKafkaProducer { producer })
    }

    fn produce(&self, topic: &str, message: &[u8]) -> PyResult<()> {
        self.produce_with_key(topic, message, "key")
    }

    fn produce_with_key(&self, topic: &str, message: &[u8], key: &str) -> PyResult<()> {
        self.producer
            .send(BaseRecord::to(topic).payload(message).key(key))
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Failed to send message: {:?}",
                    e
                ))
            })?;

        Ok(())
    }

    fn flush(&self) -> PyResult<()> {
        self.producer.flush(Duration::from_secs(10)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to flush producer: {:?}",
                e
            ))
        })?;
        Ok(())
    }
}

impl Drop for PyrKafkaProducer {
    fn drop(&mut self) {
        let _ = self.producer.flush(Duration::from_secs(10));
    }
}
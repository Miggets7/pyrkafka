use std::collections::HashMap;
use std::time::Duration;

use pyo3::prelude::*;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use rdkafka::ClientConfig;

#[pyclass]
pub struct PyrKafkaProducer {
    producer: BaseProducer,
}

#[pymethods]
impl PyrKafkaProducer {
    #[new]
    #[pyo3(signature = (broker, config=None))]
    fn new(broker: &str, config: Option<HashMap<String, String>>) -> PyResult<Self> {
        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", broker);

        if let Some(config) = config {
            for (key, value) in config {
                client_config.set(&key, &value);
            }
        }

        let producer: BaseProducer = client_config.create().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyConnectionError, _>(format!(
                "Failed to create producer: {}",
                e
            ))
        })?;

        Ok(PyrKafkaProducer { producer })
    }

    fn produce(&self, topic: &str, message: &[u8]) -> PyResult<()> {
        self.producer
            .send(BaseRecord::<str, [u8]>::to(topic).payload(message))
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Failed to send message: {:?}",
                    e
                ))
            })?;
        Ok(())
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

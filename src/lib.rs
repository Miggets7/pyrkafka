
use pyo3::prelude::*;

use crate::consumer::PyrKafkaConsumer;
use crate::producer::PyrKafkaProducer;

mod consumer;
mod producer;

#[pymodule]
fn pyrkafka(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyrKafkaConsumer>()?;
    m.add_class::<PyrKafkaProducer>()?;

    Ok(())
}

use pyo3::prelude::*;

mod consumer;
mod producer;

#[pymodule]
fn pyrkafka(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<consumer::PyrKafkaConsumer>()?;
    m.add_class::<producer::PyrKafkaProducer>()?;
    Ok(())
}

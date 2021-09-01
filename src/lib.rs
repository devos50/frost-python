use pyo3::prelude::*;

extern crate frost_dalek;

pub mod participant;
pub mod parameters;
//pub mod distributedkeygeneration;

pub use participant::Participant;
pub use parameters::Parameters;
//pub use distributedkeygeneration::DistributedKeyGeneration;

#[pyfunction]
fn test_list(participant_list: Vec<Participant>) -> Vec<i32> {
    Vec::new()
}

#[pymodule]
fn frost(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Parameters>()?;
    m.add_class::<Participant>()?;
    //m.add_class::<DistributedKeyGeneration>()?;
    Ok(())
}

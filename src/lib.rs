use pyo3::prelude::*;

extern crate frost_dalek;
extern crate curve25519_dalek;

pub mod participant;
pub mod parameters;
pub mod distributedkeygeneration;

pub use participant::Participant;
pub use parameters::Parameters;
pub use distributedkeygeneration::DistributedKeyGenerationR1;

#[pyfunction]
fn test_list(participants: Vec<PyRef<Participant>>) -> Participant {
    let a = participants.get(0).unwrap();
    Participant { participant: a.participant.clone(), coefficients: a.coefficients.clone() }
}

#[pymodule]
fn frost(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Parameters>()?;
    m.add_class::<Participant>()?;
    m.add_class::<DistributedKeyGenerationR1>()?;

    m.add_wrapped(wrap_pyfunction!(test_list)).unwrap();

    Ok(())
}

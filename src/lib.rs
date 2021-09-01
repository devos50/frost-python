use pyo3::prelude::*;
use rand::rngs::OsRng;

extern crate frost_dalek;
extern crate curve25519_dalek;

pub mod participant;
pub mod parameters;
pub mod distributedkeygeneration;

pub use participant::Participant;
pub use parameters::Parameters;
pub use distributedkeygeneration::DistributedKeyGenerationR1;

use frost_dalek::precomputation::generate_commitment_share_lists as gcsl;

#[pyfunction]
fn generate_commitment_share_lists(participant_index: u32, number_of_shares: usize) -> u32 {
    let (public_comshares, mut secret_comshares) = gcsl(&mut OsRng, participant_index, number_of_shares);
    42
}

#[pymodule]
fn frost(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Parameters>()?;
    m.add_class::<Participant>()?;
    m.add_class::<DistributedKeyGenerationR1>()?;

    m.add_wrapped(wrap_pyfunction!(generate_commitment_share_lists)).unwrap();

    Ok(())
}

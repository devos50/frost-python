use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::parameters::Parameters;

#[pyclass]
#[derive(Clone)]
pub struct Participant {
    pub participant: frost_dalek::keygen::Participant,
    pub coefficients: frost_dalek::keygen::Coefficients,
}

#[pymethods]
impl Participant {
    #[new]
    pub fn new(parameters: &Parameters, index: u32) -> Self {
        let (participant, coefficients) = frost_dalek::keygen::Participant::new(&parameters.parameters, index);

        Participant {
            participant: participant,
            coefficients: coefficients,
        }
    }

    pub fn verify_proof_of_secret_key(&self) -> PyResult<()> {
        match self.participant.proof_of_secret_key.verify(&self.participant.index, &self.participant.public_key().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(PyValueError::new_err("errors")),
        }
    }
}
use pyo3::prelude::*;

#[pyclass]
pub struct Parameters {
    pub parameters: frost_dalek::Parameters,
}

#[pymethods]
impl Parameters {
    #[new]
    pub fn new(n: u32, t: u32) -> Self {
        Parameters {
            parameters: frost_dalek::Parameters { n: n, t: t, },
        }
    }
}
// use pyo3::prelude::*;
//
// use frost_dalek::keygen::RoundOne;
//
// use crate::parameters::Parameters;
// use crate::participant::Participant;
//
// #[pyclass]
// pub struct DistributedKeyGeneration {
//     distributed_key_generation: frost_dalek::keygen::DistributedKeyGeneration<RoundOne>,
// }
//
// #[pymethods]
// impl DistributedKeyGeneration {
//     #[new]
//     pub fn new(parameters: &Parameters,
//         participant: &Participant,
//         other_participants: Vec<&Participant>) -> Self {
//
//             //let participants_list: Vec<PyRef<Participant>> = other_participants.extract().unwrap();
//
//             let mut rust_participants = Vec::new();
//             for item in other_participants.iter() {
//                 let participant: Participant = item.extract().unwrap();
//                 rust_participants.push(participant.participant);
//             }
//
//             let res = frost_dalek::keygen::DistributedKeyGeneration::<RoundOne>::new(&parameters.parameters, &participant.participant.index, &participant.coefficients, &mut rust_participants).unwrap();
//
//             DistributedKeyGeneration {
//                 distributed_key_generation: res,
//             }
//     }
// }
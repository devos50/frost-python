use pyo3::prelude::*;

use curve25519_dalek::scalar::Scalar;
use frost_dalek::keygen::{RoundOne, RoundTwo, SecretShare};

use crate::parameters::Parameters;
use crate::participant::Participant;

use pyo3::types::PyBytes;

#[pyclass]
pub struct DistributedKeyGenerationR1 {
    distributed_key_generation: frost_dalek::keygen::DistributedKeyGeneration<RoundOne>,
}

#[pymethods]
impl DistributedKeyGenerationR1 {
    #[new]
    pub fn new(parameters: &Parameters,
        participant: &Participant,
        other_participants: Vec<PyRef<Participant>>) -> Self {

            let mut rust_participants = other_participants.iter().map(|i| i.participant.clone()).collect();

            let res = frost_dalek::keygen::DistributedKeyGeneration::<RoundOne>::new(&parameters.parameters, &participant.participant.index, &participant.coefficients, &mut rust_participants).unwrap();

            DistributedKeyGenerationR1 {
                distributed_key_generation: res,
            }
    }

    pub fn their_secret_shares(&self) -> Vec<(u32, [u8;32])> {
        let shares = self.distributed_key_generation.their_secret_shares().unwrap();
        let mut v = Vec::new();
        for share in shares {
            v.push((share.index, share.polynomial_evaluation.to_bytes()))
        }
        v
    }

    fn to_round_two(&self, my_index: u32, shares: Vec<&PyBytes>) -> DistributedKeyGenerationR2 {
        let mut secret_shares = Vec::new();
        for share in shares {
            let b = share.extract().unwrap();
            let scalar: Scalar = Scalar::from_canonical_bytes(b).unwrap();
            let secret_share: SecretShare = SecretShare { index: my_index, polynomial_evaluation: scalar };
            secret_shares.push(secret_share)
        }
        let next_round = self.distributed_key_generation.clone().to_round_two(secret_shares).unwrap();
        DistributedKeyGenerationR2 { distributed_key_generation: next_round }
    }
}

#[pyclass]
pub struct DistributedKeyGenerationR2 {
    distributed_key_generation: frost_dalek::keygen::DistributedKeyGeneration<RoundTwo>,
}

#[pymethods]
impl DistributedKeyGenerationR2 {

    fn finish(&self, participant: Participant) -> ([u8;32], [u8;32], [u8;32]) {
        let (group_key, secret_key) = self.distributed_key_generation.clone().finish(participant.participant.public_key().unwrap()).unwrap();
        (group_key.to_bytes(), secret_key.key.to_bytes(), secret_key.to_public().share.compress().to_bytes())
    }

}
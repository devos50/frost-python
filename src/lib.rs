use pyo3::prelude::*;
use rand::rngs::OsRng;

extern crate frost_dalek;
extern crate curve25519_dalek;

pub mod participant;
pub mod parameters;
pub mod distributedkeygeneration;
pub mod aggregator;

pub use participant::Participant;
pub use parameters::Parameters;
pub use distributedkeygeneration::DistributedKeyGenerationR1;
pub use aggregator::SignatureAggregatorInitial;

use frost_dalek::precomputation::{generate_commitment_share_lists as gcsl, Commitment, CommitmentShare, SecretCommitmentShareList};
use frost_dalek::signature::{compute_message_hash as cmh, Signer, ThresholdSignature};
use frost_dalek::keygen::SecretKey;
use curve25519_dalek::scalar::Scalar;
use frost_dalek::GroupKey;
use curve25519_dalek::ristretto::CompressedRistretto;
use crate::aggregator::SignatureAggregatorFinalized;

#[pyfunction]
fn generate_commitment_share_lists(participant_index: u32, number_of_shares: usize) -> (Vec<([u8;32], [u8;32])>, Vec<(([u8;32], [u8;32]), ([u8;32], [u8;32]))>) {
    let (public_comshares, secret_comshares) = gcsl(&mut OsRng, participant_index, number_of_shares);

    // convert to list of byte vectors
    let mut py_pub_comshares = Vec::new();
    for public_comshare in public_comshares.commitments {
        py_pub_comshares.push((public_comshare.0.compress().to_bytes(), public_comshare.1.compress().to_bytes()))
    }

    let mut py_secret_comshares = Vec::new();
    for secret_comshare in secret_comshares.commitments {
        py_secret_comshares.push(((secret_comshare.binding.nonce.to_bytes(), secret_comshare.binding.sealed.compress().to_bytes()),(secret_comshare.hiding.nonce.to_bytes(), secret_comshare.hiding.sealed.compress().to_bytes())))
    }

    (py_pub_comshares, py_secret_comshares)
}

#[pyfunction]
fn compute_message_hash(context_string: &[u8], message: &[u8]) -> [u8;64] {
    cmh(context_string, message)
}

#[pyfunction]
fn sign(my_index: u32, secret_key: [u8;32], message_hash: [u8;64], group_key_bytes: [u8;32], my_secret_commitment_share_list: Vec<(([u8;32], [u8;32]), ([u8;32], [u8;32]))>, my_commitment_share_index: usize, signers_raw: Vec<(u32, ([u8;32], [u8;32]))>) -> (u32, [u8;32]) {
    let sk = SecretKey { index: my_index, key: Scalar::from_canonical_bytes(secret_key).unwrap() };
    let group_key = GroupKey::from_bytes(group_key_bytes).unwrap();

    let mut commitment_shares = Vec::new();
    for raw_commitment_share in my_secret_commitment_share_list {
        let binding = Commitment { nonce: Scalar::from_canonical_bytes(raw_commitment_share.0.0).unwrap(), sealed: CompressedRistretto(raw_commitment_share.0.1).decompress().unwrap() };
        let hiding = Commitment { nonce: Scalar::from_canonical_bytes(raw_commitment_share.1.0).unwrap(), sealed: CompressedRistretto(raw_commitment_share.1.1).decompress().unwrap() };
        let commitment_share = CommitmentShare { hiding: hiding, binding: binding };
        commitment_shares.push(commitment_share);
    }

    let mut commitment_share_list = SecretCommitmentShareList { commitments: commitment_shares };

    let mut signers = Vec::new();
    for signer_raw in signers_raw.iter() {
        let share = (CompressedRistretto(signer_raw.1.0).decompress().unwrap(), CompressedRistretto(signer_raw.1.1).decompress().unwrap());
        let signer = Signer { participant_index: signer_raw.0, published_commitment_share: share };
        signers.push(signer);
    }

    let res = sk.sign(&message_hash, &group_key, &mut commitment_share_list, my_commitment_share_index, &signers).unwrap();
    (res.index, res.z.to_bytes())
}

#[pyfunction]
fn verify(threshold_sig_raw: ([u8;32], [u8;32]), group_key_bytes: [u8;32], message_hash: [u8;64]) -> u32 {
    let threshold_sig = ThresholdSignature { R: CompressedRistretto(threshold_sig_raw.0).decompress().unwrap(), z: Scalar::from_canonical_bytes(threshold_sig_raw.1).unwrap() };
    let group_key = GroupKey::from_bytes(group_key_bytes).unwrap();
    match threshold_sig.verify(&group_key, &message_hash) {
        Ok(_) => 1,
        Err(_) => 0
    }
}

#[pymodule]
fn frost(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Parameters>()?;
    m.add_class::<Participant>()?;
    m.add_class::<DistributedKeyGenerationR1>()?;
    m.add_class::<SignatureAggregatorInitial>()?;
    m.add_class::<SignatureAggregatorFinalized>()?;

    m.add_wrapped(wrap_pyfunction!(generate_commitment_share_lists)).unwrap();
    m.add_wrapped(wrap_pyfunction!(compute_message_hash)).unwrap();
    m.add_wrapped(wrap_pyfunction!(sign)).unwrap();
    m.add_wrapped(wrap_pyfunction!(verify)).unwrap();

    Ok(())
}

use pyo3::prelude::*;
use frost_dalek::signature::{Initial, PartialThresholdSignature, Finalized};

use crate::parameters::Parameters;
use curve25519_dalek::ristretto::CompressedRistretto;
use frost_dalek::{GroupKey, IndividualPublicKey};
use curve25519_dalek::scalar::Scalar;

#[pyclass]
pub struct SignatureAggregatorInitial {
    pub aggregator: frost_dalek::signature::SignatureAggregator<Initial>,
}

#[pymethods]
impl SignatureAggregatorInitial {
    #[new]
    pub fn new(params: &Parameters, group_key_bytes: [u8;32], context: &[u8], message: &[u8]) -> Self {
        let group_key = GroupKey::from_bytes(group_key_bytes.clone()).unwrap();

        SignatureAggregatorInitial {
            aggregator: frost_dalek::signature::SignatureAggregator::new(params.parameters, group_key, context, message)
        }
    }

    pub fn include_signer(&mut self, index: u32, published_commitment_share: ([u8;32], [u8;32]), public_key: [u8;32]) {
        let shares = (CompressedRistretto(published_commitment_share.0).decompress().unwrap(), CompressedRistretto(published_commitment_share.1).decompress().unwrap());
        let ipk = IndividualPublicKey { index: index, share: CompressedRistretto(public_key).decompress().unwrap() };
        self.aggregator.include_signer(index, shares, ipk)
    }

    pub fn get_signers(&mut self) -> Vec<(u32, ([u8;32], [u8;32]))> {
        self.aggregator.get_signers().iter().map(|i| (i.participant_index, (i.published_commitment_share.0.compress().to_bytes(), i.published_commitment_share.1.compress().to_bytes()))).collect()
    }

    pub fn include_partial_signature(&mut self, partial_sig_raw: (u32, [u8;32])) {
        let partial_sig = PartialThresholdSignature { index: partial_sig_raw.0, z: Scalar::from_canonical_bytes(partial_sig_raw.1).unwrap() };
        self.aggregator.include_partial_signature(partial_sig);
    }

    pub fn finalize(&self) -> SignatureAggregatorFinalized {
        let new_agg = self.aggregator.clone().finalize().unwrap();
        SignatureAggregatorFinalized { aggregator: new_agg }
    }
}

#[pyclass]
pub struct SignatureAggregatorFinalized {
    pub aggregator: frost_dalek::signature::SignatureAggregator<Finalized>
}

#[pymethods]
impl SignatureAggregatorFinalized {
    pub fn aggregate(&mut self) -> ([u8;32], [u8;32]) {
        let threshold_sig = self.aggregator.aggregate().unwrap();
        (threshold_sig.R.compress().to_bytes(), threshold_sig.z.to_bytes())
    }
}
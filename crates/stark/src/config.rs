use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StarkConfig {
    pub traces: cairovm_verifier_air::trace::config::Config,
    pub composition: cairovm_verifier_commitment::table::config::Config,
    pub fri: cairovm_verifier_fri::config::Config,
    pub proof_of_work: cairovm_verifier_pow::config::Config,
    // Log2 of the trace domain size.
    #[serde_as(as = "UfeHex")]
    pub log_trace_domain_size: Felt,
    // Number of queries to the last component, FRI.
    #[serde_as(as = "UfeHex")]
    pub n_queries: Felt,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    #[serde_as(as = "UfeHex")]
    pub log_n_cosets: Felt,
    // Number of layers that use a verifier friendly hash in each commitment.
    #[serde_as(as = "UfeHex")]
    pub n_verifier_friendly_commitment_layers: Felt,
}

impl StarkConfig {
    pub fn validate(&self, security_bits: Felt) -> Result<(), Error> {
        self.proof_of_work.validate()?;

        assert!(
            security_bits
                <= self.n_queries * self.log_n_cosets + Felt::from(self.proof_of_work.n_bits)
        );

        // Validate traces config.
        let log_eval_domain_size = self.log_trace_domain_size + self.log_n_cosets;
        self.traces.validate(log_eval_domain_size, self.n_verifier_friendly_commitment_layers)?;

        // Validate composition config.
        self.composition
            .vector
            .validate(log_eval_domain_size, self.n_verifier_friendly_commitment_layers)?;

        // Validate Fri config.
        self.fri.validate(self.log_n_cosets, self.n_verifier_friendly_commitment_layers)?;
        Ok(())
    }
}

use cairovm_verifier_commitment::vector;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vector Error")]
    Vector(#[from] vector::config::Error),
    #[error("Fri Error")]
    Fri(#[from] cairovm_verifier_fri::config::Error),
    #[error("Pow Error")]
    Pow(#[from] cairovm_verifier_pow::config::Error),
    #[error("Trace Error")]
    Trace(#[from] cairovm_verifier_air::trace::config::Error),
}
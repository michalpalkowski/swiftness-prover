use std::convert::TryFrom;

use crate::{json_parser::ProofJSON, stark_proof::StarkProof};

mod annotations;
mod ast;
mod builtins;
mod conversion;
mod json_parser;
mod layout;
mod stark_proof;
mod utils;

extern crate clap;
extern crate num_bigint;
extern crate regex;
extern crate serde;

pub use ast::{Expr, Exprs};
use cairovm_verifier_stark::types::StarkProof as StarkProofFromVerifier;

pub struct ParseStarkProof {
    pub config: Exprs,
    pub public_input: Exprs,
    pub unsent_commitment: Exprs,
    pub witness: Exprs,
}

pub fn parse(input: String) -> anyhow::Result<StarkProofFromVerifier> {
    let proof_json = serde_json::from_str::<ProofJSON>(&input)?;
    let stark_proof = StarkProof::try_from(proof_json)?;
    let stark_proof_verifier: StarkProofFromVerifier = stark_proof.into();
    Ok(stark_proof_verifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!(
            "../../example/fibonacci_proof_generator/recursive/cairo0_example_proof.json"
        );
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let stark_proof_verifier: StarkProofFromVerifier = stark_proof.into();
        println!("{:?}", stark_proof_verifier);
    }
}

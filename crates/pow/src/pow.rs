use alloc::vec::Vec;
use blake2::Blake2s256;
use sha3::{Digest, Keccak256};

use serde::{Deserialize, Serialize};
use starknet_crypto::Felt;
use swiftness_transcript::{assure, transcript::Transcript};
use swiftness_commitment::table::decommit::{HASHER_BLAKE2S, HASHER_248_LSB};

use crate::config::Config;

const MAGIC: u64 = 0x0123456789abcded;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsentCommitment {
    pub nonce: u64,
}

impl UnsentCommitment {
    pub fn commit(&self, transcript: &mut Transcript, config: &Config) -> Result<(), Error> {
        verify_pow(transcript.digest().to_bytes_be(), config.n_bits, self.nonce)?;
        transcript.read_uint64_from_prover(self.nonce);
        Ok(())
    }
}

pub fn verify_pow(digest: [u8; 32], n_bits: u8, nonce: u64) -> Result<(), Error> {
    // Compute the initial hash.
    // Hash(0x0123456789abcded || digest   || n_bits )
    //      8 bytes            || 32 bytes || 1 byte
    // Total of 0x29 = 41 bytes.

    let mut init_data = Vec::with_capacity(41);
    init_data.extend_from_slice(&MAGIC.to_be_bytes());
    init_data.extend_from_slice(&digest);
    init_data.push(n_bits);

    let init_hash = unsafe {
        if HASHER_BLAKE2S {
            let mut hasher = Blake2s256::new();
            hasher.update(&init_data);
            hasher.finalize().to_vec()
        } else {
            let mut hasher = Keccak256::new();
            hasher.update(&init_data);
            hasher.finalize().to_vec()
        }
    };

    // Reverse the endianness of the initial hash.
    // init_hash.reverse();

    // Compute Hash(init_hash || nonce)
    //              32 bytes  || 8 bytes
    // Total of 0x28 = 40 bytes.

    let mut hash_data = Vec::with_capacity(40);
    hash_data.extend_from_slice(&init_hash);
    hash_data.extend_from_slice(&nonce.to_be_bytes());

    let final_hash = unsafe {
        if HASHER_BLAKE2S {
            let mut hasher = Blake2s256::new();
            hasher.update(&hash_data);
            hasher.finalize()
        } else {
            let mut hasher = Keccak256::new();
            hasher.update(&hash_data);
            hasher.finalize()
        }
    };

    assure!(
        Felt::from_bytes_be_slice(&final_hash.as_slice()[0..16]) < Felt::TWO.pow(128 - n_bits),
        Error::ProofOfWorkFail
    )
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[cfg_attr(feature = "std", error("proof of work verification fail"))]
    ProofOfWorkFail,
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("proof of work verification fail")]
    ProofOfWorkFail,
}

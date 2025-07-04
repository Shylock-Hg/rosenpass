//! Contains the implementations of the crypto algorithms used throughout Rosenpass.

pub mod keyed_hash;

pub use custom::incorrect_hmac_blake2b;
pub use rust_crypto::{blake2b, keyed_shake256};

pub mod custom;
pub mod rust_crypto;

#[cfg(any(
    feature = "experiment_libcrux_define_blake2",
    feature = "experiment_libcrux_define_chachapoly",
    feature = "experiment_libcrux_define_kyber",
))]
pub mod libcrux;

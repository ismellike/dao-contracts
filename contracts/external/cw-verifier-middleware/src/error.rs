
use cosmwasm_std::{StdError, OverflowError, VerificationError};
use thiserror::Error;
use secp256k1::Error as SecpError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Message expiration has passed")]
    MessageExpired,

    #[error("Message signature is invalid")]
    SignatureInvalid,
}



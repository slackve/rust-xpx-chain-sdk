// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{HashAlgorithmEnum, Uint64Dto};

/// SecretLockTransactionDto : Transaction that sends mosaics to a recipient if the proof used is revealed. If the duration is reached, the locked funds go back to the sender of the transaction.
#[derive(Serialize, Deserialize)]
pub(crate) struct SecretLockTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(rename = "max_fee")]
    max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    deadline: Uint64Dto,
    #[serde(rename = "duration")]
    duration: Uint64Dto,
    #[serde(rename = "mosaic_id")]
    mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
    #[serde(rename = "hashAlgorithm")]
    hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    secret: String,
    /// The address in hexadecimal that will receive the funds once the transaction is unlocked.
    #[serde(rename = "recipient")]
    recipient: String,
}
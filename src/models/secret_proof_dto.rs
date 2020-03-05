use crate::models::hash_lock_dto::HashAlgorithmEnum;
use crate::models::uint_64::Uint64Dto;

#[derive(Serialize, Deserialize)]
pub struct SecretProofTransactionBodyDto {
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    #[serde(rename = "proof")]
    pub proof: String,
}

impl SecretProofTransactionBodyDto {
    pub fn new(hash_algorithm: HashAlgorithmEnum, secret: String, proof: String) -> Self {
        SecretProofTransactionBodyDto {
            hash_algorithm,
            secret,
            recipient: None,
            proof,
        }
    }
}

/// SecretProofTransactionDto : Transaction that revealed a proof.
#[derive(Serialize, Deserialize)]
pub struct SecretProofTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    #[serde(rename = "proof")]
    pub proof: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedSecretProofTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    #[serde(rename = "proof")]
    pub proof: String,
}

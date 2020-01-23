/// HashAlgorithmEnum : The hash algorithm used to hash te proof: * 0 (Op_Sha3_256)  - The proof is hashed using sha3 256. * 1 (Op_Keccak_256)  - The proof is hashed using Keccak (ETH compatibility). * 2 (Op_Hash_160)  - The proof is hashed twice: first with Sha-256 and then with RIPEMD-160 (bitcoin’s OP_HASH160). * 3 (Op_Hash_256)  - The proof is hashed twice with Sha-256 (bitcoin’s OP_HASH256).
/// The hash algorithm used to hash te proof: * 0 (Op_Sha3_256)  - The proof is hashed using sha3 256. * 1 (Op_Keccak_256)  - The proof is hashed using Keccak (ETH compatibility). * 2 (Op_Hash_160)  - The proof is hashed twice: first with Sha-256 and then with RIPEMD-160 (bitcoin’s OP_HASH160). * 3 (Op_Hash_256)  - The proof is hashed twice with Sha-256 (bitcoin’s OP_HASH256).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HashAlgorithmEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HashLockTransactionBodyDto {
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::MosaicDto,
    #[serde(rename = "duration")]
    pub duration: crate::models::MosaicDto,
    /// The aggregate bonded transaction hash that has to be confirmed before unlocking the mosaics. 
    #[serde(rename = "hash")]
    pub hash: String,
}

impl HashLockTransactionBodyDto {
    pub fn new(mosaic: crate::models::MosaicDto, duration: crate::models::MosaicDto, hash: String) -> HashLockTransactionBodyDto {
        HashLockTransactionBodyDto {
            mosaic,
            duration,
            hash,
        }
    }
}

/// HashLockTransactionDto : Transaction to lock funds before sending an aggregate bonded transaction.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HashLockTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::MosaicDto,
    #[serde(rename = "duration")]
    pub duration: crate::models::MosaicDto,
    /// The aggregate bonded transaction hash that has to be confirmed before unlocking the mosaics.
    #[serde(rename = "hash")]
    pub hash: String,
}

impl HashLockTransactionDto {
    /// Transaction to lock funds before sending an aggregate bonded transaction.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic: crate::models::MosaicDto, duration: crate::models::MosaicDto, hash: String) -> HashLockTransactionDto {
        HashLockTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic,
            duration,
            hash,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedHashLockTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::MosaicDto,
    #[serde(rename = "duration")]
    pub duration: crate::models::MosaicDto,
    /// The aggregate bonded transaction hash that has to be confirmed before unlocking the mosaics.
    #[serde(rename = "hash")]
    pub hash: String,
}

impl EmbeddedHashLockTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, signature: String, mosaic: crate::models::MosaicDto, duration: crate::models::MosaicDto, hash: String) -> EmbeddedHashLockTransactionDto {
        EmbeddedHashLockTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            signature,
            mosaic,
            duration,
            hash,
        }
    }
}

/// AggregateTransactionDto : Transaction that combines multiple transactions together.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateTransactionDto {
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
    /// An array of transaction cosignatures.
    #[serde(rename = "cosignatures")]
    pub cosignatures: Vec<crate::models::CosignatureDto>,
    /// The array of transactions initiated by different accounts.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::EmbeddedTransactionInfoDto>,
}

impl AggregateTransactionDto {
    /// Transaction that combines multiple transactions together.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, cosignatures: Vec<crate::models::CosignatureDto>, transactions: Vec<crate::models::EmbeddedTransactionInfoDto>) -> AggregateTransactionDto {
        AggregateTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            cosignatures,
            transactions,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateTransactionBodyDto {
    /// An array of transaction cosignatures.
    #[serde(rename = "cosignatures")]
    pub cosignatures: Vec<crate::models::CosignatureDto>,
    /// The array of transactions initiated by different accounts.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::EmbeddedTransactionInfoDto>,
}

impl AggregateTransactionBodyDto {
    pub fn new(cosignatures: Vec<crate::models::CosignatureDto>, transactions: Vec<crate::models::EmbeddedTransactionInfoDto>) -> AggregateTransactionBodyDto {
        AggregateTransactionBodyDto {
            cosignatures,
            transactions,
        }
    }
}

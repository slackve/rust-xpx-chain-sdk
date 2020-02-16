use crate::models::Uint64;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasDto {
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "mosaic_id", skip_serializing_if = "Option::is_none")]
    pub mosaic_id: Option<Uint64>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl AliasDto {
    pub fn new(_type: u16) -> AliasDto {
        AliasDto {
            _type,
            mosaic_id: None,
            address: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressAliasTransactionBodyDto {
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
}

impl AddressAliasTransactionBodyDto {
    pub fn new(alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, address: String) -> AddressAliasTransactionBodyDto {
        AddressAliasTransactionBodyDto {
            alias_action,
            namespace_id,
            address,
        }
    }
}

/// AddressAliasTransactionDto : Transaction that attaches a namespace to an account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressAliasTransactionDto {
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
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
}

impl AddressAliasTransactionDto {
    /// Transaction that attaches a namespace to an account.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, address: String) -> AddressAliasTransactionDto {
        AddressAliasTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            alias_action,
            namespace_id,
            address,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicAliasTransactionBodyDto {
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
}

impl MosaicAliasTransactionBodyDto {
    pub fn new(alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, mosaic_id: Vec<i32>) -> MosaicAliasTransactionBodyDto {
        MosaicAliasTransactionBodyDto {
            alias_action,
            namespace_id,
            mosaic_id,
        }
    }
}

/// MosaicAliasTransactionDto : Transaction that attaches a namespace to a mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicAliasTransactionDto {
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
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
}

impl MosaicAliasTransactionDto {
    /// Transaction that attaches a namespace to a mosaic.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, mosaic_id: Vec<i32>) -> MosaicAliasTransactionDto {
        MosaicAliasTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            alias_action,
            namespace_id,
            mosaic_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedAddressAliasTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
}

impl EmbeddedAddressAliasTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, address: String) -> EmbeddedAddressAliasTransactionDto {
        EmbeddedAddressAliasTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            alias_action,
            namespace_id,
            address,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedMosaicAliasTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::alias::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
}

impl EmbeddedMosaicAliasTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, alias_action: crate::models::alias::AliasActionEnum, namespace_id: Vec<i32>, mosaic_id: Vec<i32>) -> EmbeddedMosaicAliasTransactionDto {
        EmbeddedMosaicAliasTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            alias_action,
            namespace_id,
            mosaic_id,
        }
    }
}

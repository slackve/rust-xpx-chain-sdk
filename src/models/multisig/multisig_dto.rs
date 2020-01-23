/// MultisigModificationTypeEnum : The type of the modification: * 0 - Add cosignatory. * 1 - Remove cosignatory.
/// The type of the modification: * 0 - Add cosignatory. * 1 - Remove cosignatory.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MultisigModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigDto {
    /// The account public key.
    #[serde(rename = "account")]
    pub account: String,
    /// The account address in hexadecimal.
    #[serde(rename = "accountAddress", skip_serializing_if = "Option::is_none")]
    pub account_address: Option<String>,
    /// The number of signatures needed to approve a transaction.
    #[serde(rename = "minApproval")]
    pub min_approval: i32,
    /// The number of signatures needed to remove a cosignatory.
    #[serde(rename = "minRemoval")]
    pub min_removal: i32,
    /// The array of public keys of the cosignatory accounts.
    #[serde(rename = "cosignatories")]
    pub cosignatories: Vec<String>,
    /// The array of multisig accounts where the account is cosignatory.
    #[serde(rename = "multisigAccounts")]
    pub multisig_accounts: Vec<String>,
}

impl MultisigDto {
    pub fn new(account: String, min_approval: i32, min_removal: i32, cosignatories: Vec<String>, multisig_accounts: Vec<String>) -> MultisigDto {
        MultisigDto {
            account,
            account_address: None,
            min_approval,
            min_removal,
            cosignatories,
            multisig_accounts,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigAccountGraphInfoDto {
    /// The level of the multisig account.
    #[serde(rename = "level")]
    pub level: i32,
    /// The array of multisig accounts for this level.
    #[serde(rename = "multisigEntries")]
    pub multisig_entries: Vec<crate::models::MultisigAccountInfoDto>,
}

impl MultisigAccountGraphInfoDto {
    pub fn new(level: i32, multisig_entries: Vec<crate::models::MultisigAccountInfoDto>) -> MultisigAccountGraphInfoDto {
        MultisigAccountGraphInfoDto {
            level,
            multisig_entries,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigAccountInfoDto {
    #[serde(rename = "multisig")]
    pub multisig: crate::models::MultisigDto,
}

impl MultisigAccountInfoDto {
    pub fn new(multisig: crate::models::MultisigDto) -> MultisigAccountInfoDto {
        MultisigAccountInfoDto {
            multisig,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyMultisigAccountTransactionBodyDto {
    /// The number of signatures needed to remove a cosignatory. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minRemovalDelta")]
    pub min_removal_delta: i32,
    /// The number of signatures needed to approve a transaction. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minApprovalDelta")]
    pub min_approval_delta: i32,
    /// The array of cosignatory accounts to add or delete.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::CosignatoryModificationDto>,
}

impl ModifyMultisigAccountTransactionBodyDto {
    pub fn new(min_removal_delta: i32, min_approval_delta: i32, modifications: Vec<crate::models::CosignatoryModificationDto>) -> ModifyMultisigAccountTransactionBodyDto {
        ModifyMultisigAccountTransactionBodyDto {
            min_removal_delta,
            min_approval_delta,
            modifications,
        }
    }
}

/// ModifyMultisigAccountTransactionDto : Transaction that creates or modifies a multisig account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyMultisigAccountTransactionDto {
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
    /// The number of signatures needed to remove a cosignatory. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minRemovalDelta")]
    pub min_removal_delta: i32,
    /// The number of signatures needed to approve a transaction. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minApprovalDelta")]
    pub min_approval_delta: i32,
    /// The array of cosignatory accounts to add or delete.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::CosignatoryModificationDto>,
}

impl ModifyMultisigAccountTransactionDto {
    /// Transaction that creates or modifies a multisig account.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, min_removal_delta: i32, min_approval_delta: i32, modifications: Vec<crate::models::CosignatoryModificationDto>) -> ModifyMultisigAccountTransactionDto {
        ModifyMultisigAccountTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            min_removal_delta,
            min_approval_delta,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedModifyMultisigAccountTransactionDto {
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
    /// The number of signatures needed to remove a cosignatory. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minRemovalDelta")]
    pub min_removal_delta: i32,
    /// The number of signatures needed to approve a transaction. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories.
    #[serde(rename = "minApprovalDelta")]
    pub min_approval_delta: i32,
    /// The array of cosignatory accounts to add or delete.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::CosignatoryModificationDto>,
}

impl EmbeddedModifyMultisigAccountTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, min_removal_delta: i32, min_approval_delta: i32, modifications: Vec<crate::models::CosignatoryModificationDto>) -> EmbeddedModifyMultisigAccountTransactionDto {
        EmbeddedModifyMultisigAccountTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            min_removal_delta,
            min_approval_delta,
            modifications,
        }
    }
}

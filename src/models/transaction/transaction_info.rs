/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::models::{account::PublicAccount, network::NetworkType, uint_64::Uint64};

use super::{deadline::Deadline, AbsVector, EntityTypeEnum, EntityVersion, Hash, Height};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_info: Option<TransactionInfo>,

    pub network_type: NetworkType,

    /// The signature was generated by the signer and can be used to validate tha the entity
    /// data was not modified by a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// The 'PublicAccount' of the entity signer formatted as hexadecimal.
    pub signer: PublicAccount,

    /// The transaction version.
    pub version: EntityVersion,

    /// The transaction type.
    #[serde(rename = "type")]
    pub transaction_type: EntityTypeEnum,

    /// The maximum fee allowed to be spent for this transaction.
    ///
    /// The higher the fee, the higher the priority of the transaction. Transactions with high
    /// priority get included in a block before transactions with lower priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64>,

    /// The 'Deadline' for the transaction to be included in a block before it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Deadline>,
}

impl AbstractTransaction {
    pub(crate) fn get_hash(&self) -> Hash {
        match self.transaction_info.to_owned() {
            Some(h) => match h.hash {
                Some(hs) => hs,
                _ => "".to_string(),
            },
            _ => "".to_string(),
        }
    }

    pub fn new_from_type(
        deadline: Deadline,
        version: EntityVersion,
        transaction_type: EntityTypeEnum,
        network_type: NetworkType,
    ) -> Self {
        AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: None,
            signer: Default::default(),
            version,
            transaction_type,
            max_fee: None,
            deadline: Some(deadline),
        }
    }

    pub fn is_unconfirmed(&self) -> bool {
        if let Some(tx_info) = &self.transaction_info {
            *tx_info.height == 0 && tx_info.hash.eq(&tx_info.merkle_component_hash)
        } else {
            false
        }
    }

    pub fn is_confirmed(&self) -> bool {
        if let Some(tx_info) = &self.transaction_info {
            *tx_info.height > 0
        } else {
            false
        }
    }

    pub fn has_missing_signatures(&self) -> bool {
        if let Some(tx_info) = &self.transaction_info {
            *tx_info.height == 0 && tx_info.hash.eq(&tx_info.merkle_component_hash)
        } else {
            false
        }
    }

    pub fn is_unannounced(&self) -> bool {
        if let Some(tx_info) = &self.transaction_info {
            tx_info.hash.is_some() || tx_info.agregate_hash.is_some()
        } else {
            false
        }
    }

    pub(crate) fn set_aggregate(&mut self, signer: PublicAccount) {
        self.signer = signer;
    }

    pub(crate) fn build_vector<'a>(
        &self,
        builder: &mut fb::FlatBufferBuilder<'a>,
    ) -> AbsVector<'a> {
        AbsVector::build_vector(self, builder)
    }
}

impl core::fmt::Display for AbstractTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub height: Height,
    pub index: u32,
    pub id: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_component_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agregate_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_aggregate_hash: Option<String>,
}

impl TransactionInfo {
    pub fn transaction_hash(&self) -> Hash {
        match self.hash.to_owned() {
            Some(h) => h,
            None => String::new(),
        }
    }
}

impl core::fmt::Display for TransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

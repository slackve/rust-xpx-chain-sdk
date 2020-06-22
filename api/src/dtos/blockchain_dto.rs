// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use sdk::blockchain::{BlockchainScore, HeightInfo};

use super::{Uint64Dto, UpgradeDto};

#[derive(Serialize, Deserialize)]
pub(crate) struct HeightInfoDto {
    #[serde(rename = "height")]
    height: Uint64Dto,
}

impl HeightInfoDto {
    pub fn compact(&self) -> HeightInfo {
        HeightInfo {
            height: self.height.compact(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainScoreDto {
    #[serde(rename = "scoreHigh")]
    score_high: Uint64Dto,
    #[serde(rename = "scoreLow")]
    score_low: Uint64Dto,
}

impl BlockchainScoreDto {
    pub fn compact(&self) -> BlockchainScore {
        BlockchainScore {
            score_high: self.score_high.compact(),
            score_low: self.score_low.compact(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainUpgradeBodyDto {
    #[serde(rename = "upgradePeriod")]
    upgrade_period: Uint64Dto,
    #[serde(rename = "newBlockChainVersion")]
    new_block_chain_version: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainUpgradeDto {
    #[serde(rename = "blockchainUpgrade")]
    blockchain_upgrade: UpgradeDto,
}

/// BlockchainUpgradeTransactionDto : Transaction that change version of blockchain.
#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainUpgradeTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(rename = "max_fee")]
    max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    deadline: Uint64Dto,
    #[serde(rename = "upgradePeriod")]
    upgrade_period: Uint64Dto,
    #[serde(rename = "newBlockChainVersion")]
    new_block_chain_version: Uint64Dto,
}
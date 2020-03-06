use crate::models::{
    account::PublicAccount,
    network::NetworkType,
    transaction::{EntityVersion, Hash, Height, Timestamp},
    Uint64
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    /// The network type.
    pub network_type: NetworkType,
    /// The signature of the entity. The signature was generated by
    /// the signer and can be used to validate tha the entity data was not modified by a node.
    pub signature: String,
    /// The 'PublicAccount' of the entity signer formatted as hexadecimal.
    pub signer: PublicAccount,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0xc8 (PRIVATE) - PRIVATE network.
    /// * 0xb0 (PRIVATE_TEST) - PRIVATE_TEST network.
    /// * 0xb8 (PUBLIC) - PUBLIC network.
    /// * 0xa8 (PUBLIC_TEST) - PUBLIC_TEST network.
    /// * 0x60 (MIJIN) - MIJIN network.
    /// * 0x90 (MIJIN_TEST) - MIJIN_TEST test network.
    /// The transaction version.
    pub version: EntityVersion,
    /// The transaction type.
    #[serde(rename = "type")]
    pub ver_type: u16,
    /// The height of which this block was confirmed.
    ///
    /// Each block has a unique height. Subsequent blocks differ in height by one.
    pub height: Height,
    /// The date elapsed since the creation of the nemesis block.
    pub timestamp: Timestamp,
    /// The Proof-of-Importance difficulty to harvest a block.
    pub difficulty: Uint64,
    /// The number of transactions included in this block.
    pub num_transactions: u64,
    /// The fee multiplier applied to transactions contained in block.
    pub fee_multiplier: i32,
    /// The generation hash.
    pub generation_hash: Hash,
    /// The hash of the previous block.
    pub previous_block_hash: Hash,
    /// The transactions included in a block are hashed forming a merkle tree.
    /// The root of the tree summarizes them.
    pub block_transactions_hash: Hash,
    /// The collection of receipts  are hashed into a merkle tree and linked to a block.
    /// The block header stores the root hash.
    pub block_receipts_hash: Hash,
    /// For each block, the state of the blockchain is stored in RocksDB,
    /// forming a patricia tree. The root of the tree summarizes the state of
    /// the blockchain for the given block.
    pub state_hash: Hash,
    /// The 'PublicAccount' of the optional beneficiary designated by harvester.
    pub beneficiary: Option<PublicAccount>,
    /// The part of the transaction fee harvester is willing to get.
    /// From 0 up to FeeInterestDenominator.
    /// The customer gets (FeeInterest / FeeInterestDenominator)'th part of
    /// the maximum transaction fee.
    pub fee_interest: u32,
    /// The sum of all transaction fees included in this block.
    pub total_fee: Uint64,
    /// Denominator of the transaction fee.
    pub fee_interest_denominator: u32,
}

impl BlockInfo {
    pub fn new(network_type: NetworkType, signature: String, signer: PublicAccount,
               version: EntityVersion, ver_type: u16, height: Height, timestamp: Timestamp,
               difficulty: Uint64, num_transactions: u64, fee_multiplier: i32,
               generation_hash: Hash, previous_block_hash: Hash, block_transactions_hash: Hash,
               block_receipts_hash: Hash, state_hash: String, beneficiary: Option<PublicAccount>,
               fee_interest: u32, total_fee: Uint64, fee_interest_denominator: u32,
    ) -> Self {
        BlockInfo {
            network_type,
            signature,
            signer,
            version,
            ver_type,
            height,
            timestamp,
            difficulty,
            num_transactions,
            fee_multiplier,
            generation_hash,
            previous_block_hash,
            block_transactions_hash,
            block_receipts_hash,
            state_hash,
            beneficiary,
            fee_interest,
            total_fee,
            fee_interest_denominator,
        }
    }
}

impl<'a> core::fmt::Display for BlockInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

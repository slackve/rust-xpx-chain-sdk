use sha3::{Digest, Sha3_256};

use crate::models::{{account::PublicAccount},
                    namespace::NAMESPACE_BIT,
                    Uint64,
                    utils::array_u8_to_u64,
};

use super::MosaicNonce;

static GET_SUPPLY_MUTABLE: u8 = 0x01;

static GET_TRANSFERABLE: u8 = 0x02;

pub(crate) static XPX_DIVISIBILITY: u64 = 1000000;

pub(crate) static XPX_MAX_VALUE: u64 = XPX_MAX_RELATIVE_VALUE * XPX_DIVISIBILITY;

pub(crate) static XPX_MAX_RELATIVE_VALUE: u64 = 9000000000;

/// MosaicPropertyId :
/// The mosaic propery id means:
/// * 0 - MosaicFlags
/// * 1 - Divisibility
/// * 2 - Duration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MosaicPropertyId {
    MosaicFlags,
    Divisibility,
    Duration,
}

/// MosaicSupplyType :
/// The supply modification direction:
/// * 0  - Decrease.
/// * 1  - Increase.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum MosaicSupplyType { Decrease, Increase }

pub(crate) fn has_bits(number: Uint64, bits: u8) -> bool {
    (number.0 & bits as u64) == bits as u64
}

pub(crate) fn generate_mosaic_id(nonce: MosaicNonce, owner_public_id: PublicAccount) -> Uint64 {
    let mut hash = Sha3_256::default();

    let nonce_bytes = nonce.to_array();

    hash.input(nonce_bytes);

    let owner_bytes: [u8; 32] = owner_public_id.to_array();

    hash.input(owner_bytes);

    let hash_to_array = hash.result();

    Uint64(array_u8_to_u64(&mut hash_to_array.as_slice()) ^ NAMESPACE_BIT)
}


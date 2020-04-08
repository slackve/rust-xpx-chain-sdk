use std::fmt::{Debug, Write};

use hyper::body::Bytes;
use serde_json::Value;
use utils::is_hex;

use sdk::{
    errors,
    mosaic::{MosaicProperties, SUPPLY_MUTABLE, TRANSFERABLE},
    multisig::CosignatoryModification,
    network::NetworkType,
    transaction::EntityTypeEnum as Entity,
    Result, Uint64,
};

use crate::dtos::{CosignatoryModificationDto, MosaicPropertyDto, TransactionDto};

static TRANSACTION_ORDER_ASC: &str = "id";

static TRANSACTION_ORDER_DESC: &str = "-id";

pub(crate) struct AccountTransactionsOption {
    pub page_size: Option<i32>,
    pub id: Option<String>,
    pub ordering: Option<String>,
}

impl AccountTransactionsOption {
    pub fn new(
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> sdk::Result<Self> {
        if ordering.is_some() {
            ensure!(
                ordering.unwrap() == TRANSACTION_ORDER_ASC
                    || ordering.unwrap() == TRANSACTION_ORDER_DESC,
                "Invalid value { } ordering",
                ordering.unwrap()
            );
        }

        Ok(Self {
            page_size,
            id: match id {
                Some(i) => Some(i.to_string()),
                _ => None,
            },
            ordering: match ordering {
                Some(i) => Some(i.to_string()),
                _ => None,
            },
        })
    }
}

pub(crate) fn valid_account_id(id: &str) -> Result<bool> {
    ensure!(!id.is_empty(), errors::ERR_EMPTY_ADDRESSES_ID);

    Ok(true)
}

pub(crate) fn valid_hash(hash: &str) -> Result<bool> {
    ensure!(!hash.is_empty(), errors::ERR_INVALID_HASH_HEX);

    ensure!(is_hex(hash), "{} {}.", errors::ERR_INVALID_HASH_HEX, hash);

    ensure!(
        hash.len() == 64,
        "{} {}.",
        errors::ERR_INVALID_HASH_LENGTH,
        hash
    );

    Ok(true)
}

pub(crate) fn valid_vec_len<T>(vector: &Vec<T>, msg: &str) -> Result<()>
where
    T: Debug,
{
    ensure!(!vector.is_empty(), "{}. {:?}", msg, vector);
    Ok(())
}

pub(crate) fn valid_vec_hash(vector: &Vec<&str>) -> Result<()> {
    for hash in vector {
        valid_hash(hash)?;
    }
    Ok(())
}

pub(crate) fn map_transaction_dto_vec(body: Bytes) -> Result<String> {
    let value_dto_vec: Value = serde_json::from_slice(&body)?;

    let mut value_dto_vec_str: String = "".to_string();
    value_dto_vec_str.write_char('[')?;
    for dto in 0..value_dto_vec.as_array().unwrap().len() {
        let to_array = &value_dto_vec.as_array().unwrap()[dto];

        let to_string = format!("{}", to_array);

        let transaction_dto = map_transaction_dto(Bytes::from(to_string))?;

        value_dto_vec_str.push_str(&serde_json::to_string(&transaction_dto)?);

        if value_dto_vec.as_array().unwrap().len() != dto + 1 {
            value_dto_vec_str.write_char(',')?;
        }
    }

    value_dto_vec_str.write_char(']')?;

    let value_dto_vec_str = value_dto_vec_str.replace(&['\\'][..], "");
    let value_dto_vec_str = value_dto_vec_str.replace(r#"["{"#, r#"[{"#);
    let value_dto_vec_str = value_dto_vec_str.replace(r#"}","{"#, r#"},{"#);
    let value_dto_vec_str = value_dto_vec_str.replace(r#"}}}"]"#, r#"}}}]"#);

    Ok(value_dto_vec_str)
}

pub(crate) fn map_transaction_dto(body: Bytes) -> Result<String> {
    let value_dto: Value = serde_json::from_slice(&body)?;

    let entity_type = Entity::from(value_dto["transaction"]["type"].as_u64().unwrap() as u16);

    let entity_dto = match entity_type {
        Entity::AccountLink => "AccountLink",
        Entity::AccountRestrictionAddress => "AccountRestrictionAddress",
        Entity::AccountRestrictionMosaic => "AccountRestrictionMosaic",
        Entity::AccountRestrictionOperation => "AccountRestrictionOperation",
        Entity::AddressAlias => "AddressAlias",
        Entity::AggregateBonded => "Aggregate",
        Entity::AggregateComplete => "Aggregate",
        Entity::Block => "Block",
        Entity::BlockchainUpgrade => "BlockchainUpgrade",
        Entity::Lock => "HashLock",
        Entity::ModifyMultisigAccount => "ModifyMultisigAccount",
        Entity::MosaicAlias => "MosaicAlias",
        Entity::MosaicDefinition => "MosaicDefinition",
        Entity::MosaicSupplyChange => "MosaicSupplyChange",
        Entity::NamespaceRegistration => "RegisterNamespace",
        Entity::NemesisBlock => "NemesisBlock:",
        Entity::NetworkConfigEntityType => "NetworkConfigEntityType",
        Entity::SecretLock => "SecretLock",
        Entity::SecretProof => "SecretProof",
        Entity::Transfer => "Transfer",
        _ => errors::ERR_UNKNOWN_BLOCKCHAIN_TYPE,
    };

    let info_dto = format!("{{\"{}TransactionInfoDto\":{{\"meta\":", entity_dto);

    Ok(format!("{}", value_dto)
        .replace(r#"{"meta":"#, &info_dto)
        .replace("}}", r#"}}}"#))
}

pub fn map_aggregate_transactions_dto(
    transactions: Vec<Value>,
) -> Result<Vec<Box<dyn TransactionDto>>> {
    let mut txs_dto: Vec<Box<dyn TransactionDto>> = vec![];
    for item in transactions.into_iter() {
        let body: Bytes = Bytes::from(item["AggregateTransactionInfoDto"].to_string());
        let map_dto = map_transaction_dto(body)?;
        txs_dto.push(serde_json::from_str(&map_dto)?);
    }

    Ok(txs_dto)
}

pub(crate) fn mosaic_properties(dto: &Vec<MosaicPropertyDto>) -> Result<MosaicProperties> {
    let mut flags: Uint64 = Uint64::default();
    let mut divisibility: u8 = 0;
    let mut duration: Uint64 = Uint64::default();

    for property in dto.into_iter() {
        match property.id {
            0 => flags = property.value.to_struct(),
            1 => divisibility = property.value.to_struct().to_u64() as u8,
            2 => duration = property.value.to_struct(),
            _ => bail!("Unknown Property Id"),
        }
    }

    MosaicProperties::new(
        has_bits(flags, SUPPLY_MUTABLE),
        has_bits(flags, TRANSFERABLE),
        divisibility,
        duration,
    )
}

pub fn cosignatory_dto_vec_to_struct(
    modifications: Vec<CosignatoryModificationDto>,
    network_type: NetworkType,
) -> Vec<CosignatoryModification> {
    modifications
        .into_iter()
        .map(|item| item.to_struct(network_type))
        .collect()
}

pub fn has_bits(number: Uint64, bits: u8) -> bool {
    (number.to_u64() & bits as u64) == bits as u64
}

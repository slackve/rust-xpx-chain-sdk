use ::sha3::Sha3_256;
use fb::FlatBufferBuilder;
use sha3::Digest;
use xpx_crypto::Keypair;

use crate::models::{
    account::Account,
    consts::{HALF_OF_SIGNATURE, SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE, TRANSACTION_HEADER_SIZE,
             TYPE_SIZE, VERSION_SIZE},
    errors::ERR_EMPTY_TRANSACTION_SIGNER,
    mosaic::MosaicProperty,
    multisig::{CosignatoryModification, CosignatoryModificationDto},
    network::NetworkType,
    utils::{u32_to_array_u8, vec_u8_to_hex}
};

use super::{
    AbsTransaction,
    AggregateTransaction, buffer::{
        modify_multisig_account as bm,
        mosaic_definition
    },
    EntityVersion, SignedTransaction, Transaction
};

pub(crate) fn extract_version(version: u32) -> EntityVersion {
    return version & 0xFFFFFF;
}

pub(super) fn sign_transaction(
    tx: impl Transaction,
    account: Account,
    generation_hash: String) -> crate::Result<SignedTransaction>
{
    let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

    let mut tx_bytes = tx.embedded_to_bytes()?;

    let generation_hash_bytes = hex::decode(&generation_hash)?;

    let signing_suffix = &tx_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..];

    let signing = [generation_hash_bytes.as_slice(), signing_suffix].concat();

    let signature = key_pair.sign(&signing);

    let mut tx_vector: Vec<u8> = Vec::with_capacity(tx_bytes.len());

    tx_vector.append(&mut tx_bytes[..4].to_vec());
    tx_vector.append(&mut signature.to_bytes().to_vec());
    tx_vector.append(&mut key_pair.public.to_bytes().to_vec());
    tx_vector.append(&mut tx_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..].to_vec());

    let payload = vec_u8_to_hex(tx_vector);

    let hash = create_transaction_hash(payload.clone(), &generation_hash);

    Ok(SignedTransaction::new(tx.entity_type(), payload, hash))
}

pub(super) fn sign_transaction_with_cosignatures(tx: AggregateTransaction, account: Account, cosignatories: Vec<Account>, generation_hash: String) -> crate::Result<SignedTransaction> {
    let entity_type = tx.entity_type();
    let stx = sign_transaction(tx, account, generation_hash)?;

    let mut payload = stx.to_owned().payload.unwrap();
    cosignatories.iter().for_each(|item|
        {
            let key_pair: Keypair = Keypair::from_private_key(item.to_owned().key_pair.secret);
            let hash_bytes = hex::decode(&stx.hash).unwrap();
            let signature = key_pair.sign(&hash_bytes);
            payload.push_str(&format!("{}{}", item.public_account.public_key, hex::encode(&signature.to_bytes()[..])));
        }
    );

    let mut payload_bytes = hex::decode(payload)?;

    let s = u32_to_array_u8(payload_bytes.len() as u32);

    s.iter().enumerate().for_each(|(i, item)|
        {
            payload_bytes[i] = *item;
        }
    );

    Ok(SignedTransaction::new(entity_type, hex::encode(payload_bytes), stx.hash))
}

pub(super) fn create_transaction_hash(p: String, generation_hash: &str) -> String {
    let mut p_bytes = hex::decode(p).unwrap();

    let mut sb = Vec::new();

    sb.append(&mut p_bytes[SIZE_SIZE..SIZE_SIZE + HALF_OF_SIGNATURE].to_vec());

    sb.append(&mut p_bytes[SIGNATURE_SIZE + SIZE_SIZE..SIZE_SIZE + SIGNATURE_SIZE + SIGNER_SIZE].to_vec());

    let generation_hash_bytes = hex::decode(generation_hash);

    sb.append(&mut generation_hash_bytes.unwrap().to_vec());

    sb.append(&mut p_bytes[100..].to_vec());

    let sha3_public_key_hash = Sha3_256::digest(sb.as_slice());

    vec_u8_to_hex(sha3_public_key_hash[..].to_vec())
}

pub(crate) fn mosaic_property_array_to_buffer(
    builder: &mut FlatBufferBuilder, properties: Vec<MosaicProperty>) -> fb::UOffsetT {
    let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(properties.len());

    for p in properties {
        let value_v = builder.create_vector(&p.value.to_int_array());

        let mut mosaic_property = mosaic_definition::buffers::MosaicPropertyBuilder::new(builder);
        mosaic_property.add_mosaic_property_id(p.id.value());
        mosaic_property.add_value(value_v);

        p_buffer.push(mosaic_property.finish().value());
    }

    builder.create_vector(&p_buffer).value()
}

pub(super) fn to_aggregate_transaction_bytes(tx: &Box<dyn Transaction>) -> crate::Result<Vec<u8>> {
    ensure!(
        tx.abs_transaction().signer.public_key != "",
        ERR_EMPTY_TRANSACTION_SIGNER
    );

    let mut signer_bytes = tx.to_owned().abs_transaction().signer.to_bytes();

    let mut tx_bytes = tx.embedded_to_bytes()?;

    let mut r_b: Vec<u8> = Vec::with_capacity(tx_bytes.len());
    r_b.append(&mut [0u8; 4].to_vec());
    r_b.append(&mut signer_bytes[..].to_vec());
    r_b.append(&mut tx_bytes[
        SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE
            + VERSION_SIZE + TYPE_SIZE].to_vec());

    r_b.append(&mut tx_bytes[TRANSACTION_HEADER_SIZE..].to_vec());

    let s = u32_to_array_u8(r_b.len() as u32);

    for (i, item) in s.iter().enumerate() {
        r_b[i] = *item;
    };

    Ok(r_b)
}

pub(super) fn cosignatory_modification_array_to_buffer<'a>(
    builder: &mut FlatBufferBuilder<'a>, modifications: Vec<CosignatoryModification>) -> fb::UOffsetT {
    let mut cosignatory_buffer: Vec<fb::WIPOffset<bm::buffers::CosignatoryModificationBuffer<'a>>> = Vec::with_capacity(modifications.len());

    for modification in modifications {
        let public_key = &modification.public_account;

        let public_key_vector = builder.create_vector_direct(&public_key.to_bytes());

        let mut modify_multisig = bm::buffers::CosignatoryModificationBufferBuilder::new(builder);
        modify_multisig.add_type_(modification.modification_type.value());
        modify_multisig.add_cosignatory_public_key(public_key_vector);

        cosignatory_buffer.push(modify_multisig.finish());
    }

    builder.create_vector(&cosignatory_buffer).value()
}

pub(crate) fn cosignatory_dto_vec_to_struct(modifications: Vec<CosignatoryModificationDto>, network_type: NetworkType) -> Vec<CosignatoryModification> {
    modifications.into_iter().map(|item| {
        item.to_struct(network_type)
    }
    ).collect()
}
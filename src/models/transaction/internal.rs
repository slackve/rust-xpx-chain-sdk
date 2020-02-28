use ::sha3::Sha3_256;
use fb::FlatBufferBuilder;
use sha3::Digest;
use xpx_crypto::Keypair;

use crate::models::{
    account::Account,
    consts::{HALF_OF_SIGNATURE, SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE},
    utils::vec_u8_to_hex,
};
use crate::models::mosaic::MosaicProperty;
use crate::models::transaction::buffer::mosaic_definition::buffers;

use super::{EntityVersion, SignedTransaction, Transaction};

pub fn extract_version(version: i32) -> EntityVersion {
    return version & 0xFFFFFF;
}

pub fn sign_transaction(tx: &dyn Transaction, account: Account, generation_hash: String) -> crate::Result<SignedTransaction> {
    let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

    let mut bytes = tx.generate_bytes();

    let generation_hash_bytes = hex::decode(&generation_hash);

    let signing_suffix = &bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..];

    let signing = [generation_hash_bytes.unwrap().as_slice(), signing_suffix].concat();

    let signature = key_pair.sign(&signing);

    let mut tx_vector: Vec<u8> = Vec::with_capacity(bytes.len());

    tx_vector.append(&mut bytes[..4].to_vec());
    tx_vector.append(&mut signature.to_bytes().to_vec());
    tx_vector.append(&mut key_pair.public.to_bytes().to_vec());
    tx_vector.append(&mut bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..].to_vec());

    let p_hex = vec_u8_to_hex(tx_vector);

    let hash = create_transaction_hash(p_hex.clone(), &generation_hash);

    Ok(SignedTransaction::new(tx.entity_type(), p_hex, hash))
}

pub fn create_transaction_hash(p: String, generation_hash: &str) -> String {
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

pub fn mosaic_property_array_to_buffer(
    builder: &mut FlatBufferBuilder, properties: Vec<MosaicProperty>) -> fb::UOffsetT {
    let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(properties.len());

    for p in properties {
        let value_v = builder.create_vector(&p.value.to_int_array());

        let mut mosaic_property = buffers::MosaicPropertyBuilder::new(builder);
        mosaic_property.add_mosaic_property_id(p.id);
        mosaic_property.add_value(value_v);

        p_buffer.push(mosaic_property.finish().value());
    }

    builder.create_vector(&p_buffer).value()
}
use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::{fb, models::{
    account::{Account, Address, PublicAccount},
    consts::{AMOUNT_SIZE, MOSAIC_DEFINITION_TRANSACTION_HEADER_SIZE, MOSAIC_ID_SIZE},
    message::Message,
    mosaic::Mosaic,
    network::NetworkType,
}};
use crate::models::consts::{MOSAIC_OPTIONAL_PROPERTY_SIZE, MOSAIC_PROPERTY_SIZE};
use crate::models::mosaic::{MosaicId, MosaicNonce, MosaicProperties, SUPPLY_MUTABLE, TRANSFERABLE};
use crate::models::transaction::MOSAIC_DEFINITION_VERSION;
use crate::models::utils::u32_to_array_u8;

use super::{
    AbstractTransaction,
    buffer::mosaic_definition::buffers,
    deadline::Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    SignedTransaction,
    Transaction,
    TRANSFER_VERSION,
};
use crate::models::Id;
use crate::models::transaction::internal::mosaic_property_array_to_buffer;
use crate::models::transaction::schema::mosaic_definition_transaction_schema;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicDefinitionTransaction {
    pub abs_transaction: AbstractTransaction,
    pub properties: MosaicProperties,
    pub mosaic_nonce: u32,
    pub mosaic_id: MosaicId
}

impl MosaicDefinitionTransaction {
    pub fn new(
        deadline: Deadline,
        nonce: u32,
        owner_public_account: PublicAccount,
        properties: MosaicProperties,
        network_type: NetworkType,
    ) -> crate::Result<MosaicDefinitionTransaction> {
        let abs_tx = AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: "".to_string(),
            signer: Default::default(),
            version: MOSAIC_DEFINITION_VERSION,
            transaction_type: EntityTypeEnum::MosaicDefinition,
            max_fee: Default::default(),
            deadline,
        };

        let mosaic_id = MosaicId::from_nonce_and_owner(
            MosaicNonce::new(u32_to_array_u8(nonce)),
            owner_public_account
        );

        Ok(MosaicDefinitionTransaction {
            abs_transaction: abs_tx,
            properties,
            mosaic_nonce: nonce,
            mosaic_id
        })
    }

    pub fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    pub fn is_unconfirmed(&self) -> bool {
        self.abs_transaction.is_unconfirmed()
    }

    pub fn is_confirmed(&self) -> bool {
        self.abs_transaction.is_confirmed()
    }

    pub fn has_missing_signatures(&self) -> bool {
        self.abs_transaction.has_missing_signatures()
    }
}

impl Transaction for MosaicDefinitionTransaction {
    fn get_abs_transaction(self) -> AbstractTransaction {
        self.abs_transaction
    }

    fn size(&self) -> usize {
        MOSAIC_DEFINITION_TRANSACTION_HEADER_SIZE
            + self.properties.optional_properties.len() * MOSAIC_OPTIONAL_PROPERTY_SIZE
    }

    fn generate_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let mut f: u8 = 0;
        if self.properties.supply_mutable {
            f += SUPPLY_MUTABLE;
        }

        if self.properties.transferable {
            f += TRANSFERABLE;
        }

        let mosaic_vec = builder.create_vector(&self.mosaic_id.to_int_array());
        let property_vec = mosaic_property_array_to_buffer(
            &mut builder, self.properties.clone().optional_properties
        );

        let abs_vector = &self.abs_transaction.generate_vector(&mut builder);

        let mut txn_builder =
            buffers::MosaicDefinitionTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.get_value());
        txn_builder.add_maxFee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));

        txn_builder.add_mosaicNonce(self.mosaic_nonce);
        txn_builder.add_mosaicId(mosaic_vec);
        txn_builder.add_flags(f);
        txn_builder.add_divisibility(self.properties.divisibility);
        txn_builder.add_numOptionalProperties(self.properties.optional_properties.len() as u8);
        txn_builder.add_optionalProperties(fb::WIPOffset::new(property_vec));
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        mosaic_definition_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn generate_embedded_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn has_missing_signatures(&self) -> bool {
        unimplemented!()
    }

    fn sign_transaction_with(&self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self as &dyn Transaction, account, generation_hash)
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Into<(String)> for MosaicDefinitionTransaction {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for MosaicDefinitionTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

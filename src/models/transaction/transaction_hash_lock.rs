use std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::models::account::Account;
use crate::models::consts::LOCK_SIZE;
use crate::models::mosaic::Mosaic;
use crate::models::network::NetworkType;
use crate::models::transaction::AbsTransaction;
use crate::Uint64;

use super::{
    AbstractTransaction, buffer::lock_funds::buffers, Deadline, EntityTypeEnum, LOCK_VERSION,
    schema::lock_funds_transaction_schema, sign_transaction, SignedTransaction,
    Transaction
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LockFundsTransaction {
    pub abs_transaction: AbstractTransaction,
    pub mosaic: Mosaic,
    pub duration: Uint64,
    pub signed_transaction: SignedTransaction,
}

impl LockFundsTransaction {
    pub fn new(
        deadline: Deadline,
        mosaic: Mosaic,
        duration: Uint64,
        signed_tx: SignedTransaction,
        network_type: NetworkType,
    ) -> crate::Result<Self> {
        ensure!(
            signed_tx.get_type() == EntityTypeEnum::AggregateBonded,
            "signed_tx must be of type AggregateBonded."
         );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            LOCK_VERSION,
            EntityTypeEnum::Lock,
            network_type
        );

        Ok(Self { abs_transaction: abs_tx, mosaic, duration, signed_transaction: signed_tx })
    }
}

impl AbsTransaction for LockFundsTransaction {
    fn transaction_hash(&self) -> &str {
        self.abs_transaction.get_hash()
    }

    fn has_missing_signatures(&self) -> bool {
        self.abs_transaction.has_missing_signatures()
    }

    fn is_unconfirmed(&self) -> bool {
        self.abs_transaction.is_unconfirmed()
    }

    fn is_confirmed(&self) -> bool {
        self.abs_transaction.is_confirmed()
    }

    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for LockFundsTransaction {
    fn size(&self) -> usize { LOCK_SIZE }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }


    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mosaic_id_vector = _builder.create_vector_direct(&self.mosaic.asset_id.to_int_array());
        let amount_vector = _builder.create_vector_direct(&self.mosaic.amount.to_int_array());
        let duration_vector = _builder.create_vector_direct(&self.duration.to_int_array());
        let hash_vector = _builder.create_vector_direct(&self.signed_transaction.hash_to_bytes());

        let abs_vector = &self.abs_transaction.generate_vector(&mut _builder);

        let mut txn_builder =
            buffers::LockFundsTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));
        txn_builder.add_mosaic_id(mosaic_id_vector);
        txn_builder.add_mosaic_amount(amount_vector);
        txn_builder.add_duration(duration_vector);
        txn_builder.add_hash(hash_vector);

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        lock_funds_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for LockFundsTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
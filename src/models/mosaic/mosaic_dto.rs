use crate::{
    models::{
        errors,
        field_dto::FieldDto,
        metadata_dto::{MetadataModificationDto, MetadataTypeEnum},
        transaction::{AbstractTransactionDto, MosaicDefinitionTransaction,
                      MosaicSupplyChangeTransaction, Transaction, TransactionDto, TransactionMetaDto},
        uint_64::Uint64Dto
    },
    Result,
};

use super::{
    internally::mosaic_properties,
    Mosaic,
    MosaicId,
    MosaicInfo,
    MosaicNames,
    MosaicNonce,
    MosaicSupplyType
};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MosaicDto {
    #[serde(rename = "id")]
    id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

impl MosaicDto {
    pub fn to_struct(&self) -> Mosaic {
        let mosaic_id = MosaicId::from(self.id.to_struct());
        let amount = self.amount.to_struct();
        Mosaic::new(mosaic_id, amount)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicInfoDto {
    #[serde(rename = "meta")]
    meta: MosaicMetaDto,
    #[serde(rename = "mosaic")]
    mosaic: MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn to_struct(&self) -> Result<MosaicInfo> {
        ensure!(
            self.mosaic.properties.len() > 0,
            errors::ERR_INVALID_MOSAIC_PROPERTIES
         );

        let mosaic_id = MosaicId::from(self.mosaic.mosaic_id.to_struct());

        let properties = mosaic_properties(&self.mosaic.properties)?;

        Ok(MosaicInfo::new(
            mosaic_id,
            self.mosaic.supply.to_struct(),
            self.mosaic.height.to_struct(),
            (&self.mosaic.owner).parse()?,
            self.mosaic.revision,
            properties,
        ))
    }
}

#[derive(Serialize, Deserialize)]
struct MosaicMetaDto {
    #[serde(rename = "id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataDto {
    pub metadata_type: i32,
    pub fields: Vec<FieldDto>,
    pub metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: MosaicMetadataDto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: MosaicDefinitionTransactionDto,
}

/// MosaicDefinitionTransactionDto : Transaction that creates a new mosaic.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    /// Random nonce used to generate the mosaic id.
    mosaic_nonce: i32,
    mosaic_id: Uint64Dto,
    properties: Vec<MosaicPropertyDto>,
}

#[typetag::serde]
impl TransactionDto for MosaicDefinitionTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        let properties = mosaic_properties(&dto.properties)?;

        Ok(Box::new(MosaicDefinitionTransaction {
            abs_transaction: abs,
            properties,
            mosaic_nonce: MosaicNonce::from(dto.mosaic_nonce as u32),
            mosaic_id: MosaicId::from(dto.mosaic_id.to_struct())
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionDto {
    mosaic_id: Uint64Dto,
    supply: Uint64Dto,
    height: Uint64Dto,
    owner: String,
    revision: usize,
    properties: Vec<MosaicPropertyDto>,
}

/// MosaicMetadataTransactionDto : Transaction that addes metadata to mosaic.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataTransactionDto {
    signature: String,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicNamesDto {
    mosaic_id: Uint64Dto,
    names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn to_struct(&self) -> MosaicNames {
        MosaicNames::new(
            MosaicId::from(self.mosaic_id.to_struct()),
            (self.names).to_owned())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MosaicPropertyDto {
    pub(crate) id: u8,
    pub(crate) value: Uint64Dto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: MosaicSupplyChangeTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicSupplyChangeTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        Ok(Box::new(MosaicSupplyChangeTransaction {
            abs_transaction: abs,
            supply_type: MosaicSupplyType::from(dto.direction),
            asset_id: Box::new(MosaicId::from(dto.mosaic_id.to_struct())),
            delta: dto.delta.to_struct()
        }))
    }
}

/// MosaicSupplyChangeTransactionDto : Transaction to increase or decrease a mosaic’s supply.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionDto {
    signature: Option<String>,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Option<Uint64Dto>,
    deadline: Option<Uint64Dto>,
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmbeddedMosaicMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}
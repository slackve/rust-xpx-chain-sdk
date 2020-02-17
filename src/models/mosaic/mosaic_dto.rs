use crate::models::{MetadataModificationDto, Uint64Dto, Uint64};
use crate::models::mosaic::{Mosaic, MosaicId, MosaicInfo, MosaicProperties, SUPPLY_MUTABLE, TRANSFERABLE};
use crate::Result;
use crate::models::mosaic::mosaic_internal::{MosaicPropertyId, has_bits};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicDto {
    #[serde(rename = "id")]
    id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

impl MosaicDto {
    pub fn to_struct(&self) -> Mosaic {
        let mosaic_id = Box::new(MosaicId::from(self.id.to_struct()));
        let amount = self.amount.to_struct();
        Mosaic::new(mosaic_id, amount)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
            "mosaic Properties is not valid."
         );

        let mosaic_id = MosaicId::from(self.mosaic.mosaic_id.to_struct());

        let properties =  Self::mosaic_properties(&self.mosaic.properties)?;

        Ok(MosaicInfo::new(
            mosaic_id,
            self.mosaic.supply.to_struct(),
            self.mosaic.height.to_struct(),
            (&self.mosaic.owner).parse()?,
            self.mosaic.revision,
            properties,
        ))
    }

    fn mosaic_properties(dto: &Vec<MosaicPropertyDto>) -> Result<MosaicProperties> {
        let mut flags: Uint64  = Uint64::default();
        let mut divisibility: u8 = 0;
        let mut duration: Uint64 = Uint64::default();

        for property  in dto {
            match property.id {
                0 => flags = property.value.to_struct(),
                1 => divisibility = property.value.to_struct().0 as u8,
                2 => duration = property.value.to_struct(),
                _ => bail!("Unknown Property Id")
            }
        };

        MosaicProperties::new(
            has_bits(flags, SUPPLY_MUTABLE),
            has_bits(flags, TRANSFERABLE),
            divisibility,
            duration,
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MosaicMetaDto {
    #[serde(rename = "id")]
    id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataBodyDto {
    metadata_id: Uint64Dto,
    metadata_type: u16,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}

impl MosaicMetadataBodyDto {
    pub fn new(metadata_id: Uint64Dto, metadata_type: u16, modifications: Vec<MetadataModificationDto>) -> MosaicMetadataBodyDto {
        MosaicMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionDto {
    mosaic_id: Uint64Dto,
    supply: Uint64Dto,
    height: Uint64Dto,
    owner: String,
    revision: usize,
    properties: Vec<MosaicPropertyDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicMetadataDto {
    pub metadata_type: i32,
    pub fields: Vec<crate::models::FieldDto>,
    pub metadata_id: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    metadata_id: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: MosaicMetadataDto,
}

/// MosaicMetadataTransactionDto : Transaction that addes metadata to mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: crate::models::EntityTypeEnum,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<crate::models::MetadataModificationDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicNamesDto {
    mosaic_id: Uint64Dto,
    /// The mosaic linked namespace names.
    names: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MosaicPropertyDto {
    id: u8,
    value: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionBodyDto {
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}

/// MosaicSupplyChangeTransactionDto : Transaction to increase or decrease a mosaic’s supply.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: u8,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionBodyDto {
    /// Random nonce used to generate the mosaic id.
    pub mosaic_nonce: i32,
    pub mosaic_id: Uint64Dto,
    pub properties: Vec<MosaicPropertyDto>,
}

/// MosaicDefinitionTransactionDto : Transaction that creates a new mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: crate::models::EntityTypeEnum,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// Random nonce used to generate the mosaic id.
    mosaic_nonce: i32,
    mosaic_id: Uint64Dto,
    properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmbeddedMosaicDefinitionTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: crate::models::EntityTypeEnum,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// Random nonce used to generate the mosaic id.
    mosaic_nonce: i32,
    mosaic_id: Uint64Dto,
    properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmbeddedMosaicMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: crate::models::EntityTypeEnum,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    metadata_id: Uint64Dto,
    metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    modifications: Vec<crate::models::MetadataModificationDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmbeddedMosaicSupplyChangeTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: i32,
    #[serde(rename = "type")]
    _type: u8,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}

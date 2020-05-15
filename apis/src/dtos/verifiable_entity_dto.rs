#[derive(Serialize, Deserialize)]
pub(crate) struct VerifiableEntityDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node. 
    #[serde(rename = "signature")]
    signature: String,
}



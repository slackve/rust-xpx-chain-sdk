#[derive(Serialize, Deserialize)]
pub struct VerifiableEntityDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node. 
    #[serde(rename = "signature")]
    pub signature: String,
}

impl VerifiableEntityDto {
    pub fn new(signature: String) -> Self {
        VerifiableEntityDto {
            signature,
        }
    }
}



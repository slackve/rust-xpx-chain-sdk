// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

/// ReceiptTypeEnum : The type of the receipt: * 0x134D (4941 decimal) - Mosaic_Rental_Fee. * 0x124E (4686 decimal) - Namespace_Rental_Fee. * 0x2143 (8515 decimal) - Harvest_Fee. * 0x2248 (8776 decimal) - LockHash_Completed. * 0x2348 (9032 decimal) - LockHash_Expired. * 0x2252 (8786 decimal) - LockSecret_Completed. * 0x2352 (9042 decimal) - LockSecret_Expired. * 0x3148 (12616 decimal) - LockHash_Created. * 0x3152 (12626 decimal) - LockSecret_Created. * 0x414D (16717 decimal) - Mosaic_Expired. * 0x414E (16718 decimal) - Namespace_Expired. * 0x5143 (20803 decimal) - Inflation. * 0xE134 (57652 decimal) - Transaction_Group. * 0xF143 (61763 decimal) - Address_Alias_Resolution. * 0xF243 (62019 decimal) - Mosaic_Alias_Resolution.
/// The type of the receipt: * 0x134D (4941 decimal) - Mosaic_Rental_Fee. * 0x124E (4686 decimal) - Namespace_Rental_Fee. * 0x2143 (8515 decimal) - Harvest_Fee. * 0x2248 (8776 decimal) - LockHash_Completed. * 0x2348 (9032 decimal) - LockHash_Expired. * 0x2252 (8786 decimal) - LockSecret_Completed. * 0x2352 (9042 decimal) - LockSecret_Expired. * 0x3148 (12616 decimal) - LockHash_Created. * 0x3152 (12626 decimal) - LockSecret_Created. * 0x414D (16717 decimal) - Mosaic_Expired. * 0x414E (16718 decimal) - Namespace_Expired. * 0x5143 (20803 decimal) - Inflation. * 0xE134 (57652 decimal) - Transaction_Group. * 0xF143 (61763 decimal) - Address_Alias_Resolution. * 0xF243 (62019 decimal) - Mosaic_Alias_Resolution.
#[derive(Serialize, Deserialize)]
pub(crate) enum ReceiptTypeEnum {
    #[serde(rename = "4685")]
    _4685,
    #[serde(rename = "4941")]
    _4941,
    #[serde(rename = "4686")]
    _4686,
    #[serde(rename = "8515")]
    _8515,
    #[serde(rename = "8776")]
    _8776,
    #[serde(rename = "9042")]
    _9042,
    #[serde(rename = "12616")]
    _12616,
    #[serde(rename = "12626")]
    _12626,
    #[serde(rename = "16717")]
    _16717,
    #[serde(rename = "16718")]
    _16718,
    #[serde(rename = "20803")]
    _20803,
    #[serde(rename = "57652")]
    _57652,
    #[serde(rename = "61763")]
    _61763,
    #[serde(rename = "62019")]
    _62019,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: ReceiptTypeEnum,
}

/// InflationReceiptDto : Native currency mosaics were created due to inflation.
#[derive(Serialize, Deserialize)]
pub(crate) struct InflationReceiptDto {
    /// The version of the receipt.
    version: u32,
    #[serde(rename = "type")]
    _type: ReceiptTypeEnum,
    mosaic_id: Vec<i32>,
    amount: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct InflationReceiptDtoAllOf {
    #[serde(rename = "mosaic_id")]
    mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    amount: Vec<i32>,
}

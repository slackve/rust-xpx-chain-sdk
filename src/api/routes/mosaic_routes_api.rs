/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Method};

use crate::{
    api::{
        internally::valid_vec_len, request as __internal_request, sirius_client::ApiClient,
        MosaicInfoDto, MosaicNamesDto,
    },
    errors_const::ERR_EMPTY_MOSAIC_IDS,
    models::Result,
    mosaic::{MosaicId, MosaicIds, MosaicInfo, MosaicNames},
};

use super::{MOSAICS_ROUTE, MOSAIC_NAMES_ROUTE, MOSAIC_ROUTE};

/// Mosaic ApiClient routes.
///
#[derive(Clone)]
pub struct MosaicRoutes(Arc<ApiClient>);

/// Mosaic related endpoints.
///
impl MosaicRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        MosaicRoutes(client)
    }

    fn __client(self) -> Arc<ApiClient> {
        self.0
    }

    /// Get [Mosaic] information.
    ///
    /// Gets the mosaic definition for a given mosaic_id.
    ///
    /// # Inputs
    ///
    /// * `mosaic_id` =    The mosaic identifier.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use xpx_chain_sdk::api::SiriusClient;
    /// use xpx_chain_sdk::mosaic::MosaicId;
    ///
    ///#[tokio::main]
    ///async fn main() {    
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let mosaic_id = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();
    ///
    ///    let mosaic_info = client.mosaic.get_mosaic_info(mosaic_id).await;
    ///
    ///    match mosaic_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [MosaicInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_mosaic_info(self, mosaic_id: MosaicId) -> Result<MosaicInfo> {
        let mut req = __internal_request::Request::new(Method::GET, MOSAIC_ROUTE.to_string());

        req = req.with_path_param("mosaic_id".to_string(), mosaic_id.to_string());

        let dto: Result<MosaicInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact()?)
    }

    /// Gets an vector of [MosaicInfo] definition.
    ///
    /// Get mosaics information for an vector of mosaics
    ///
    /// # Inputs
    ///
    /// * `mosaics_id` =    The vector of mosaic identifiers.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use xpx_chain_sdk::api::SiriusClient;
    ///use xpx_chain_sdk::mosaic::MosaicId;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let mosaic_id_a = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();
    ///    let mosaic_id_b = MosaicId::from_hex("6208AE4D56451357").unwrap();
    ///
    ///    let mosaics_info = client.mosaic.get_mosaics_info(vec![mosaic_id_a, mosaic_id_b]).await;
    ///
    ///    match mosaics_info {
    ///        Ok(resp) => {
    ///            for mosaic in resp {
    ///                println!("{}", mosaic)
    ///            }
    ///        }
    ///        Err(err) => panic!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an verctor of [MosaicInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_mosaics_info(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicInfo>> {
        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;

        let mosaics_ids = MosaicIds::from(mosaic_ids);
        let mut req = __internal_request::Request::new(Method::POST, MOSAICS_ROUTE.to_string());

        req = req.with_body_param(mosaics_ids);

        let dto: Vec<MosaicInfoDto> = req.execute(self.__client()).await?;

        let mut mosaics_info: Vec<MosaicInfo> = vec![];
        for mosaic_dto in dto.into_iter() {
            mosaics_info.push(mosaic_dto.compact()?);
        }

        Ok(mosaics_info)
    }

    /// Get readable names for a set of mosaics
    ///
    /// # Inputs
    ///
    /// * `mosaics_id` =    The vector of mosaic identifiers.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use xpx_chain_sdk::api::SiriusClient;
    ///use xpx_chain_sdk::mosaic::MosaicId;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let mosaic_id_a = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();
    ///    let mosaic_id_b = MosaicId::from_hex("6208AE4D56451357").unwrap();
    ///
    ///    let mosaics_names = client.mosaic.get_mosaics_names(vec![mosaic_id_a, mosaic_id_b]).await;
    ///
    ///    match mosaics_names {
    ///        Ok(resp) => {
    ///            for mosaic in resp {
    ///                println!("{}", mosaic)
    ///            }
    ///        }
    ///        Err(err) => panic!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an friendly names for mosaics or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_mosaics_names(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicNames>> {
        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;

        let mosaics_ids = MosaicIds::from(mosaic_ids);

        let mut req =
            __internal_request::Request::new(Method::POST, MOSAIC_NAMES_ROUTE.to_string());

        req = req.with_body_param(mosaics_ids);

        let dto: Vec<MosaicNamesDto> = req.execute(self.__client()).await?;

        let mosaics_names = dto
            .into_iter()
            .map(move |name_dto| name_dto.compact())
            .collect();

        Ok(mosaics_names)
    }
}

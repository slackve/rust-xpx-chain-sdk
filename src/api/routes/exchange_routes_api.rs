/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Method};

use crate::api::OfferInfoDTOs;
use crate::{
    account::PublicAccount,
    api::{request as __internal_request, sirius_client::ApiClient, ExchangeInfoDto},
    exchange::{OfferInfos, OfferType, UserExchangeInfo},
    models::Result,
    network::NetworkType,
    AssetId, AssetIdType,
};

use super::{resolver_routes_api::ResolverRoutes, EXCHANGE_ROUTE, OFFERS_BY_MOSAIC_ROUTE};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct ExchangeRoutes(Arc<ApiClient>, NetworkType, ResolverRoutes);

/// Exchange related endpoints.
///
impl ExchangeRoutes {
    pub(crate) fn new(
        client: Arc<ApiClient>,
        network_type: NetworkType,
        resolver_routes: ResolverRoutes,
    ) -> Self {
        ExchangeRoutes(client, network_type, resolver_routes)
    }

    fn __client(self) -> Arc<ApiClient> {
        self.0
    }

    fn __network_type(&self) -> NetworkType {
        self.1
    }
    fn __resolver_routes(self) -> ResolverRoutes {
        self.2
    }

    pub async fn get_account_exchange_info(
        self,
        account: PublicAccount,
    ) -> Result<UserExchangeInfo> {
        let mut req = __internal_request::Request::new(Method::GET, EXCHANGE_ROUTE.to_string());

        req = req.with_path_param("account_id".to_string(), account.public_key.to_string());

        let dto: Result<ExchangeInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact(account.address.network_type)?)
    }

    pub async fn get_exchange_offer_by_asset_id(
        self,
        asset_id: impl AssetId,
        offer_type: OfferType,
    ) -> Result<OfferInfos> {
        let mut req =
            __internal_request::Request::new(Method::GET, OFFERS_BY_MOSAIC_ROUTE.to_string());

        let asset_id = match asset_id.get_type() {
            AssetIdType::Namespace => {
                let asset_info = self
                    .clone()
                    .__resolver_routes()
                    .get_mosaic_info_by_asset_id(asset_id)
                    .await?;
                asset_info.mosaic_id.to_hex()
            }
            _ => asset_id.to_hex(),
        };
        req = req.with_path_param("mosaic_id".to_string(), asset_id);

        req = req.with_path_param("offer_type".to_string(), offer_type.to_string());

        let network_type = self.__network_type();

        let dto: OfferInfoDTOs = req.execute(self.__client()).await?;

        let mut offer_infos: OfferInfos = vec![];
        for offer_info_dto in &dto {
            let owner_account = PublicAccount::from_public_key(
                &offer_info_dto.owner.as_ref().unwrap(),
                network_type,
            )?;

            let offer = offer_info_dto.compact(owner_account)?;
            offer_infos.push(offer);
        }

        Ok(offer_infos)
    }
}

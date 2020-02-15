use std::fmt::{Debug, Display};
use std::sync::Arc;

use hyper::client::connect::Connect;
use serde::Deserialize;
use serde_json::Value;

use crate::apis::sirius_client::ApiClient;
use crate::models::account::{AccountInfo, AccountInfoDto};

use super::request as __internal_request;

#[derive(Debug, Clone)]
pub struct AccountRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> AccountRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> AccountRoutesApiClient<C> {
        let clone = client.clone();

        AccountRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> AccountRoutesApiClient<C>
    where
        C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_account_info(self, account_id: &str) -> super::Result<AccountInfo> {
        assert!(
            !account_id.is_empty(),
            "account_id string is empty."
        );

        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/account/{accountId}".to_string(),
        );

        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        let dto: super::Result<AccountInfoDto> = req.execute(self.client).await;

        Ok(dto.unwrap().to_struct()?)
    }
}

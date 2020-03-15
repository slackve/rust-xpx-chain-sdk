use self::error::Error;

mod account_routes_api;
mod block_routes_api;
mod chain_routes_api;
mod error;
mod mosaic_routes_api;
mod node_routes_api;
mod request;
mod transaction_routes_api;
pub(crate) mod namespace_routes_api;

pub mod sirius_client;
pub mod internally;

type Result<T> = std::result::Result<T, Error<serde_json::Value>>;

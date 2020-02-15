use hyper;
use serde;
use serde_json;
use serde_json::Value;

pub use self::account_routes_api::AccountRoutesApiClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiriusError {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error<T> {
    SiriusError(SiriusError),
    UriError(hyper::http::uri::InvalidUri),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
    Failure(failure::Error),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

impl<T> From<failure::Error> for Error<T> {
    fn from(e: failure::Error) -> Self {
        return Error::Failure(e);
    }
}

mod request;
mod account_routes_api;
mod block_routes_api;
//mod chain_routes_api;

pub mod sirius_client;

type Result<T> = std::result::Result<T, Error<Value>>;

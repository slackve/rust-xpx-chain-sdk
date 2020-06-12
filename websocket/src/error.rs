// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use {
    std::{
        borrow::Cow,
        fmt::{Display, Formatter},
        result,
    },
    tokio_tungstenite::tungstenite::Error as WsError,
};

/// Result type of all Websocket library calls.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Tungsten(WsError),
    Failure(failure::Error),
    Url(Cow<'static, str>),
    Api(api::error::Error),
}

impl ::failure::Fail for Error {}

impl From<WsError> for Error {
    fn from(e: WsError) -> Self {
        return Error::Tungsten(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        return Error::Failure(e);
    }
}

impl From<api::error::Error> for Error {
    fn from(e: api::error::Error) -> Self {
        return Error::Api(e);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        match self.to_owned() {
            Error::Serde(e) => write!(f, "{}", e),
            Error::Tungsten(e) => write!(f, "{}", e),
            Error::Failure(e) => write!(f, "{}", e),
            Error::Url(ref msg) => write!(f, "{}", msg),
            Error::Api(ref msg) => write!(f, "{}", msg),
        }
    }
}
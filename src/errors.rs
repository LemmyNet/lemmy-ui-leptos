use serde::{Deserialize, Serialize};
use serde_urlencoded::ser;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum LemmyAppError {
  #[error("Not Found")]
  NotFound,
  #[error("Internal Server Error")]
  InternalServerError,
  #[error("Couldn't parse params")]
  ParamsError,
  #[error("{error:?}")]
  APIError { error: String },
}

impl LemmyAppError {
  // pub fn status_code(&self) -> StatusCode {
  //   match self {
  //     LemmyAppError::NotFound => StatusCode::NOT_FOUND,
  //     LemmyAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
  //     Lemmy
  //   }
  // }
}

impl From<ser::Error> for LemmyAppError {
  fn from(value: ser::Error) -> Self {
    Self::APIError {
      error: value.to_string(),
    }
  }
}

impl From<ParseIntError> for LemmyAppError {
  fn from(_value: ParseIntError) -> Self {
    Self::ParamsError
  }
}

#[cfg(not(feature = "ssr"))]
impl From<gloo_net::Error> for LemmyAppError {
  fn from(_value: gloo_net::Error) -> Self {
    Self::InternalServerError
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::JsonPayloadError> for LemmyAppError {
  fn from(value: awc::error::JsonPayloadError) -> Self {
    Self::APIError {
      error: value.to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::SendRequestError> for LemmyAppError {
  fn from(value: awc::error::SendRequestError) -> Self {
    Self::APIError {
      error: value.to_string(),
    }
  }
}

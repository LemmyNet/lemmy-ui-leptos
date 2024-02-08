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

pub type LemmyAppResult<T> = Result<T, LemmyAppError>;

// impl From<lemmy_client::Error> for LemmyAppError {
//   fn from(e: lemmy_client::Error) -> Self {
//     Self::APIError {
//       error: e.message().to_owned(),
//     }
//   }
// }

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

#[cfg(feature = "ssr")]
impl From<actix_session::SessionGetError> for LemmyAppError {
  fn from(_value: actix_session::SessionGetError) -> Self {
    Self::InternalServerError
  }
}

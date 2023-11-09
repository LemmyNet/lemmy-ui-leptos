use leptos_router::ParamsError;
use serde::{Deserialize, Serialize};
use serde_urlencoded::ser;
use strum_macros::{Display, EnumIter};
// use thiserror::Error;
use tracing_error::SpanTrace;
use std::{
  fmt,
  fmt::{Debug, Display},
  num::ParseIntError, error::Error,
};

use crate::lemmy_errors::LemmyErrorType;


pub type LemmyAppResult<T> = Result<T, LemmyAppError>;

#[derive(Display, Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
#[serde(tag = "error", content = "message", rename_all = "snake_case")]
pub enum LemmyAppErrorType {
  // #[error("Not Found")]
  NotFound,
  // #[error("Internal Server Error")]
  InternalServerError,
  // #[error("Couldn't parse params")]
  ParamsError,
  // #[error("{error:?}")]
  APIError { error: String },

  LoginError,

  Unknown(String),
}

pub struct LemmyAppError {
  pub error_type: LemmyAppErrorType,
  pub inner: anyhow::Error,
  pub context: SpanTrace,
  pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleError {
  pub error: String,
}


impl serde::ser::StdError for LemmyAppError {
    
}

// impl<T> From<T> for LemmyAppError
// where
//   T: Into<anyhow::Error>,
//   // anyhow::Error: From<T>,
// {
//   fn from(t: T) -> Self {
//     let cause = t.into();
//     LemmyAppError {
//       error_type: LemmyAppErrorType::Unknown(format!("{}", &cause)),
//       // inner: cause,
//       // context: SpanTrace::capture(),
//     }
//   }
// }

impl Debug for LemmyAppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("LemmyError")
      .field("message", &self.error_type)
      .field("inner", &self.inner)
      .field("context", &self.context)
      .finish()
  }
}

impl Display for LemmyAppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // write!(f, "error_type - {}: ", &self.error_type)?;
    // writeln!(f, "inner - {:?} context - ", self.inner)?;
    // fmt::Display::fmt(&self.context, f)
    fmt::Display::fmt(&self.content, f)
  }
}

impl From<String> for LemmyAppError {
  fn from(error_type: String) -> Self {
    let inner = anyhow::anyhow!("{}", error_type);
    LemmyAppError {
      error_type: LemmyAppErrorType::APIError {
        error: error_type.clone(),
      },
      inner,
      context: SpanTrace::capture(),
      content: error_type,
    }
  }
}

impl From<LemmyErrorType> for LemmyAppError {
  fn from(error_type: LemmyErrorType) -> Self {
    let inner = anyhow::anyhow!("{}", error_type);
    LemmyAppError {
      error_type: LemmyAppErrorType::APIError {
        error: error_type.to_string(),
      },
      inner,
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

impl From<LemmyAppErrorType> for LemmyAppError {
  fn from(error_type: LemmyAppErrorType) -> Self {
    let inner = anyhow::anyhow!("{}", error_type);
    LemmyAppError {
      error_type,
      inner,
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

// impl LemmyAppError {
  // pub fn status_code(&self) -> StatusCode {
  //   match self {
  //     LemmyAppError::NotFound => StatusCode::NOT_FOUND,
  //     LemmyAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
  //     Lemmy
  //   }
  // }
// }

impl From<ser::Error> for LemmyAppError {
  fn from(value: ser::Error) -> Self { 
    Self { 
      error_type: LemmyAppErrorType::APIError {
        error: value.to_string(),
      },
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

impl From<ParseIntError> for LemmyAppError {
  fn from(value: ParseIntError) -> Self {
    Self{ 
      error_type: LemmyAppErrorType::ParamsError,
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(not(feature = "ssr"))]
impl From<gloo_net::Error> for LemmyAppError {
  fn from(value: gloo_net::Error) -> Self {
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::JsonPayloadError> for LemmyAppError {
  fn from(value: awc::error::JsonPayloadError) -> Self {
    Self { 
      error_type: LemmyAppErrorType::APIError {
        error: value.to_string(),
      },
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::SendRequestError> for LemmyAppError {
  fn from(value: awc::error::SendRequestError) -> Self {
    Self { 
      error_type: LemmyAppErrorType::APIError {
        error: value.to_string(),
      },
      inner: anyhow::anyhow!("{}", value),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<actix_session::SessionGetError> for LemmyAppError {
  fn from(value: actix_session::SessionGetError) -> Self {
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<actix_http::error::PayloadError> for LemmyAppError {
  fn from(value: actix_http::error::PayloadError) -> Self {
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<std::str::Utf8Error> for LemmyAppError {
  fn from(value: std::str::Utf8Error) -> Self {
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      inner: value.into(),
      context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

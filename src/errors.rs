use leptos::{leptos_dom::logging, ServerFnError};
use leptos_router::ParamsError;
use serde::{Deserialize, Serialize};
use serde_urlencoded::ser;
use strum_macros::{Display, EnumIter};
// use thiserror::Error;
use tracing_error::SpanTrace;
use std::{
  // *,
  // fmt::*,
  // fmt::{Debug, Display},
  num::ParseIntError, error::Error,
};

use crate::lemmy_errors::LemmyErrorType;


pub type LemmyAppResult<T> = Result<T, LemmyAppError>;

#[derive(Default, Display, Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
#[serde(tag = "error", content = "message", rename_all = "snake_case")]
pub enum LemmyAppErrorType {
  #[default]
  Unknown,

  // #[error("Not Found")]
  NotFound,
  // #[error("Internal Server Error")]
  InternalServerError,
  // #[error("Couldn't parse params")]
  ParamsError,
  // #[error("{error:?}")]
  // ApiError { /* error: String */ inner: Option<LemmyErrorType> },
  ApiError(LemmyErrorType),

  EmptyUsername,
  EmptyPassword,
  MissingToken,

}

#[derive(/* Debug, Clone,  */Serialize, Deserialize/* , PartialEq */)]
pub struct LemmyAppError {
  pub error_type: LemmyAppErrorType,
  // pub inner: anyhow::Error,
  // pub context: SpanTrace,
  pub content: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct SimpleError {
//   pub error: String,
// }


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

impl std::fmt::Debug for LemmyAppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("LemmyAppError")
      .field("message", &self.error_type)
      // .field("inner", &self.inner)
      // .field("context", &self.context)
      .field("context", &self.content)
      .finish()
  }
}

impl std::fmt::Display for LemmyAppError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // leptos::logging::log!("woop {}", SpanTrace::capture());
    leptos::logging::log!("woop {:#?}", &self);
    match &self.error_type {
        // LemmyAppErrorType::ApiError { inner } => {                                                                                  
        //   write!(f, "{}: {{ {} }}", &self.error_type, inner)
        // },
        LemmyAppErrorType::ApiError(inner) => {
          write!(f, "{{\"error_type\":{{\"{}\": {}}}}}", &self.error_type, serde_json::to_string(inner).ok().unwrap())
        },
        // LemmyAppErrorType::ApiError(inner) => {
        //   write!(f, "{}", serde_json::to_string(inner).ok().unwrap())
        // },
        _ => {
          write!(f, "{{\"error_type\":\"{}\"}}", &self.error_type)
        },
    }
    // writeln!(f, "inner - {:?} context - ", self.inner)?;
    // fmt::Display::fmt(&self.context, f)
    // std::fmt::Display::fmt(&self.error_type, f)
  }
}

// impl From<String> for LemmyAppError {
//   fn from(error_type: String) -> Self {
//     let inner = anyhow::anyhow!("{}", error_type);
//     LemmyAppError {
//       error_type: LemmyAppErrorType::ApiError {
//         error: error_type.clone(),
//       },
//       // inner,
//       // context: SpanTrace::capture(),
//       // content: error_type,
//     }
//   }
// }

impl From<LemmyErrorType> for LemmyAppError {
  fn from(error_type: LemmyErrorType) -> Self {
    // leptos::logging::log!("trace {}", SpanTrace::capture());
    // let inner = anyhow::anyhow!("{}", error_type);
    LemmyAppError {
      // error_type: LemmyAppErrorType::ApiError {
      //   // error: error_type.to_string(),
      //   inner: Some(error_type),
      // },
      error_type: LemmyAppErrorType::ApiError(error_type.clone()),
      // inner,
      // context: SpanTrace::capture(),
      content: format!("{:#?}", error_type),
    }
  }
}

impl From<LemmyAppErrorType> for LemmyAppError {
  fn from(error_type: LemmyAppErrorType) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    // let inner = anyhow::anyhow!("{}", error_type);
    LemmyAppError {
      error_type,
      // inner,
      // context: SpanTrace::capture(),
      content: "".to_string(),
    }
  }
}

impl From<ser::Error> for LemmyAppError {
  fn from(value: ser::Error) -> Self { 
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self { 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

impl From<ParseIntError> for LemmyAppError {
  fn from(value: ParseIntError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::ParamsError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(not(feature = "ssr"))]
impl From<gloo_net::Error> for LemmyAppError {
  fn from(value: gloo_net::Error) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::JsonPayloadError> for LemmyAppError {
  fn from(value: awc::error::JsonPayloadError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self { 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<awc::error::SendRequestError> for LemmyAppError {
  fn from(value: awc::error::SendRequestError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self { 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: anyhow::anyhow!("{}", value),
      // context: SpanTrace::capture(),
      content: format!("{} {:#?}", value, value.source()),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<actix_session::SessionGetError> for LemmyAppError {
  fn from(value: actix_session::SessionGetError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<actix_http::error::PayloadError> for LemmyAppError {
  fn from(value: actix_http::error::PayloadError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<std::str::Utf8Error> for LemmyAppError {
  fn from(value: std::str::Utf8Error) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

#[cfg(feature = "ssr")]
impl From<ServerFnError> for LemmyAppError {
  fn from(value: ServerFnError) -> Self {
    leptos::logging::log!("trace {}", SpanTrace::capture());
    Self{ 
      error_type: LemmyAppErrorType::InternalServerError,
      // inner: value.into(),
      // context: SpanTrace::capture(),
      content: format!("{:#?}", value),
      // content: "".to_string(),
    }
  }
}

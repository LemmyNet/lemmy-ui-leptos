// TODO: Figure out how to handle errors

// use crate::{i18n::*, lemmy_errors::LemmyErrorType};
// // use lemmy_api_common::error::*;
// use core::num::ParseIntError;
// use leptos::*;
// use serde::{Deserialize, Serialize};
// use serde_urlencoded::ser;
// use strum_macros::Display;

// pub type LemmyAppResult<T> = Result<T, LemmyAppError>;

// #[derive(Default, Display, Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(tag = "error", content = "message", rename_all = "snake_case")]
// pub enum LemmyAppErrorType {
//   #[default]
//   Unknown,
//   NotFound,
//   InternalServerError,
//   InternalClientError,
//   ParamsError,

//   ApiError(LemmyErrorType),

//   EmptyUsername,
//   EmptyPassword,
//   MissingToken,

//   MissingReason,
// }

// pub fn message_from_error(error: &LemmyAppError) -> String {
//   let i18n = use_i18n();

//   let s = match error.error_type {
//     LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin) => {
//       t!(i18n, invalid_login)().to_string()
//     }
//     LemmyAppErrorType::EmptyUsername => t!(i18n, empty_username)().to_string(),
//     LemmyAppErrorType::EmptyPassword => t!(i18n, empty_password)().to_string(),
//     LemmyAppErrorType::MissingReason => t!(i18n, empty_reason)().to_string(),
//     LemmyAppErrorType::InternalServerError => t!(i18n, internal)().to_string(),
//     LemmyAppErrorType::Unknown => t!(i18n, unknown)().to_string(),
//     _ => "An error without description".to_string(),
//   };

//   logging::error!("{} - {}", s, error.content);

//   s
// }

// #[derive(Clone, Serialize, Deserialize)]
// pub struct LemmyAppError {
//   pub error_type: LemmyAppErrorType,
//   pub content: String,
// }

// impl serde::ser::StdError for LemmyAppError {}

// impl core::fmt::Debug for LemmyAppError {
//   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//     f.debug_struct("debug LemmyAppError")
//       .field("error_type", &self.error_type)
//       .field("content", &self.content)
//       .finish()
//   }
// }

// impl core::fmt::Display for LemmyAppError {
//   fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//     match &self.error_type {
//       LemmyAppErrorType::ApiError(inner) => {
//         write!(
//           f,
//           "{{\"error_type\":{{\"{}\": {}}}}}",
//           &self.error_type,
//           serde_json::to_string(inner).ok().unwrap()
//         )
//       }
//       _ => {
//         write!(f, "{{\"error_type\":\"{}\"}}", &self.error_type)
//       }
//     }
//   }
// }

// impl From<LemmyErrorType> for LemmyAppError {
//   fn from(error_type: LemmyErrorType) -> Self {
//     LemmyAppError {
//       error_type: LemmyAppErrorType::ApiError(error_type.clone()),
//       content: format!("{:#?}", error_type),
//     }
//   }
// }

// impl From<LemmyAppErrorType> for LemmyAppError {
//   fn from(error_type: LemmyAppErrorType) -> Self {
//     LemmyAppError {
//       error_type,
//       content: "".to_string(),
//     }
//   }
// }

// impl From<ser::Error> for LemmyAppError {
//   fn from(value: ser::Error) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// impl From<serde_json::error::Error> for LemmyAppError {
//   fn from(value: serde_json::error::Error) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// impl From<ParseIntError> for LemmyAppError {
//   fn from(value: ParseIntError) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::ParamsError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// impl From<web_sys::wasm_bindgen::JsValue> for LemmyAppError {
//   fn from(value: web_sys::wasm_bindgen::JsValue) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalClientError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// pub trait NoneError<T>
// where
//   Self: std::marker::Sized,
// {
//   fn n(self) -> Result<T, LemmyAppErrorType>;
// }

// impl<T> NoneError<T> for Option<T> {
//   fn n(self) -> Result<T, LemmyAppErrorType> {
//     self.ok_or(LemmyAppErrorType::InternalClientError)
//   }
// }

// #[cfg(not(feature = "ssr"))]
// impl From<gloo_net::Error> for LemmyAppError {
//   fn from(value: gloo_net::Error) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// #[cfg(not(feature = "ssr"))]
// impl From<wasm_cookies::FromUrlEncodingError> for LemmyAppError {
//   fn from(value: wasm_cookies::FromUrlEncodingError) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// #[cfg(feature = "ssr")]
// impl From<awc::error::JsonPayloadError> for LemmyAppError {
//   fn from(value: awc::error::JsonPayloadError) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// #[cfg(feature = "ssr")]
// impl From<awc::error::SendRequestError> for LemmyAppError {
//   fn from(value: awc::error::SendRequestError) -> Self {
//     use std::error::Error;
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{} - source: {:?}", value, value.source()),
//     }
//   }
// }

// #[cfg(feature = "ssr")]
// impl From<actix_http::error::PayloadError> for LemmyAppError {
//   fn from(value: actix_http::error::PayloadError) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// #[cfg(feature = "ssr")]
// impl From<core::str::Utf8Error> for LemmyAppError {
//   fn from(value: core::str::Utf8Error) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

// #[cfg(feature = "ssr")]
// impl From<ServerFnError> for LemmyAppError {
//   fn from(value: ServerFnError) -> Self {
//     Self {
//       error_type: LemmyAppErrorType::InternalServerError,
//       content: format!("{:#?}", value),
//     }
//   }
// }

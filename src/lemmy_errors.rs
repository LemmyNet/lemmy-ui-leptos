// TODO: Figure out how to handle errors

// use anyhow::Error;
// use core::fmt::{self, Debug, Display};
// use lemmy_client::lemmy_api_common::LemmyErrorType;
// use serde::{Deserialize, Serialize};
// use strum_macros::{Display, EnumIter};
// use tracing_error::SpanTrace;

// #[allow(dead_code)]
// pub type LemmyResult<T> = Result<T, LemmyError>;

// pub struct LemmyError {
//   pub error_type: LemmyErrorType,
//   pub inner: Error,
//   pub context: SpanTrace,
// }

// #[allow(dead_code)]
// pub const MAX_API_PARAM_ELEMENTS: usize = 1000;

// impl Debug for LemmyError {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     f.debug_struct("LemmyError")
//       .field("message", &self.error_type)
//       .field("inner", &self.inner)
//       .field("context", &self.context)
//       .finish()
//   }
// }

// impl Display for LemmyError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}: ", &self.error_type)?;
//     writeln!(f, "{:?}", self.inner)?;
//     Display::fmt(&self.context, f)
//   }
// }

// impl From<LemmyErrorType> for LemmyError {
//   fn from(error_type: LemmyErrorType) -> Self {
//     let inner = anyhow::anyhow!("{error_type}");
//     LemmyError {
//       error_type,
//       inner,
//       context: SpanTrace::capture(),
//     }
//   }
// }

// impl<T, E: Into<Error>> LemmyErrorExt<T, E> for Result<T, E> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError> {
//     self.map_err(|error| LemmyError {
//       error_type,
//       inner: error.into(),
//       context: SpanTrace::capture(),
//     })
//   }
// }

// pub trait LemmyErrorExt2<T> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError>;
//   fn into_anyhow(self) -> Result<T, Error>;
// }

// pub trait LemmyErrorExt<T, E: Into<Error>> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError>;
// }

// impl<T> LemmyErrorExt2<T> for Result<T, LemmyError> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError> {
//     self.map_err(|mut e| {
//       e.error_type = error_type;
//       e
//     })
//   }
//   fn into_anyhow(self) -> Result<T, Error> {
//     self.map_err(|e| e.inner)
//   }
// }

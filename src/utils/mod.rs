mod derive_query_signal;
use actix_session::{Session, SessionGetError};
pub use derive_query_signal::*;

mod derive_user_is_logged_in;
pub use derive_user_is_logged_in::*;
mod types;
pub use types::*;

#[cfg(feature = "ssr")]
mod get_client_and_session;
use crate::constants::AUTH_COOKIE;
#[cfg(feature = "ssr")]
pub use get_client_and_session::*;

pub trait GetJwt {
  fn get_jwt(&self) -> Result<Option<String>, SessionGetError>;
}

impl GetJwt for Session {
  fn get_jwt(&self) -> Result<Option<String>, SessionGetError> {
    self.get::<String>(AUTH_COOKIE)
  }
}

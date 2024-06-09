mod derive_query_signal;
pub use derive_query_signal::*;

mod derive_user_is_logged_in;
pub use derive_user_is_logged_in::*;
pub mod types;

mod format_number_si;
pub use format_number_si::format_number_si;

#[cfg(feature = "ssr")]
mod get_client_and_session;
use crate::constants::AUTH_COOKIE;
#[cfg(feature = "ssr")]
pub use get_client_and_session::*;

#[cfg(feature = "ssr")]
pub trait GetJwt {
  fn get_jwt(&self) -> Result<Option<String>, actix_session::SessionGetError>;
}

#[cfg(feature = "ssr")]
impl GetJwt for actix_session::Session {
  fn get_jwt(&self) -> Result<Option<String>, actix_session::SessionGetError> {
    self.get::<String>(AUTH_COOKIE)
  }
}

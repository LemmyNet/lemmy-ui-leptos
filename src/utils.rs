mod apub_name;
pub use apub_name::*;

mod derive_user_is_logged_in;
pub use derive_user_is_logged_in::*;

mod filetype;
pub use filetype::*;

mod get_time_since;
pub use get_time_since::get_time_since;

pub mod traits;
pub mod types;

mod derive_query_param_type;
pub use derive_query_param_type::*;

mod markdown;
pub use markdown::markdown_to_html;

mod build_comment_tree;

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

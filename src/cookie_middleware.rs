use crate::constants::AUTH_COOKIE;
use actix_session::{
  config::{CookieContentSecurity, PersistentSession},
  storage::CookieSessionStore,
  SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};

/// Where the session key comes from.
pub const SESSION_KEY_VAR: &str = "LEMMY_UI_SESSION_KEY";

/// The key that encrypts the session cookie, which holds the user's Lemmy JWT.
///
/// Set [`SESSION_KEY_VAR`] to at least 64 bytes of random text, for example the output of
/// `openssl rand -base64 64`, and keep it secret: anyone with it can read the JWT out of a
/// session cookie. Keep it the same across restarts and between instances behind one domain, or
/// everyone is signed out. A release build refuses to start without one; a debug build falls
/// back to a fixed, public key and says so.
pub fn session_key() -> Result<Key, String> {
  match std::env::var(SESSION_KEY_VAR) {
    Ok(value) => Key::try_from(value.as_bytes()).map_err(|_| {
      format!("{SESSION_KEY_VAR} must be at least 64 bytes (try `openssl rand -base64 64`)")
    }),
    Err(_) if cfg!(debug_assertions) => {
      eprintln!(
        "warning: {SESSION_KEY_VAR} isn't set, so sessions use a fixed, public key. \
         Fine for development, never for a real instance."
      );
      Ok(Key::from(&[0; 64]))
    }
    Err(_) => Err(format!(
      "{SESSION_KEY_VAR} isn't set. It encrypts the session cookie (which holds the user's \
       Lemmy JWT); set it to at least 64 random bytes, e.g. `openssl rand -base64 64`."
    )),
  }
}

pub fn cookie_middleware(key: Key) -> SessionMiddleware<CookieSessionStore> {
  let debug_mode = cfg!(debug_assertions);

  SessionMiddleware::builder(CookieSessionStore::default(), key)
    .cookie_name(AUTH_COOKIE.into())
    .cookie_secure(!debug_mode)
    .session_lifecycle(PersistentSession::default())
    .cookie_same_site(if debug_mode {
      SameSite::Lax
    } else {
      SameSite::Strict
    })
    .cookie_content_security(CookieContentSecurity::Private)
    .cookie_http_only(true)
    .build()
}

#[cfg(test)]
mod tests {
  use super::*;

  // One test, since the environment is shared between tests running at once.
  #[test]
  fn session_key_comes_from_the_environment() {
    std::env::set_var(SESSION_KEY_VAR, "too short");
    assert!(session_key().is_err());

    let key = "k".repeat(64);
    std::env::set_var(SESSION_KEY_VAR, &key);
    assert_eq!(
      session_key().unwrap().master(),
      Key::from(key.as_bytes()).master()
    );

    std::env::remove_var(SESSION_KEY_VAR);
    // Tests are debug builds: the fixed development key.
    assert_eq!(
      session_key().unwrap().master(),
      Key::from(&[0; 64]).master()
    );
  }
}

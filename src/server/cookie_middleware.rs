use crate::constants::AUTH_COOKIE;
use actix_session::{
  config::{CookieContentSecurity, PersistentSession},
  storage::CookieSessionStore,
  SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};
pub fn cookie_middleware() -> SessionMiddleware<CookieSessionStore> {
  SessionMiddleware::builder(
    CookieSessionStore::default(),
    Key::from(&[0; 64]), // TODO: The key should definitely be read from a config value for production
  )
  .cookie_name(AUTH_COOKIE.into())
  .cookie_secure(cfg!(debug_assertions))
  .session_lifecycle(PersistentSession::default())
  .cookie_same_site(SameSite::Strict)
  .cookie_content_security(CookieContentSecurity::Private)
  .cookie_http_only(true)
  .build()
}

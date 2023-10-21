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
  .cookie_name(String::from("jwt"))
  .cookie_secure(false) // TODO: Make cookie secure option depend on whether in dev or prod
  .session_lifecycle(PersistentSession::default())
  .cookie_same_site(SameSite::Strict)
  .cookie_content_security(CookieContentSecurity::Private)
  .cookie_http_only(true)
  .build()
}

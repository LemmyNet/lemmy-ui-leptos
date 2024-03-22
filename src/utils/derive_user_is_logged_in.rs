use crate::queries::site_state_query::SiteStateSignal;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserLoggedIn(pub bool);

pub fn derive_user_is_logged_in(site_response: SiteStateSignal) -> Signal<UserLoggedIn> {
  Signal::derive(move || {
    with!(|site_response| UserLoggedIn(
      site_response
        .as_ref()
        .and_then(|site_response| site_response.as_ref().ok())
        .map_or(false, |site_response| site_response.my_user.is_some())
    ))
  })
}

use leptos::prelude::{Read, Signal};

use crate::contexts::site_resource_context::SiteResource;

pub fn derive_user_is_logged_in(site_signal: SiteResource) -> Signal<bool> {
  Signal::derive(move || {
    site_signal
      .read()
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .map_or(false, |s| s.my_user.is_some())
  })
}

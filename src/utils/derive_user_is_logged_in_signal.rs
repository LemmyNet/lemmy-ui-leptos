use crate::resources::site_resource::SiteResource;
use leptos::{with, Signal};

pub fn derive_user_is_logged_in_signal(site: SiteResource) -> Signal<bool> {
  Signal::derive(move || {
    with!(|site| site
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .map_or(false, |s| s.my_user.is_some()))
  })
}

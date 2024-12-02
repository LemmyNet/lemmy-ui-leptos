use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::prelude::{Read, ServerFnError, Signal};

pub fn derive_user_is_logged_in<S>(site_signal: S) -> Signal<bool>
where
  S: Read<Value = Option<Result<GetSiteResponse, ServerFnError>>> + 'static,
{
  Signal::derive(move || {
    site_signal
      .read()
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .map_or(false, |s| s.my_user.is_some())
  })
}

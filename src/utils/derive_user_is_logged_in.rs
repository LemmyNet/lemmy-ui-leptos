use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{with, ServerFnError, Signal, SignalWith};

pub fn derive_user_is_logged_in<S>(site_signal: S) -> Signal<bool>
where
  S: SignalWith<Value = Option<Result<GetSiteResponse, ServerFnError>>> + 'static,
{
  Signal::derive(move || {
    with!(|site_signal| site_signal
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .map_or(false, |s| s.my_user.is_some()))
  })
}

use crate::config::{LEMMY_UI_LEPTOS_LEMMY_HOST, LEMMY_UI_LEPTOS_LEMMY_HTTPS};
use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
pub fn get_internal_host() -> String {
  std::env::var("LEMMY_UI_LEPTOS_LEMMY_HOST").unwrap_or_else(|_| LEMMY_UI_LEPTOS_LEMMY_HOST.into())
}

#[cfg(not(feature = "ssr"))]
pub fn get_external_host() -> String {
  if let Some(s) = option_env!("LEMMY_UI_LEPTOS_LEMMY_HOST") {
    s.into()
  } else {
    LEMMY_UI_LEPTOS_LEMMY_HOST.into()
  }
}

pub fn get_host() -> String {
  cfg_if! {
      if #[cfg(feature="ssr")] {
        get_internal_host()
      } else {
        get_external_host()
      }
  }
}

pub fn get_https() -> String {
  cfg_if! {
      if #[cfg(feature="ssr")] {
        std::env::var("LEMMY_UI_LEPTOS_LEMMY_HTTPS").unwrap_or(format!("{LEMMY_UI_LEPTOS_LEMMY_HTTPS}"))
      } else {
        if let Some(s) = option_env!("LEMMY_UI_LEPTOS_LEMMY_HTTPS") {
          s.into()
        } else {
          format!("{LEMMY_UI_LEPTOS_LEMMY_HTTPS}")
        }
      }
  }
}

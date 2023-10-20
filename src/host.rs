use crate::config::{LEMMY_UI_HTTPS, LEMMY_UI_LEMMY_INTERNAL_HOST};
use cfg_if::cfg_if;

pub fn get_internal_host() -> String {
  std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST")
    .unwrap_or_else(|_| String::from(LEMMY_UI_LEMMY_INTERNAL_HOST))
}

// #[allow(dead_code)]
pub fn get_external_host() -> String {
  leptos::logging::log!("ext 1");

  cfg_if! {
    if #[cfg(not(feature="bypass_internal_proxy"))] {
      leptos::logging::log!("ext 2");
      let location = leptos::window().location();

      format!(
        "{}:{}",
        location.hostname().unwrap(),
        location.port().unwrap()
      )
    } else {
      leptos::logging::log!("ext 3 {} ", env!("LEMMY_UI_LEMMY_INTERNAL_HOST"));

      // std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST").unwrap_or_else(|_| String::from(LEMMY_UI_LEMMY_INTERNAL_HOST))
      // std::env::var("LEMMY_UI_LEMMY_EXTERNAL_HOST").unwrap_or_else(|_| String::from(TEST_HOST))
      env!("LEMMY_UI_LEMMY_INTERNAL_HOST").to_string()
    }
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
        std::env::var("LEMMY_UI_HTTPS").unwrap_or(format!("{LEMMY_UI_HTTPS}"))
      } else {
        env!("LEMMY_UI_HTTPS").to_string()
      }
  }
}

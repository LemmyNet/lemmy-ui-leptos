use crate::config::LEMMY_UI_LEMMY_INTERNAL_HOST;
use cfg_if::cfg_if;

pub fn get_internal_host() -> String {
  // cfg_if! {
  // if #[cfg(feature="ssr")] {
  std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST")
    .unwrap_or_else(|_| String::from(LEMMY_UI_LEMMY_INTERNAL_HOST))
  // } else {
  //     String::from(TEST_HOST)
  // }
  // }
}

// #[allow(dead_code)]
pub fn get_external_host() -> String {
  cfg_if! {
    if #[cfg(not(feature="bypass_internal_proxy"))] {
      let location = leptos::window().location();

      format!(
        "{}:{}",
        location.hostname().unwrap(),
        location.port().unwrap()
      )
    } else {
      get_host()
      // std::env::var("LEMMY_UI_LEMMY_EXTERNAL_HOST").unwrap_or_else(|_| String::from(TEST_HOST))
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

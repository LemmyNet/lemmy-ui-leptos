use crate::config::TEST_HOST;
use cfg_if::cfg_if;

pub fn get_internal_host() -> String {
  cfg_if! {
      if #[cfg(feature="ssr")] {
          std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST").unwrap_or_else(|_| String::from(TEST_HOST))
      } else {
          String::from(TEST_HOST)
      }
  }
}

#[allow(dead_code)]
pub fn get_external_host() -> String {
  cfg_if! {
      if #[cfg(not(feature="ssr"))] {
          let location = leptos::window().location();

          format!("{}:{}", location.hostname().unwrap(), location.port().unwrap())
      } else {
        std::env::var("LEMMY_UI_LEMMY_EXTERNAL_HOST").unwrap_or_else(|_| String::from(TEST_HOST))
      }
  }
}

pub fn get_host() -> String {
  cfg_if! {
      if #[cfg(not(feature="ssr"))] {
          get_external_host()
      } else {
          get_internal_host()
      }
  }
}

use crate::config::{LEMMY_UI_HTTPS, LEMMY_UI_LEMMY_INTERNAL_HOST};
use cfg_if::cfg_if;
use lemmy_client::{ClientOptions, LemmyClient};

#[cfg(feature = "ssr")]
fn get_internal_host() -> String {
  std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST")
    .unwrap_or_else(|_| LEMMY_UI_LEMMY_INTERNAL_HOST.into())
}

#[cfg(not(feature = "ssr"))]
fn get_external_host() -> String {
  cfg_if! {
    if #[cfg(not(feature = "bypass_internal_proxy"))] {
      let location = leptos::window().location();

      format!(
        "{}:{}",
        location.hostname().unwrap(),
        location.port().unwrap()
      )
    } else {
      if let Some(s) = option_env!("LEMMY_UI_LEMMY_INTERNAL_HOST") {
        s.into()
      } else {
        LEMMY_UI_LEMMY_INTERNAL_HOST.into()
      }
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
        if let Some(s) = option_env!("LEMMY_UI_HTTPS") {
          s.into()
        } else {
          format!("{LEMMY_UI_HTTPS}")
        }
      }
  }
}

fn should_use_https() -> bool {
  let https_env_var;
  cfg_if! {
      if #[cfg(feature="ssr")] {
        https_env_var = std::env::var("LEMMY_UI_HTTPS");
      } else {
        https_env_var = option_env!("LEMMY_UI_HTTPS");
      }
  }

  https_env_var.map_or(LEMMY_UI_HTTPS, |var| var == "true")
}

pub fn get_client() -> LemmyClient {
  LemmyClient::new(ClientOptions {
    domain: get_host(),
    secure: should_use_https(),
  })
}

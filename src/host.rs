use crate::constants::{HTTPS, INTERNAL_HOST};
use cfg_if::cfg_if;
use lemmy_client::{ClientOptions, LemmyClient};

#[cfg(feature = "ssr")]
fn get_internal_host() -> String {
  std::env::var("INTERNAL_HOST").unwrap_or_else(|_| INTERNAL_HOST.into())
}

#[cfg(not(feature = "ssr"))]
fn get_external_host() -> String {
  let location = leptos::window().location();

  format!(
    "{}:{}",
    location.hostname().unwrap(),
    location.port().unwrap()
  )
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
        std::env::var("HTTPS").unwrap_or_else(|_| format!("{HTTPS}"))
      } else {
        option_env!("HTTPS").map_or_else(|| format!("{HTTPS}"), Into::into)
      }
  }
}

fn should_use_https() -> bool {
  #[allow(clippy::needless_late_init)]
  let https_env_var;
  cfg_if! {
      if #[cfg(feature="ssr")] {
        https_env_var = std::env::var("HTTPS");
      } else {
        https_env_var = option_env!("HTTPS");
      }
  };

  https_env_var.map_or(HTTPS, |var| var == "true")
}

pub fn get_client() -> LemmyClient {
  LemmyClient::new(ClientOptions {
    domain: get_host(),
    secure: should_use_https(),
  })
}

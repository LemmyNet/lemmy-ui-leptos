use cfg_if::cfg_if;
use std::str::FromStr;

pub fn create_user_apub_name(name: &str, actor_id: &str) -> String {
  create_apub_name::<'@'>(name, actor_id).unwrap_or_default()
}

pub fn create_community_apub_name(name: &str, actor_id: &str) -> String {
  create_apub_name::<'!'>(name, actor_id).unwrap_or_default()
}

fn format_apub_name<const PREFIX: char>(name: &str, instance: &str) -> String {
  format!("{PREFIX}{name}@{instance}")
}

fn create_apub_name<const PREFIX: char>(name: &str, actor_id: &str) -> Option<String> {
  cfg_if! {
      if #[cfg(feature = "ssr")] {
        use actix_web::http::Uri;
        let url = Uri::from_str(actor_id).ok()?;
        let instance = url.host().expect("No host name in actor id");
      } else {
        use web_sys::Url;
        let instance = Url::new(actor_id).ok()?.host();
        let instance = instance.as_str();
      }
  }

  Some(format_apub_name::<PREFIX>(name, instance))
}

// TODO: Figure out how to test functions when targeting wasm
#[cfg(test)]
mod tests {
  use super::{create_community_apub_name, create_user_apub_name};

  #[test]
  fn formats_user_apub_correctly() {
    let result = create_user_apub_name("lemmy", "http://lemmy.ml/u/lemmy");
    assert_eq!(result, "@lemmy@lemmy.ml");
  }

  #[test]
  fn formats_community_apub_correctly() {
    let result = create_community_apub_name("memes", "https://lemmy.ml/c/memes");
    assert_eq!(result, "!memes@lemmy.ml");
  }
}

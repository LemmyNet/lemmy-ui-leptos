use cfg_if::cfg_if;

pub fn create_user_apub_name(name: &str, actor_id: &str) -> String {
  create_apub_name::<'@'>(name, actor_id)
}

pub fn create_community_apub_name(name: &str, actor_id: &str) -> String {
  create_apub_name::<'!'>(name, actor_id)
}

fn format_apub_name<const PREFIX: char>(name: &str, instance: &str) -> String {
  format!("{PREFIX}{name}@{instance}")
}

fn create_apub_name<const PREFIX: char>(name: &str, actor_id: &str) -> String {
  // TODO Strange issue where the must be defined, or leptos will fail at startup
  let default_url = "https://example.com";

  cfg_if! {
      if #[cfg(feature = "ssr")] {
        use url::Url;
        let url = Url::parse(actor_id).unwrap_or(Url::parse(default_url).unwrap());
        let instance = url.host_str().expect("No host name in actor id");
      } else {
        use web_sys::Url;
        let instance = Url::new(actor_id).unwrap_or(Url::new(default_url).unwrap()).host();
        let instance = instance.as_str();
      }
  }

  format_apub_name::<PREFIX>(name, instance)
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

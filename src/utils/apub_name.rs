use cfg_if::cfg_if;
use std::sync::LazyLock;

pub fn create_user_apub_name(actor_id: &str) -> String {
  create_apub_name::<'@'>(actor_id)
}

pub fn create_community_apub_name(actor_id: &str) -> String {
  create_apub_name::<'!'>(actor_id)
}

fn format_apub_name<const PREFIX: char>(instance: &str, name: &str) -> String {
  format!("{PREFIX}{name}@{instance}")
}

fn create_apub_name<const PREFIX: char>(actor_id: &str) -> String {
  const REGEX_STR: &str = r"https?:\/\/((?:(?:[^.]\.)*[^.]+\.[^:]+)|localhost)(?::\d{1,5})?\/[uc]\/([^?#]+)";

  cfg_if! {
      if #[cfg(feature = "ssr")] {
        use regex::Regex;
        static INSTANCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(REGEX_STR)
          .expect("Could not create server side regex.")
        );

        INSTANCE_REGEX
          .captures(actor_id)
          .map_or_else(String::new, |captures| format_apub_name::<PREFIX>(&captures[1], &captures[2]))
      } else {
        use js_sys::RegExp;
        const REGEX_GROUP_NOT_STRING_ERROR_MESSAGE: &str = "Regex capture groups should always be Strings";

        thread_local! {
            static INSTANCE_REGEX: LazyLock<RegExp> = LazyLock::new(|| RegExp::new(REGEX_STR, "u"));
        }

        INSTANCE_REGEX
          .with(|instance_regex| {
            instance_regex
              .exec(actor_id)
              .map_or_else(String::new, |captures| {
                let instance = captures
                .at(1)
                .as_string()
                .expect(REGEX_GROUP_NOT_STRING_ERROR_MESSAGE);
              let name = captures
                .at(2)
                .as_string()
                .expect(REGEX_GROUP_NOT_STRING_ERROR_MESSAGE);

              format_apub_name::<PREFIX>(&instance, &name)
            })
        })
      }
  }
}

// TODO: Figure out how to test functions when targeting wasm
#[cfg(test)]
mod tests {
  use rstest::rstest;
  use super::{create_community_apub_name, create_user_apub_name};

  #[rstest]
  #[case("https://lemmy.ml/u/lemmy", String::from("@lemmy@lemmy.ml"))]
  #[case("http://lemmy.ml/u/lemmy", String::from("@lemmy@lemmy.ml"))]
  #[case("http://lemmy.ml:3000/u/lemmy", String::from("@lemmy@lemmy.ml"))]
  #[case("http://localhost/u/lemmy", String::from("@lemmy@localhost"))]
  #[case("http://localhost:8536/u/lemmy", String::from("@lemmy@localhost"))]
  #[case("http://sh.itjust.works/u/lemmy", String::from("@lemmy@sh.itjust.works"))]
  #[case("http://lemmy.ml/post/123", String::new())]
  #[case("https://foo/u/person", String::new())]
  fn formats_user_apub_correctly(#[case] actor_id: &str, #[case] expected: String) {
    assert_eq!(create_user_apub_name(actor_id), expected);
  }

  #[rstest]
  #[case("https://lemmy.ml/c/memes", String::from("!memes@lemmy.ml"))]
  #[case("http://lemmy.ml/c/memes", String::from("!memes@lemmy.ml"))]
  #[case("http://lemmy.ml:3000/c/memes", String::from("!memes@lemmy.ml"))]
  #[case("http://localhost/c/memes", String::from("!memes@localhost"))]
  #[case("http://localhost:8536/c/memes", String::from("!memes@localhost"))]
  #[case("http://sh.itjust.works/c/memes", String::from("!memes@sh.itjust.works"))]
  #[case("http://lemmy.ml/post/123", String::new())]
  #[case("https://foo/c/comm", String::new())]
  fn formats_community_apub_correctly(#[case] actor_id: &str, #[case] expected: String) {
    assert_eq!(create_community_apub_name(actor_id), expected);
  }
}

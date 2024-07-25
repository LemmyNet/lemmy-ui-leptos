use cfg_if::cfg_if;
use std::sync::LazyLock;

pub fn create_user_apub_name(actor_id: &str) -> String {
  create_apub_name::<'@'>(actor_id)
}

pub fn create_community_apub_name(actor_id: &str) -> String {
  create_apub_name::<'!'>(actor_id)
}

fn create_apub_name<const PREFIX: char>(actor_id: &str) -> String {
  const REGEX_STR: &str = r"https?:\/\/(\S+\.\S+)\/[uc]\/(\S+)";
  const ACTOR_ID_PARSE_ERROR_MESSAGE: &str = "Cannot parse actor ID regex";

  cfg_if! {
      if #[cfg(feature = "ssr")] {
        use regex::Regex;
        static INSTANCE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(REGEX_STR)
        .expect("Could not create server side regex."));

        let captures = INSTANCE_REGEX.captures(actor_id).expect(ACTOR_ID_PARSE_ERROR_MESSAGE);
        let instance = &captures[1];
        let name = &captures[2];

        format!("{PREFIX}{name}@{instance}")
      } else {
        use js_sys::RegExp;
        const REGEX_GROUP_NOT_STRING_ERROR_MESSAGE: &str = "Regex capture groups should always be Strings";

        thread_local! {
            static INSTANCE_REGEX: LazyLock<RegExp> = LazyLock::new(|| RegExp::new(REGEX_STR, "gu"));
        }

        INSTANCE_REGEX.with(|instance_regex| {
            let captures = instance_regex.exec(actor_id).expect(ACTOR_ID_PARSE_ERROR_MESSAGE);
            let instance = captures
                .at(1)
                .as_string()
                .expect(REGEX_GROUP_NOT_STRING_ERROR_MESSAGE);
            let name = captures
                .at(2)
                .as_string()
                .expect(REGEX_GROUP_NOT_STRING_ERROR_MESSAGE);

            format!("{PREFIX}{name}@{instance}")
        })
      }
  }
}

// TODO: Figure out how to test functions when targeting wasm
#[cfg(test)]
mod tests {
  use super::{create_community_apub_name, create_user_apub_name};

  #[test]
  fn formats_user_correctly_with_https() {
    let result = create_user_apub_name("https://lemmy.ml/u/lemmy");
    assert_eq!(result, String::from("@lemmy@lemmy.ml"));
  }

  #[test]
  fn formats_user_correctly_with_http() {
    let result = create_user_apub_name("http://lemmy.ml/u/lemmy");
    assert_eq!(result, String::from("@lemmy@lemmy.ml"));
  }

  #[test]
  #[should_panic]
  fn user_panics_with_malformed_actor_id() {
    let _ = create_user_apub_name("http://lemmy.ml/post/123");
  }

  #[test]
  fn formats_community_correctly_with_https() {
    let result = create_community_apub_name("https://lemmy.ml/c/memes");
    assert_eq!(result, String::from("!memes@lemmy.ml"));
  }

  #[test]
  fn formats_community_correctly_with_http() {
    let result = create_community_apub_name("http://lemmy.ml/c/memes");
    assert_eq!(result, String::from("!memes@lemmy.ml"));
  }

  #[test]
  #[should_panic]
  fn community_panics_with_malformed_actor_id() {
    let _ = create_community_apub_name("http://lemmy.ml/post/123");
  }
}

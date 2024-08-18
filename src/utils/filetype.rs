const IMAGE_TYPES: [&str; 6] = ["jpg", "jpeg", "gif", "png", "svg", "webp"];
const VIDEO_TYPES: [&str; 2] = ["mp4", "webm"];

pub fn is_image(url: &str) -> bool {
  is_filetype(url, &IMAGE_TYPES)
}

pub fn is_video(url: &str) -> bool {
  is_filetype(url, &VIDEO_TYPES)
}

fn is_filetype(url: &str, exts: &[&str]) -> bool {
  url
    .split('?')
    .next()
    .and_then(|s| s.rsplit('.').next().map(str::to_lowercase))
    .is_some_and(|ext| exts.iter().any(|file_type| ext.ends_with(file_type)))
}

#[cfg(test)]
mod test {
  use rstest::rstest;
  use crate::utils::{is_image, is_video};

  #[rstest]
  #[case("https://my.test.image.co/keNu2D9.jpg")]
  #[case("https://my.test.image.co/keNu2D9.jpeg")]
  #[case("https://my.test.image.co/keNu2D9.gif")]
  #[case("https://my.test.image.co/keNu2D9.png")]
  #[case("https://my.test.image.co/keNu2D9.svg")]
  #[case("https://my.test.image.co/keNu2D9.webp")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.jpg?format=webp&thumbnail=256")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.jpeg?format=webp&thumbnail=256")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.gif?format=webp&thumbnail=256")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.png?format=webp&thumbnail=256")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.svg?format=webp&thumbnail=256")]
  #[case("https://lemmy.foo.bar.us/pictrs/image/d1821922-855d-47d3-86da-a516d0c0f188.webp?format=webp&thumbnail=256")]
  fn is_image_test(#[case] url: &str) {
    assert!(is_image(url));
  }

  #[rstest]
  #[case("http://lemmy.instance.com")]
  #[case("http://lemmy.instance.com/")]
  #[case("https://foo.bar.xyz/baz/qux.pdf")]
  #[case("https://foo.bar.xyz/baz/qux.txt")]
  #[case("https://foo.bar.xyz/baz/qux.mp4?arg=thing")]
  fn is_not_image_test(#[case] url: &str) {
    assert!(!is_image(url));
  }

  #[rstest]
  #[case("http://vids.inv.pizza/myvid.mp4")]
  #[case("http://vids.inv.pizza/myvid.webm")]
  #[case("http://vids.inv.pizza/myvid.mp4?tracking=plsno")]
  #[case("http://vids.inv.pizza/myvid.webm?tracking=plsno")]
  fn is_video_test(#[case] url: &str) {
    assert!(is_video(url));
  }

  #[rstest]
  #[case("http://lemmy.instance.com")]
  #[case("http://lemmy.instance.com/")]
  #[case("https://foo.bar.xyz/baz/qux.pdf")]
  #[case("https://foo.bar.xyz/baz/qux.txt")]
  #[case("https://foo.bar.xyz/baz/qux.png?arg=thing")]
  fn is_not_video_test(#[case] url: &str) {
    assert!(!is_video(url));
  }
}
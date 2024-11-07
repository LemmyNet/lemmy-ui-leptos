use crate::utils::markdown_to_html;
use leptos::*;

macro_rules! make_element_class {
  ($selector:literal: $($class:literal),+) => {
      const_str::join!(&[$(const_str::concat!("[&_", $selector, "]:", $class),)*], " ")
  };
}

// The class for this is so long that breaking it up into smaller consts makes it easier to deal with.
const BASE_CLASS: &str = "bg-base-200 mt-4 p-5 rounded space-y-4";
const H1_CLASS: &str = make_element_class!("h1": "text-2xl", "font-black");
const H2_CLASS: &str = make_element_class!("h2": "text-2xl", "font-bold");
const H3_CLASS: &str = make_element_class!("h3": "text-xl", "font-black");
const H4_CLASS: &str = make_element_class!("h4": "text-xl", "font-bold");
const H5_CLASS: &str = make_element_class!("h5": "text-lg", "font-black");
const H6_CLASS: &str = make_element_class!("h5": "text-lg", "font-bold");
const MONOSPACE: &str = make_element_class!("code": "bg-neutral", "text-neutral-content", "p-1", "rounded-md", "font-medium", "slashed-zero", "text-sm", "font-medium");

const CLASS: &str = const_str::join!(
  &[BASE_CLASS, H1_CLASS, H2_CLASS, H3_CLASS, H4_CLASS, H5_CLASS, H6_CLASS, MONOSPACE],
  " "
);

#[component]
pub fn MarkdownContent(#[prop(into)] content: TextProp) -> impl IntoView {
  view! {
    <div
      class=CLASS
      inner_html=markdown_to_html(content.get().as_str())
    />
  }
}

#[cfg(test)]
mod test {
  use rstest::rstest;

  #[rstest]
  #[case("[&_h1]:text-2xl [&_h1]:font-black", make_element_class!("h1": "text-2xl", "font-black"))]
  #[case("[&_code]:bg-neutral [&_code]:text-neutral-content [&_code]:p-1", make_element_class!("code": "bg-neutral", "text-neutral-content", "p-1"))]
  fn class_test(#[case] expected: &str, #[case] result: &str) {
    assert_eq!(expected, result);
  }
}

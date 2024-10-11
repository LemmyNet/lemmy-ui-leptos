use crate::utils::markdown_to_html;
use leptos::*;

// The class for this is so long that breaking it up into smaller consts makes it easier to deal with.
const BASE_CLASS: &str = "bg-base-200 mt-4 p-5 rounded space-y-4";
const H1_CLASS: &str = "[&>h1]:text-2xl [&>h1]:font-black";
const H2_CLASS: &str = "[&>h2]:text-2xl [&>h2]:font-bold";
const H3_CLASS: &str = "[&>h3]:text-xl [&>h3]:font-black";
const H4_CLASS: &str = "[&>h4]:text-xl [&>h4]:font-bold";
const H5_CLASS: &str = "[&>h5]:text-lg [&>h5]:font-black";
const H6_CLASS: &str = "[&>h6]:text-lg [&>h6]:font-bold";

const CLASS_ARRAY_SLICE: &[&str] = &[
  BASE_CLASS, H1_CLASS, H2_CLASS, H3_CLASS, H4_CLASS, H5_CLASS, H6_CLASS,
];

#[component]
pub fn MarkdownContent(#[prop(into)] content: TextProp) -> impl IntoView {
  view! {
    <div
      class=const_str::join!(CLASS_ARRAY_SLICE, " ")
      inner_html=markdown_to_html(content.get().as_str())
    />
  }
}

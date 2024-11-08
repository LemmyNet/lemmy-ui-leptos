use crate::utils::markdown_to_html;
use leptos::*;

// The class for this is so long that breaking it up into smaller consts makes it easier to deal with.
const BASE_CLASS: &str = "bg-base-200 mt-4 p-5 rounded space-y-4";
const H1_CLASS: &str = "[&_h1]:text-2xl [&_h1]:font-black";
const H2_CLASS: &str = "[&_h2]:text-2xl [&_h2]:font-bold";
const H3_CLASS: &str = "[&_h3]:text-xl [&_h3]:font-black";
const H4_CLASS: &str = "[&_h4]:text-xl [&_h4]:font-bold";
const H5_CLASS: &str = "[&_h5]:text-lg [&_h5]:font-black";
const H6_CLASS: &str = "[&_h6]:text-lg [&_h6]:font-bold";
const MONOSPACE: &str =
  "[&_code]:font-mono [&_code]:bg-neutral [&_code]:text-neutral-content [&_code]:p-1 [&_code]:rounded-md [&_code]:not-italic [&_code]:font-medium [&_code]:slashed-zero [&_code]:text-sm [&_code]:font-medium";
const BLOCKQUOTE_CLASS: &str = "[&_blockquote]:ps-1.5 [&_blockquote]:py-2 [&_blockquote]:bg-base-300 [&_blockquote]:border-s-4 [&_blockquote]:border-info [&_blockquote]:font-light [&_blockquote]:text-sm [&_blockquote_code]:text-xs [&_blockquote]:italic [&_blockquote>blockquote]:me-4";

const CLASS_ARRAY_SLICE: &[&str] = &[
  BASE_CLASS,
  H1_CLASS,
  H2_CLASS,
  H3_CLASS,
  H4_CLASS,
  H5_CLASS,
  H6_CLASS,
  MONOSPACE,
  BLOCKQUOTE_CLASS,
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

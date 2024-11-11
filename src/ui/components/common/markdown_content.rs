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
const MONOSPACE_CLASS: &str =
  "[&_code]:font-mono [&_code]:bg-neutral [&_code]:text-neutral-content [&_code]:p-1 [&_code]:rounded-md [&_code]:not-italic [&_code]:font-medium [&_code]:slashed-zero [&_code]:text-sm [&_code]:font-medium";
const BLOCKQUOTE_CLASS: &str = "[&_blockquote]:ps-1.5 [&_blockquote]:py-2 [&_blockquote]:bg-base-300 [&_blockquote]:border-s-4 [&_blockquote]:border-info [&_blockquote]:font-light [&_blockquote]:text-sm [&_blockquote_code]:text-xs [&_blockquote]:italic [&_blockquote>blockquote]:me-4";
const LIST_CLASS: &str =
  "[&_ul]:list-disc [&_ul]:list-inside [&_ol]:list-decimal [&_ol]:list-inside [&_li]:my-1.5";
const LINK_CLASS: &str =
  "[&_a]:text-accent [&_a]:font-medium hover:[&_a]:underline hover:[&_a]:underline-offset-2";
const IMG_CLASS: &str = "[&_img[title^=emoji]]:inline [&_img[title^=emoji]]:w-16 [&_img]:max-h-[40vh] [&_img]:max-w-full [&_img]:h-auto";
const HR_CLASS: &str = "[&_hr]:border-secondary";
const TABLE_CLASS: &str =
  "[&_table]:table [&_table]:w-auto [&_table]:mx-auto [&_table]:shadow-lg [&_table]:rounded-md [&_table]:bg-base-100 [&_thead_tr]:bg-base-300 [&_th]:bg-base-max-w-fit [&_td]:bg-base-max-w-fit [&_tbody_tr]:border-t-2 [&_tbody_tr]:border-accent [&_tbody_td:not(:first-child)]:border-accent [&_tbody_td:not(:first-child)]:border-l-2 [&_thead_th:not(:first-child)]:border-accent [&_thead_th:not(:first-child)]:border-l-2";
const SPOLIER_CLASS: &str =
  "[&_summary]:flex [&_summary]:justify-start [&_summary]:cursor-pointer [&_summary]:before:align-middle [&_summary]:before:content-[url(/icons.svg#css-warning)] [&_summary]:before:block [&_summary]:before:size-6 [&_summary]:before:me-1 [&_summary]:after:align-middle [&_summary]:after:content-[url(/icons.svg#css-caret)] [&_summary]:after:block [&_summary]:after:size-6 [&_summary]:after:ms-auto [&_details]:bg-warning [&_details]:text-warning-content [&_details]:p-3 [&_details]:rounded-md [&_details[open]>summary]:mb-2 [&_details[open]>summary]:after:rotate-180";

const CLASS_ARRAY_SLICE: &[&str] = &[
  BASE_CLASS,
  H1_CLASS,
  H2_CLASS,
  H3_CLASS,
  H4_CLASS,
  H5_CLASS,
  H6_CLASS,
  MONOSPACE_CLASS,
  BLOCKQUOTE_CLASS,
  LIST_CLASS,
  LINK_CLASS,
  IMG_CLASS,
  HR_CLASS,
  TABLE_CLASS,
  SPOLIER_CLASS,
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

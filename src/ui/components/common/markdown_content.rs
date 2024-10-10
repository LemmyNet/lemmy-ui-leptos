use crate::utils::markdown_to_html;
use leptos::*;

#[component]
pub fn MarkdownContent(#[prop(into)] content: TextProp) -> impl IntoView {
  view! {
    <div
      class="bg-base-200 mt-4 p-5 rounded space-y-4 [&>h1]:text-3xl [&>h1]:font-bold [&>h2]:text-2xl [&>h2]:font-bold [&>h3]:text-xl [&>h3]:font-bold [&>h4]:text-xl [&>h4]:font-semibold [&>h5]:text-lg [&>h5]:font-bold"
      inner_html=markdown_to_html(content.get().as_str())
    />
  }
}

use crate::utils::markdown_to_html;
use leptos::*;

#[component]
pub fn MarkdownContent(#[prop(into)] content: TextProp) -> impl IntoView {
  view! { <div class="markdown-content" inner_html=markdown_to_html(content.get().as_str()) /> }
}

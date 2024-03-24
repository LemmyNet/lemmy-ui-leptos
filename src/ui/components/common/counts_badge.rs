use leptos::*;

#[component]
pub fn CountsBadge(children: Children) -> impl IntoView {
  view! { <span class="badge badge-neutral inline-block whitespace-nowrap">{children()}</span> }
}

use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListing(cx: Scope, post_view: MaybeSignal<PostView>) -> impl IntoView {
  view! { cx, <div>{post_view().post.name}</div> }
}

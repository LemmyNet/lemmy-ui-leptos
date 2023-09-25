use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;
use leptos_router::A;

#[component]
pub fn PostListing(post_view: MaybeSignal<PostView>) -> impl IntoView {
  let pv = post_view();
  let link = format!("post/{}", pv.post.id);
  view! {
    <A href=link class="link">
      {pv.creator.name}
      -
      {pv.post.name}
    </A>
  }
}

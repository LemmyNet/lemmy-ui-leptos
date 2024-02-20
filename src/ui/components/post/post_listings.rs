use crate::ui::components::post::post_listing::PostListing;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
  view! {
    <table class="table">
      <For each=move || posts.get() key=|pv| pv.post.id let:pv>
        <PostListing post_view=pv.into()/>
      </For>
    </table>
  }
}

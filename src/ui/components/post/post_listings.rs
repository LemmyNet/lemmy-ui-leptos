use crate::ui::components::post::post_listing::PostListing;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(
  posts: MaybeSignal<Vec<PostView>>,
  error: RwSignal<Option<String>>,
) -> impl IntoView {
  view! {
    <table class="table">
      <For each=move || posts.get() key=|pv| pv.post.id let:pv>
        <PostListing post_view=pv.into() error/>
      </For>
    </table>
  }
}

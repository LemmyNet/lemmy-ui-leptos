use crate::ui::components::post::post_listing::PostListing;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
  view! {
    <ul>
      <For each=posts key=|pv| pv.post.id let:pv>
        <li>
          <PostListing post_view=pv.into()/>
        </li>
      </For>

    </ul>
  }
}

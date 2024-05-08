use crate::ui::components::post::post_listing::PostListing;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(#[prop(into)] posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
  view! {
    <ul>
      <For each=move || posts.get() key=|pv| pv.post.id let:pv>
        <li class="[&:not(:first-child)]:border-t-neutral [&:not(:first-child)]:border-t py-4">
          <PostListing post_view=pv/>
        </li>
      </For>
    </ul>
  }
}

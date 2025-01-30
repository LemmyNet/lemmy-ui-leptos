use crate::ui::components::post::post_listing::PostListing;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::prelude::*;

#[component]
pub fn PostListings(posts: Vec<PostView>) -> impl IntoView {
  view! {
    <ul class="divide-y divide-neutral min-h-0 h-fit sm:h-full mb-14 sm:mb-4">
      {posts
        .into_iter()
        .map(|pv| {
          view! {
            <li class="py-4 h-fit">
              <PostListing post_view=pv />
            </li>
          }
        })
        .collect::<Vec<_>>()}
    </ul>
  }
}

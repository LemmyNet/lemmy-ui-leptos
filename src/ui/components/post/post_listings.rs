use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::posts::list_posts,
  ui::components::post::post_listing::PostListing,
  utils::{derive_listing_type, derive_sort_type},
};
use lemmy_client::lemmy_api_common::post::GetPosts;
use leptos::prelude::*;

#[component]
pub fn PostListings() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let listing_type = derive_listing_type(site_resource);
  let sort_type = derive_sort_type(site_resource);

  let posts_resource = Resource::new_blocking(
    move || GetPosts {
      type_: Some(listing_type.get()),
      sort: Some(sort_type.get()),
      limit: Some(20),
      ..Default::default()
    },
    list_posts,
  );

  move || {
    Suspend::new(async move {
      posts_resource.await.map(|posts_response| {
        view! {
          <ul class="divide-y divide-neutral min-h-0 h-fit sm:h-full mb-14 sm:mb-4">
            {posts_response.posts.into_iter().map(|pv| view! {
              <li class="py-4 h-fit">
                <PostListing post_view=pv />
              </li>
            }).collect::<Vec<_>>()}
          </ul>
        }
      })
    })
  }
}

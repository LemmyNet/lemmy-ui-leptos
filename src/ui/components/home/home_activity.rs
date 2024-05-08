use crate::{
  serverfns::list_posts::list_posts,
  ui::components::{
    common::unpack::Unpack,
    home::{site_summary::SiteSummary, trending::Trending},
    post::post_listings::PostListings,
  },
  utils::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  post::GetPosts,
};
use leptos::*;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let listing_type = expect_context::<ReadSignal<ListingType>>();
  let sort_type = expect_context::<ReadSignal<SortType>>();
  let filter_bar = expect_context::<Signal<View>>();

  let posts_resource = create_blocking_resource(
    move || GetPosts {
      type_: Some(listing_type.get()),
      sort: Some(sort_type.get()),
      ..Default::default()
    },
    list_posts,
  );

  let posts = derive_query_signal(posts_resource, |r| r.posts.clone());

  view! {
    <div class="md:container sm:grid sm:grid-cols-5 md:grid-cols-4 sm:grid-rows-1 mx-4 md:mx-auto my-4 sm:gap-20">
      <main class="sm:col-span-3">
        {filter_bar} <Suspense fallback=|| "Loading">
          <ErrorBoundary fallback=|_| { "Could not load posts!" }>
            <Unpack item=posts let:posts>
              <PostListings posts=posts/>
            </Unpack>
          </ErrorBoundary>
        </Suspense>
      </main>

      <aside class="hidden md:block sm:col-span-2 md:col-span-1">
        <Trending/>
        <SiteSummary/>
      </aside>
    </div>
  }
}

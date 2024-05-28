use crate::{
  serverfns::posts::list_posts,
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
pub fn HomePage() -> impl IntoView {
  let listing_type = expect_context::<ReadSignal<ListingType>>();
  let sort_type = expect_context::<ReadSignal<SortType>>();
  let filter_bar = expect_context::<Signal<View>>();

  let posts_resource = create_blocking_resource(
    move || GetPosts {
      type_: Some(listing_type.get()),
      sort: Some(sort_type.get()),
      limit: Some(20),
      ..Default::default()
    },
    list_posts,
  );

  let posts = derive_query_signal(posts_resource, |r| r.posts.clone());

  view! {
    <div class="md:container md:grid md:grid-cols-5 xl:grid-cols-4 md:grid-rows-1 mx-auto my-4 md:gap-20 h-full">
      <main class="md:col-span-3">
        {filter_bar} <h1 class="text-4xl font-bold">Home Feed</h1> <Suspense fallback=|| "Loading">
          <ErrorBoundary fallback=|_| { "Could not load posts!" }>
            <Unpack item=posts let:posts>
              <PostListings posts=posts/>
            </Unpack>
          </ErrorBoundary>
        </Suspense>
      </main>

      <aside class="hidden md:block md:col-span-2 xl:col-span-1">
        <Trending/>
        <SiteSummary/>
      </aside>
    </div>
  }
}

use crate::{
  serverfns::posts::list_posts,
  ui::components::{
    common::unpack::Unpack,
    home::site_summary::SiteSummary,
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
    <div class="md:container md:grid md:grid-cols-5 xl:grid-cols-4 md:grid-rows-1 mx-auto mt-4 mb-1 md:gap-20 h-fit md:h-full">
      <main class="md:col-span-3 flex flex-col mx-2.5 md:mx-0 h-fit md:h-full">
        <div class="flex flex-wrap gap-y-2 gap-x-4 pb-1.5 border-b-4 border-base-300 rounded-b-md">
          <h1 class="text-4xl font-bold text-nowrap">Home Feed</h1>
          {filter_bar}
        </div>
        <Suspense fallback=|| "Loading">
          <ErrorBoundary fallback=|_| { "Could not load posts!" }>
            <Unpack item=posts let:posts>
              <PostListings posts=posts/>
            </Unpack>
          </ErrorBoundary>
        </Suspense>
      </main>

      <aside class="hidden md:block md:col-span-2 xl:col-span-1">
        <SiteSummary/>
      </aside>
    </div>
  }
}

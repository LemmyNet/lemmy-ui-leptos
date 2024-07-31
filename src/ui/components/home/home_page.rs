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
use leptos_fluent::tr;

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
    <div class="flex lg:container mx-auto mt-4 mb-1 lg:gap-12 h-fit lg:h-full">
      <main class="basis-full lg:basis-[65%] xl:basis-3/4 flex flex-col mx-2.5 lg:mx-0 h-fit lg:h-full">
        <div class="flex flex-wrap gap-y-2 gap-x-4 pb-1.5 border-b-4 border-base-300 rounded-b-md">
          <h1 class="text-4xl font-bold text-nowrap">{tr!("home-feed")}</h1>
          {filter_bar}
        </div>
        <Suspense fallback=|| tr!("loading")>
          <ErrorBoundary fallback=|_| { tr!("could-not-load-posts") }>
            <Unpack item=posts let:posts>
              <PostListings posts=posts />
            </Unpack>
          </ErrorBoundary>
        </Suspense>
      </main>

      <aside class="hidden basis-[35%] xl:basis-1/4 lg:block me-8 overflow-y-auto min-h-0">
        <SiteSummary />
      </aside>
    </div>
  }
}

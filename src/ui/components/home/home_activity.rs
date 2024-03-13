use crate::{
  queries::posts_list_query::use_posts,
  ui::components::{
    common::unpack::Unpack,
    home::{site_summary::SiteSummary, trending::Trending},
    post::post_listings::PostListings,
  },
  utils::derive_query_signal::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  post::GetPosts,
};
use leptos::*;
use leptos_query::QueryResult;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let listing_type = expect_context::<Signal<ListingType>>();
  let sort_type = expect_context::<Signal<SortType>>();

  let QueryResult {
    data: list_posts_response,
    ..
  } = use_posts().use_query(move || GetPosts {
    type_: Some(listing_type()),
    sort: Some(sort_type()),
    ..Default::default()
  });

  let posts = derive_query_signal(list_posts_response, |list_posts_response| {
    list_posts_response.posts.clone()
  });

  view! {
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="flex flex-col">
        <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">
          <Suspense fallback=|| "Loading">
            <Unpack item=posts let:posts>
              <PostListings posts=posts/>
            </Unpack>
          </Suspense>
        </div>
      </div>

      <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 hidden lg:block">
        <Trending/>
        <SiteSummary/>
      </div>
    </main>
  }
}

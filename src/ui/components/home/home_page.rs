use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::posts::list_posts,
  ui::components::{
    common::{
      sidebar::{
        sidebar_data::{SidebarData, SiteSidebarData},
        Sidebar,
      },
      unpack::Unpack,
    },
    post::post_listings::PostListings,
  },
  utils::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  post::GetPosts,
};
use leptos::*;
use leptos_fluent::move_tr;

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

  let site_resource = expect_context::<SiteResource>();
  let sidebar_data = derive_query_signal(site_resource, |site_response| {
    SidebarData::Site(SiteSidebarData {
      site: site_response.site_view.site.clone(),
      counts: site_response.site_view.counts,
      admins: site_response
        .admins
        .iter()
        .map(|admin| admin.person.clone())
        .collect(),
    })
  });

  view! {
    <div class="max-w-6xl mx-auto flex-auto flex items-stretch mt-4 mb-1 gap-6 overflow-y-auto">
      <main class="basis-full lg:basis-13/20 xl:basis-7/10 flex flex-col mx-2.5 lg:mx-0 h-fit">
        <div class="flex flex-wrap gap-y-2 gap-x-4 pb-1.5 border-b-4 border-base-300 rounded-b-md">
          <h1 class="text-4xl font-bold text-nowrap">{move_tr!("home-feed")}</h1>
          {filter_bar}
        </div>
        <Suspense fallback=|| move_tr!("loading")>
          <ErrorBoundary fallback=|_| { move_tr!("could-not-load-posts") }>
            <Unpack item=posts let:posts>
              <PostListings posts=posts />
            </Unpack>
          </ErrorBoundary>
        </Suspense>
      </main>

      <aside class="hidden basis-7/20 xl:basis-3/10 lg:block me-8 sticky top-0 h-fit">
        <Transition>
          <Unpack item=sidebar_data let:data>
            <Sidebar data=&data />
          </Unpack>
        </Transition>
      </aside>
    </div>
  }
}

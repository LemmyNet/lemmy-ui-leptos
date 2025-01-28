use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::posts::list_posts,
  ui::components::{
    common::sidebar::{
      sidebar_data::{SidebarData, SiteSidebarData},
      Sidebar,
    },
    post::post_listings::PostListings,
  },
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  post::GetPosts,
  site::GetSiteResponse,
};
use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn HomePage() -> impl IntoView {
  let listing_type = expect_context::<ReadSignal<ListingType>>();
  let sort_type = expect_context::<ReadSignal<SortType>>();
  // TODO: Handle filter bar.
  // let filter_bar = expect_context::<Signal<View>>();

  let posts_resource = Resource::new_blocking(
    move || GetPosts {
      type_: Some(listing_type.get()),
      sort: Some(sort_type.get()),
      limit: Some(20),
      ..Default::default()
    },
    list_posts,
  );

  let site_resource = expect_context::<SiteResource>();

  view! {
    <div class="max-w-screen-2xl mx-auto flex mb-1 gap-6">
      <main class="basis-full lg:basis-13/20 xl:basis-7/10 flex flex-col mx-2.5 lg:mx-0 h-fit">
        <div class="flex flex-wrap gap-y-2 gap-x-4 pb-1.5 pt-4 border-b-4 border-base-300 rounded-b-md sticky top-0 z-10 bg-base-100">
          <h1 class="text-4xl font-bold text-nowrap">{move_tr!("home-feed")}</h1>
        // {filter_bar}
        </div>
        <Suspense fallback=|| move_tr!("loading")>

          <ErrorBoundary fallback=|_| {
            move_tr!("could-not-load-posts")
          }>

            {move || Suspend::new(async move {
              posts_resource
                .await
                .map(|posts_response| {
                  view! { <PostListings posts=posts_response.posts /> }
                })
            })}
          </ErrorBoundary>
        </Suspense>

      </main>

      <aside class="hidden basis-7/20 xl:basis-3/10 lg:block me-8 sticky top-6 h-fit">
        <Transition>
          {move || Suspend::new(async move {
            site_resource
              .await
              .map(|site_response| {
                let GetSiteResponse { site_view, admins, .. } = site_response;
                let sidebar_data = SidebarData::Site(SiteSidebarData {
                  site: site_view.site,
                  counts: site_view.counts,
                  admins: admins.into_iter().map(|admin| admin.person).collect(),
                });

                view! { <Sidebar data=sidebar_data /> }
              })
          })}
        </Transition>
      </aside>
    </div>
  }
}

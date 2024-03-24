use crate::{
  queries::communities_list_query::use_communities_scope,
  ui::components::common::unpack::Unpack,
  utils::derive_query_signal::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  community::*,
  lemmy_db_schema::{ListingType, SortType},
};
use leptos::*;
use leptos_query::{QueryOptions, QueryResult, ResourceOption};
use leptos_router::A;
use std::time::Duration;

#[component]
pub fn Trending() -> impl IntoView {
  let QueryResult {
    data: trending_communities_response,
    ..
  } = use_communities_scope(QueryOptions {
    resource_option: Some(ResourceOption::Blocking),
    stale_time: Some(Duration::from_secs(300)),
    ..Default::default()
  })
  .use_query(|| ListCommunities {
    type_: Some(ListingType::Local),
    sort: Some(SortType::Hot),
    limit: Some(5),
    ..Default::default()
  });

  let trending_communities = derive_query_signal(
    trending_communities_response,
    |trending_communities_response| {
      trending_communities_response
        .communities
        .iter()
        .map(|community_view| {
          (
            community_view.community.id,
            community_view.community.name.clone(),
          )
        })
        .collect::<Vec<_>>()
    },
  );

  view! {
    <div class="card w-full bg-base-300 text-base-content mb-3">
      <figure>
        <div class="card-body bg-info">
          <h2 class="card-title text-info-content">"Trending Communities"</h2>
        </div>
      </figure>
      <div>
        <p>
          // TODO: make better fallbacks for transition and errorboundary
          <Transition fallback=|| "Loading">
            <ErrorBoundary fallback=|_| "Error loading trending communities">
              <Unpack item=trending_communities let:communities>
                <For each=move || communities.clone() key=|c| c.0 let:c>
                  <A
                    class="text-1 font-bold link link-accent whitespace-nowrap"
                    href=format!("/c/{}", c.1.clone())
                  >
                    {c.1.clone()}
                  </A>
                </For>
              </Unpack>
            </ErrorBoundary>
          </Transition>
        </p>
        <A class="btn" href="/create_community">
          "Create a community"
        </A>
        <A class="btn" href="/communities">
          "Explore communities"
        </A>
      </div>
    </div>
  }
}

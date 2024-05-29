use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::layouts::filter_bar_layout::filter_bar::FilterBar,
  use_i18n,
  utils::derive_user_is_logged_in,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{
  source::{local_site::LocalSite, local_user::LocalUser},
  ListingType,
  SortType,
};
use leptos::*;
use leptos_router::{use_query_map, Outlet, A};
use serde::Deserialize;

mod filter_bar;
mod listing_type_link;
mod sort_type_link;

fn derive_link_type<T: for<'a> Deserialize<'a> + Default>(
  key: &'static str,
  get_user_default: impl Fn(&LocalUser) -> T + 'static,
  get_site_default: impl Fn(&LocalSite) -> T + 'static,
) -> Signal<T> {
  let site_resource = expect_context::<SiteResource>();
  let query = use_query_map();

  Signal::derive(move || {
    with!(|site_resource, query| {
      let site_response = site_resource
        .as_ref()
        .and_then(|site_response| site_response.as_ref().ok());

      query
        .get(key)
        .and_then(|value| serde_json::from_str(format!(r#""{value}""#).as_str()).ok())
        .or_else(|| {
          site_response.and_then(|site_response| {
            site_response
              .my_user
              .as_ref()
              .map(|my_user| get_user_default(&my_user.local_user_view.local_user))
          })
        })
        .or_else(|| {
          site_response.map(|site_response| get_site_default(&site_response.site_view.local_site))
        })
        .unwrap_or_default()
    })
  })
}

#[component]
pub fn FilterBarLayout() -> impl IntoView {
  let listing_type = RwSignal::new(ListingType::default());
  let sort_type = RwSignal::new(SortType::default());

  provide_context(listing_type.read_only());
  provide_context(sort_type.read_only());

  let filter_bar = Signal::derive(move || {
    view! {
      <Transition>
        <FilterBar listing_type=listing_type sort_type=sort_type/>
      </Transition>
    }
  });

  provide_context(filter_bar);

  view! { <Outlet/> }
}

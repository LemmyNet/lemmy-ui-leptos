use std::str::FromStr;

use crate::contexts::site_resource_context::SiteResource;
use lemmy_client::lemmy_api_common::lemmy_db_schema::{
  source::{local_site::LocalSite, local_user::LocalUser},
  ListingType, SortType,
};
use leptos::prelude::{Read, Signal};
use leptos_router::hooks::query_signal;

fn derive_link_type<T>(
  site_resource: SiteResource,
  key: &'static str,
  get_user_default: impl Fn(&LocalUser) -> T + Send + Sync + 'static,
  get_site_default: impl Fn(&LocalSite) -> T + Send + Sync + 'static,
) -> Signal<T>
where
  T: Copy + Default + Send + Sync + FromStr + ToString + PartialEq + 'static,
{
  let (query_type, _) = query_signal::<T>(key);

  Signal::derive(move || {
    let site_response = site_resource.read();
    let site_response = site_response
      .as_ref()
      .and_then(|site_response| site_response.as_ref().ok());

    query_type
      .read()
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
}

pub fn derive_sort_type(site_resource: SiteResource) -> Signal<SortType> {
  derive_link_type(
    site_resource,
    "sort",
    |u| u.default_sort_type,
    |s| s.default_sort_type,
  )
}

pub fn derive_listing_type(site_resource: SiteResource) -> Signal<ListingType> {
  derive_link_type(
    site_resource,
    "listingType",
    |u| u.default_listing_type,
    |s| s.default_post_listing_type,
  )
}

use crate::serverfns::get_site::get_site;
use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{create_blocking_resource, provide_context, Resource, ServerFnError};

pub type SiteResource = Resource<(), Result<GetSiteResponse, ServerFnError>>;

pub fn provide_site_resource_context() {
  let site_resource = create_blocking_resource(|| (), |_| get_site());

  provide_context(site_resource);
}

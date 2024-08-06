use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::sidebar::{Aggregates, Sidebar},
  utils::derive_query_signal,
};
use leptos::*;

#[component]
pub fn SiteSidebar() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();

  let site_name = derive_query_signal(site_resource, |site_response| {
    site_response.site_view.site.name.clone()
  });

  let site_description = derive_query_signal(site_resource, |site_response| {
    site_response
      .site_view
      .site
      .description
      .clone()
      .unwrap_or_default()
  });

  let counts = derive_query_signal(site_resource, |site_response| {
    Aggregates::Site(site_response.site_view.counts)
  });

  let admins = derive_query_signal(site_resource, |site_response| {
    site_response
      .admins
      .iter()
      .map(|admin| admin.person.clone())
      .collect::<Vec<_>>()
  });

  view! { <Sidebar name=site_name description=site_description counts=counts team=admins /> }
}

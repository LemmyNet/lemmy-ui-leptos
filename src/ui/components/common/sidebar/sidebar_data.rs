use lemmy_client::lemmy_api_common::lemmy_db_schema::{
  aggregates::structs::{CommunityAggregates, SiteAggregates},
  source::{community::Community, person::Person, site::Site},
};

#[derive(Clone)]
pub struct SiteSidebarData {
  pub site: Site,
  pub admins: Vec<Person>,
  pub counts: SiteAggregates,
}

#[derive(Clone)]
pub struct CommunitySidebarData {
  pub community: Community,
  pub moderators: Vec<Person>,
  pub counts: CommunityAggregates,
}

#[derive(Clone)]
pub enum SidebarData {
  Site(SiteSidebarData),
  Community(CommunitySidebarData),
}

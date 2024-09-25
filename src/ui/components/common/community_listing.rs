use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::create_community_apub_name,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::community::Community;
use leptos::*;
use leptos_router::A;

#[component]
pub fn CommunityListing<'a>(community: &'a Community) -> impl IntoView {
  let community_apub_name =
    create_community_apub_name(&community.name, community.actor_id.inner().as_str());
  let icon = community.icon.as_ref().map(|i| i.inner().to_string());

  view! {
    <div class="flex items-center gap-x-2">
      {icon
        .map_or_else(
          || view! { <Icon icon=IconType::Community /> },
          |icon| view! { <img src=icon class="size-6" /> }.into_view(),
        )} <div>
        <div class="text-sm mb-px font-medium">{&community.title}</div>
        <A href=format!("/c/{}", community.name) class="text-xs block text-secondary font-light">
          {community_apub_name}
        </A>
      </div>
    </div>
  }
}

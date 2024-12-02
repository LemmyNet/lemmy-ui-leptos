use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::create_community_apub_name,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::community::Community;
use leptos::{either::Either, prelude::*};
use leptos_router::components::A;

#[component]
pub fn CommunityListing<'a>(community: &'a Community) -> impl IntoView {
  let community_apub_name =
    create_community_apub_name(&community.name, community.actor_id.inner().as_str());
  let icon = community.icon.as_ref().map(|i| i.inner().to_string());

  view! {
    <div class="flex items-center gap-x-2">
      {icon
        .map_or_else(
          || Either::Left(view! { <Icon icon=IconType::Community /> }),
          |icon| Either::Right(view! { <img src=icon class="size-6" /> }),
        )} <div>
        <div class="text-sm mb-px font-medium">{community.title.clone()}</div>
        <A href=format!("/c/{}", community.name) attr:class="text-xs block text-secondary font-light">
          {community_apub_name}
        </A>
      </div>
    </div>
  }
}

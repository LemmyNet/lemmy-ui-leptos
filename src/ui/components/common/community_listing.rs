use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::create_community_apub_name,
};
use leptos::*;
use leptos_router::A;

#[component]
pub fn CommunityListing(
  icon: Option<String>,
  title: String,
  name: String,
  actor_id: String,
) -> impl IntoView {
  view! {
    <div class="flex items-center gap-x-2">
      {icon
          .map_or_else(
              || view! { <Icon icon=IconType::Community /> },
              |icon| view! { <img src=icon class="size-6" /> }.into_view(),
          )} <div>
        <div class="text-sm mb-px font-medium">{title}</div>
        <A href=format!("/c/{}", name) class="text-xs block text-secondary font-light">
          {create_community_apub_name(&name, &actor_id)}
        </A>
      </div>
    </div>
  }
}

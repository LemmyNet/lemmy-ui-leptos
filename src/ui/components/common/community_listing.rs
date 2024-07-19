use crate::{
  host::get_host,
  ui::components::common::icon::{Icon, IconType},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::community::Community;
use leptos::*;
use leptos_router::A;

#[component]
pub fn CommunityListing(community: Community) -> impl IntoView {
  // TODO: Find less crappy way of handling icon conditional. Maybe when Leptos 0.7 drops.
  view! {
    <div class="flex items-center gap-x-2">
      {if community.icon.is_some() {
          view! {
            <img src=community.icon.clone().map(|url| url.to_string()).unwrap() class="size-8"/>
          }
              .into_view()
      } else {
          view! { <Icon icon=IconType::Community/> }.into_view()
      }}
      <div>
        <div class="text-sm mb-px font-medium">{community.title.clone()}</div>
        <A href=format!("/c/{}", community.name.clone()) class="text-xs block text-secondary font-light">
          {format!("!{}@{}", community.name, get_host())}
        </A>
      </div>
    </div>
  }
}

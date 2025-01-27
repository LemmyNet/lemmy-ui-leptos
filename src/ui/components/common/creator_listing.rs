use crate::{constants::DEFAULT_AVATAR_PATH, utils::create_user_apub_name};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn CreatorListing(#[prop(into)] creator: Signal<Person>) -> impl IntoView {
  let creator = creator.read_untracked();
  let user_apub_name = create_user_apub_name(&creator.name, creator.actor_id.inner().as_str());
  let creator_display_name = creator.display_name.as_ref().unwrap_or(&creator.name);
  let avatar = creator
    .avatar
    .as_deref()
    .map(AsRef::as_ref)
    .map(ToOwned::to_owned)
    .unwrap_or_else(|| DEFAULT_AVATAR_PATH.to_owned());

  view! {
    <div class="flex items-center gap-x-2">
      <img src=avatar class="size-8" />
      <div>
        <div class="text-sm mb-px font-medium">{creator_display_name.clone()}</div>
        <A href=format!("/u/{}", creator.name) attr:class="text-xs block text-primary font-light">
          {user_apub_name}
        </A>
      </div>
    </div>
  }
}

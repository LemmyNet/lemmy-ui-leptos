use crate::utils::create_user_apub_name;
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_router::A;

#[component]
pub fn CreatorListing<'a>(creator: &'a Person) -> impl IntoView {
  let user_apub_name = create_user_apub_name(&creator.name, creator.actor_id.inner().as_str());
  let creator_display_name = creator.display_name.as_ref().unwrap_or(&creator.name);
  let avatar = creator
    .avatar
    .as_ref()
    .map(|url| url.inner().to_string())
    .unwrap_or_else(|| String::from("assets/default-avatar.png"));

  view! {
    <div class="flex items-center gap-x-2">
      <img src=avatar class="size-8" />
      <div>
        <div class="text-sm mb-px font-medium">{creator_display_name}</div>
        <A href=format!("/u/{}", creator.name) class="text-xs block text-primary font-light">
          {user_apub_name}
        </A>
      </div>
    </div>
  }
}

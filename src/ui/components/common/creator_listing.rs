use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_router::A;

use crate::host::get_host;

#[component]
pub fn CreatorListing(creator: Person) -> impl IntoView {
  view! {
    <div class="flex items-center gap-x-2">
      <img src=creator
          .avatar
          .clone()
          .map(|url| url.to_string())
          .unwrap_or_else(|| String::from("assets/default-avatar.png"))
        class="size-8"
          />
        <div>
            <div class="text-sm mb-px font-medium">{creator.display_name.clone().unwrap_or(creator.name.clone())}</div>
            <A href={format!("/u/{}", creator.name.clone())} class="text-xs block text-primary">
                {format!("{}@{}", creator.name, get_host())}
            </A>
        </div>
    </div>
  }
}

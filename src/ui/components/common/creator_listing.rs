use crate::utils::create_user_apub_name;
use leptos::*;
use leptos_router::A;

#[component]
pub fn CreatorListing(
  avatar: Option<String>,
  display_name: Option<String>,
  name: String,
  actor_id: String,
) -> impl IntoView {
  let name = StoredValue::new(name);

  view! {
    <div class="flex items-center gap-x-2">
      <img
        src=avatar
            .as_ref()
            .map(|url| url.to_string())
            .unwrap_or_else(|| String::from("assets/default-avatar.png"))
        class="size-8"
      />
      <div>
        <div class="text-sm mb-px font-medium">
          {display_name.as_ref().map(ToString::to_string).unwrap_or_else(|| name.get_value())}
        </div>
        <A href=format!("/u/{}", name.get_value()) class="text-xs block text-primary font-light">
          {create_user_apub_name(&actor_id)}
        </A>
      </div>
    </div>
  }
}

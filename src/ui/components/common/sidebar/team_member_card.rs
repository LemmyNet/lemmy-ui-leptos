use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_router::A;

#[component]
pub fn TeamMemberCard(person: Person) -> impl IntoView {
  view! {
    <li class="flex-1 text-center max-w-fit rounded-lg p-3 even:bg-base-100 odd:bg-base-300 shadow-lg shadow-neutral">
      <img
        src=person
            .avatar
            .map(|p| p.to_string())
            .unwrap_or(String::from("/assets/default-avatar.png"))

        loading="lazy"
        class="mx-auto size-12"
      />
      <div class="font-medium">{person.display_name.unwrap_or_else(|| person.name.clone())}</div>
      <A href=format!("/u/{}", person.name.clone()) class="text-xs block text-primary font-light">
        {format!("@{}", person.name.clone())}
      </A>
    </li>
  }
}

use crate::ui::components::common::text_input::TextInput;
use leptos::prelude::*;

#[component]
pub fn CommunityForm() -> impl IntoView {
  view! {
    <form class="space-y-3">
      <TextInput
        id="community-name"
        name="community-name"
        label="name"
        required=true
        min_length=3
      ></TextInput>
      <button class="btn btn-lg" type="submit">
        create
      </button>
    </form>
  }
}

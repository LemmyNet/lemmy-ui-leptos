use crate::ui::components::common::text_input::TextInput;
use leptos::prelude::*;
use leptos_fluent::{move_tr, tr};

#[component]
pub fn CommunityForm() -> impl IntoView {
  view! {
    <form class="space-y-3">
      <TextInput
        id="community-name"
        name="community-name"
        label=move || tr!("name")
        required=true
        min_length=3
      ></TextInput>
      <button class="btn btn-lg" type="submit">
        {move_tr!("create")}
      </button>
    </form>
  }
}

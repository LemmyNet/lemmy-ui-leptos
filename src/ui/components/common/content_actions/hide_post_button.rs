use crate::{
  serverfns::posts::HidePostAction,
  ui::components::common::icon::{Icon, IconType},
  utils::{traits::ToStr, types::Hidden},
};
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn HidePostButton(id: i32) -> impl IntoView {
  let hide_post_action = expect_context::<HidePostAction>();
  let hidden = expect_context::<Signal<Hidden>>();
  let icon = Signal::derive(move || {
    if hidden.get().0 {
      IconType::Eye
    } else {
      IconType::EyeSlash
    }
  });

  view! {
    <li>
      <ActionForm action=hide_post_action>
        <input type="hidden" name="id" value=id />
        <input type="hidden" name="hide" value=move || (!hidden.get().0).to_str() />
        <button class="text-xs whitespace-nowrap" type="submit">
          <Icon icon=icon class="inline-block" />
          " "
          {move || if hidden.get().0 { "Unhide post" } else { "Hide post" }}
        </button>
      </ActionForm>
    </li>
  }
}

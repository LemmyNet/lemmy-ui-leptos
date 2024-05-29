use crate::{
  contexts::theme_resource_context::ThemeResource,
  serverfns::theme::create_set_theme_action,
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::types::Theme,
};
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn ThemeSelect() -> impl IntoView {
  let theme_action = create_set_theme_action();
  let theme = expect_context::<ThemeResource>();

  Effect::new(move |_| {
    if theme_action.version().get() > 0 {
      theme.refetch();
    }
  });

  view! {
    <details class="dropdown dropdown-end group">
      <summary class="btn btn-circle btn-ghost relative" aria-label="Theme">
        <Icon class="absolute left-1 inset-y-auto" icon=IconType::Theme/>
        <Icon
          class="absolute right-2.5 bottom-1 group-open:rotate-180 transition-transform"
          icon=IconType::DropdownCaret
          size=IconSize::Small
        />
      </summary>
      <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Dark/>
            <button type="submit" class="p-2.5">
              "Dark"
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Light/>
            <button type="submit" class="p-2.5">
              "Light"
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Retro/>
            <button type="submit" class="p-2.5">
              "Retro"
            </button>
          </ActionForm>
        </li>
      </ul>
    </details>
  }
}

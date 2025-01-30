use crate::{
  contexts::theme_resource_context::ThemeResource,
  serverfns::theme::create_set_theme_action,
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::types::Theme,
};
use leptos::{form::ActionForm, html::Details, prelude::*};
use leptos_fluent::move_tr;
#[cfg(not(feature = "ssr"))]
use leptos_use::on_click_outside;

#[component]
pub fn ThemeSelect() -> impl IntoView {
  let theme_action = create_set_theme_action();
  let theme = expect_context::<ThemeResource>();
  Effect::new(move |_| {
    if theme_action.version().get() > 0 {
      theme.refetch();
    }
  });

  #[allow(unused_variables)]
  let dropdown_node_ref = NodeRef::<Details>::new();
  #[cfg(not(feature = "ssr"))]
  let _ = on_click_outside(dropdown_node_ref, move |_| {
    // Using this approach instead of conditional rendering so that the dropdown works at least somewhat when JS is disabled
    if let Some(el) = dropdown_node_ref.get() {
      use leptos::attr::*;
      let _ = el.attr(Attr(Open, None::<&str>));
    }
  });

  view! {
    <details class="dropdown dropdown-end group" node_ref=dropdown_node_ref>
      <summary class="btn btn-circle btn-ghost relative" aria-label=move_tr!("theme")>
        <Icon class="absolute left-1 inset-y-auto" icon=IconType::Theme />
        <Icon
          class="absolute right-2.5 bottom-1 group-open:rotate-180 transition-transform"
          icon=IconType::DropdownCaret
          size=IconSize::Small
        />
      </summary>
      <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
        <li>
          <ActionForm action=theme_action attr:class="p-0">
            <input type="hidden" name="theme" value=Theme::Dark />
            <button type="submit" class="p-2.5">
              {move_tr!("dark")}
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action attr:class="p-0">
            <input type="hidden" name="theme" value=Theme::Light />
            <button type="submit" class="p-2.5">
              {move_tr!("light")}
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action attr:class="p-0">
            <input type="hidden" name="theme" value=Theme::Retro />
            <button type="submit" class="p-2.5">
              {move_tr!("retro")}
            </button>
          </ActionForm>
        </li>
      </ul>
    </details>
  }
}

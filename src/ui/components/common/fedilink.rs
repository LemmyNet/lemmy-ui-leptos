use crate::ui::components::common::icon::{Icon, IconType};
use leptos::{prelude::*, text_prop::TextProp};
use leptos_fluent::move_tr;

#[component]
pub fn Fedilink(#[prop(into)] href: TextProp) -> impl IntoView {
  let label = move_tr!("fedilink-label");
  view! {
    <a
      href=move || href.get()
      class="hover:animate-color-cycle active:animate-color-cycle focus:animate-color-cycle cursor-pointer"
      title=move || label.get()
      aria-label=label
    >
      <Icon icon=IconType::Fediverse />
    </a>
  }
}

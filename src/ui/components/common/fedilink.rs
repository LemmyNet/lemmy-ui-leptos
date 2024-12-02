use crate::ui::components::common::icon::{Icon, IconType};
use leptos::{prelude::*, text_prop::TextProp};
use leptos_fluent::move_tr;

#[component]
pub fn Fedilink(#[prop(into)] href: TextProp) -> impl IntoView {
  // Need to make this a variable since using a literal makes leptos expect a format string
  let class = "[@media(hover:hover){&:hover}]:animate-color-cycle active:animate-color-cycle focus:animate-color-cycle cursor-pointer";

  let label = move_tr!("fedilink-label");
  view! {
    <a href=href class=class title=label aria-label=label>
      <Icon icon=IconType::Fediverse />
    </a>
  }
}

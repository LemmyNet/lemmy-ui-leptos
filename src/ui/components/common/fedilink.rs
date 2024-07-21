use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;

#[component]
pub fn Fedilink(#[prop(into)] href: TextProp) -> impl IntoView {
  // Need to make this a variable since using a literal makes leptos expect a format string
  let class = "[@media(hover:hover){&:hover}]:animate-color-cycle active:animate-color-cycle focus:animate-color-cycle cursor-pointer";
  view! {
    <a href=href class=class>
      <Icon icon=IconType::Fediverse/>
    </a>
  }
}

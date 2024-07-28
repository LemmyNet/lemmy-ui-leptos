use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::types::Comments,
};
use leptos::*;
use leptos_router::A;

#[component]
pub fn CommentCount(id: i32) -> impl IntoView {
  let comments = expect_context::<Signal<Comments>>();
  let num_comments_label = Signal::derive(move || format!("{} comments", comments.get().0));

  view! {
    <A
      href=move || { format!("/post/{id}") }
      class="text-sm whitespace-nowrap"
      attr:title=num_comments_label
      attr:aria-label=num_comments_label
    >
      <Icon icon=IconType::Comment class="inline align-baseline" />
      " "
      <span class="align-sub">{move || comments.get().0}</span>
    </A>
  }
}

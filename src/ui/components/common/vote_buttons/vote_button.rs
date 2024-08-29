use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::types::{PostOrCommentId, ServerAction, ServerActionFn},
};
use leptos::*;
use leptos_router::ActionForm;
use tailwind_fuse::{tw_merge, AsTailwindClass, TwVariant};

#[derive(TwVariant)]
#[tw(class = "align-bottom disabled:cursor-not-allowed disabled:text-neutral-content")]
pub enum VoteType {
  #[tw(default, class = "text-success")]
  Up,
  #[tw(class = "text-error")]
  Down,
}

#[component]
pub fn VoteButton<VA>(
  vote_action: ServerAction<VA>,
  id: PostOrCommentId,
  is_voted: Signal<bool>,
  user_is_logged_in: Signal<bool>,
  title: Signal<String>,
  icon: IconType,
  vote_value: i8,
  vote_type: VoteType,
) -> impl IntoView
where
  VA: ServerActionFn,
{
  view! {
    <ActionForm action=vote_action>
      <input type="hidden" name="id" value=id.get_id() />
      <input
        type="hidden"
        name="score"
        value=move || with!(| is_voted | if * is_voted { 0 } else { vote_value })
      />
      <button
        type="submit"
        class=move || {
            with!(
                | is_voted | tw_merge!(vote_type.as_class(), (!is_voted).then_some("text-neutral"))
            )
        }

        title=title
        disabled=move || !user_is_logged_in.get() || vote_action.pending().get()
      >

        <Icon icon=icon />
      </button>
    </ActionForm>
  }
}
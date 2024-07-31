use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::icon::{Icon, IconType},
  utils::{
    derive_user_is_logged_in,
    types::{PostOrCommentId, ServerAction, ServerActionFn},
  },
};
use leptos::*;
use leptos_fluent::tr;
use leptos_router::ActionForm;
use tailwind_fuse::{
  AsTailwindClass,
  IntoBuilder,
  IntoTailwindClass,
  TailwindFuse,
  TailwindMerge,
  TwClass,
  TwVariant,
};

#[derive(TwClass)]
#[tw(class = "align-bottom disabled:cursor-not-allowed disabled:text-neutral-content")]
struct VoteBtn {
  vote: Vote,
}

#[derive(TwVariant)]
enum Vote {
  #[tw(default, class = "text-neutral")]
  None,
  #[tw(class = "text-success")]
  Up,
  #[tw(class = "text-error")]
  Down,
}

#[component]
pub fn VoteButtons<VA>(
  my_vote: Signal<Option<i16>>,
  id: PostOrCommentId,
  score: Signal<i64>,
  vote_action: ServerAction<VA>,
) -> impl IntoView
where
  VA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let is_upvote = Signal::derive(move || my_vote.get().unwrap_or_default() == 1);
  let is_downvote = Signal::derive(move || my_vote.get().unwrap_or_default() == -1);

  view! {
    <div class="w-fit">
      <ActionForm action=vote_action>
        <input type="hidden" name="id" value=id.get_id() />
        <input
          type="hidden"
          name="score"
          value=move || with!(| is_upvote | if * is_upvote { 0 } else { 1 })
        />
        <button
          type="submit"
          class=move || {
              with!(
                  | is_upvote | { VoteBtn { vote : if * is_upvote { Vote::Up } else { Vote::None } }
                  .to_class() }
              )
          }

          title=tr!("upvote")
          disabled=move || !user_is_logged_in.get() || vote_action.pending().get()
        >

          <Icon icon=IconType::Upvote />
        </button>
      </ActionForm>
      <div class="text-sm text-center">{score}</div>
      <ActionForm action=vote_action class="w-fit">
        <input type="hidden" name="id" value=id.get_id() />
        <input
          type="hidden"
          name="score"
          value=move || with!(| is_downvote | if * is_downvote { 0 } else { - 1 })
        />
        <button
          type="submit"
          class=move || {
              with!(
                  | is_downvote | { VoteBtn { vote : if * is_downvote { Vote::Down } else {
                  Vote::None } } .to_class() }
              )
          }

          title=tr!("downvote")

          disabled=move || !user_is_logged_in.get() || vote_action.pending().get()
        >
          <Icon icon=IconType::Downvote />
        </button>
      </ActionForm>
    </div>
  }
}

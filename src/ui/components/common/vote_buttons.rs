use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::icon::IconType,
  utils::{
    derive_user_is_logged_in,
    types::{PostOrCommentId, ServerActionFn},
  },
};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use pretty_num::PrettyNumber;
use tailwind_fuse::tw_merge;
use vote_button::{VoteButton, VoteType};

mod vote_button;

#[component]
pub fn VoteButtons<VA>(
  my_vote: Signal<Option<i16>>,
  id: PostOrCommentId,
  score: Signal<i64>,
  vote_action: ServerAction<VA>,
  #[prop(optional)] class: &'static str,
) -> impl IntoView
where
  VA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let is_upvote = Signal::derive(move || my_vote.get().unwrap_or_default() == 1);
  let is_downvote = Signal::derive(move || my_vote.get().unwrap_or_default() == -1);

  view! {
    <div class=tw_merge!("w-fit flex justify-center gap-1.5 flex-row sm:flex-col", class)>
      <VoteButton
        vote_action=vote_action
        id=id
        is_voted=is_upvote
        user_is_logged_in=user_is_logged_in
        title=move_tr!("upvote")
        icon=IconType::Upvote
        vote_value=1
        vote_type=VoteType::Up
      />
      <div class="text-sm text-center font-medium">{move || score.get().pretty_format()}</div>
      <VoteButton
        vote_action=vote_action
        id=id
        is_voted=is_downvote
        user_is_logged_in=user_is_logged_in
        title=move_tr!("downvote")
        icon=IconType::Downvote
        vote_value=-1
        vote_type=VoteType::Down
      />
    </div>
  }
}

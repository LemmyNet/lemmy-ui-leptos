use crate::{
  serverfns::posts::create_vote_post_action,
  ui::components::common::vote_buttons::VoteButtons,
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostVoteButtons(
  #[prop(into)] my_vote: MaybeProp<i16>,
  #[prop(into)] id: MaybeSignal<i32>,
  #[prop(into)] score: MaybeSignal<i64>,
  post_write_signal: WriteSignal<PostView>,
) -> impl IntoView {
  let vote_action = create_vote_post_action();

  Effect::new(move |_| {
    let response = vote_action.value();

    with!(|response| {
      if let Some(response) = response.as_ref().and_then(|r| r.as_ref().ok()) {
        update!(|post_write_signal| {
          post_write_signal.counts.score = response.post_view.counts.score;
          post_write_signal.counts.upvotes = response.post_view.counts.upvotes;
          post_write_signal.counts.downvotes = response.post_view.counts.downvotes;
          post_write_signal.my_vote = response.post_view.my_vote;
        });
      }
    });
  });

  view! { <VoteButtons my_vote=my_vote id=id score=score vote_action=vote_action /> }
}

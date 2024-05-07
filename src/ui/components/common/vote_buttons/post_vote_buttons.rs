use crate::ui::components::common::vote_buttons::VoteButtons;
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    lemmy_db_views::structs::PostView,
    post::{CreatePostLike, PostResponse},
  },
  LemmyRequest,
};
use leptos::*;

#[server(prefix = "/serverfn")]
async fn vote_post(id: PostId, score: i16) -> Result<PostResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .like_post(LemmyRequest {
      body: CreatePostLike { post_id: id, score },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn PostVoteButtons(
  #[prop(into)] my_vote: MaybeProp<i16>,
  #[prop(into)] id: MaybeSignal<i32>,
  #[prop(into)] score: MaybeSignal<i64>,
  post_write_signal: WriteSignal<PostView>,
) -> impl IntoView {
  let vote_action = Action::<VotePost, _>::server();

  Effect::new(move |_| {
    let version = vote_action.version().get();

    if version > 0 {
      vote_action.value().with(|value| {
        let new_post_view = &value.as_ref().unwrap().as_ref().unwrap().post_view;

        update!(|post_write_signal| {
          post_write_signal.counts.score = new_post_view.counts.score;
          post_write_signal.counts.upvotes = new_post_view.counts.upvotes;
          post_write_signal.counts.downvotes = new_post_view.counts.downvotes;
          post_write_signal.my_vote = new_post_view.my_vote;
        });
      });
    }
  });

  view! { <VoteButtons my_vote=my_vote id=id score=score vote_action=vote_action/> }
}

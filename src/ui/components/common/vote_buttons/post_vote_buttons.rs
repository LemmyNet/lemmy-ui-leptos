use crate::{ui::components::common::vote_buttons::VoteButtons, utils::GetJwt};
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
  use crate::utils::get_client_and_session;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

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

  view! { <VoteButtons my_vote=my_vote id=id score=score vote_action=vote_action/> }
}

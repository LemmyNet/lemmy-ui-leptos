use crate::ui::components::common::vote_buttons::VoteButtons;
use lemmy_client::{
  lemmy_api_common::{
    comment::{CommentResponse, CreateCommentLike},
    lemmy_db_schema::newtypes::CommentId,
  },
  LemmyRequest,
};
use leptos::*;

#[server(prefix = "/serverfn")]
async fn vote_comment(id: CommentId, score: i16) -> Result<CommentResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .like_comment(LemmyRequest {
      body: CreateCommentLike {
        comment_id: id,
        score,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn CommentVoteButtons(
  #[prop(into)] my_vote: MaybeProp<i16>,
  #[prop(into)] id: i32,
  #[prop(into)] score: i64,
) -> impl IntoView {
  let vote_action = Action::<VoteComment, _>::server();
  view! { <VoteButtons my_vote=my_vote id=id score=score vote_action=vote_action /> }
}

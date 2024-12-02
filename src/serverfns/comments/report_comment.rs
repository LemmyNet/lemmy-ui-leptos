use lemmy_client::{
  lemmy_api_common::{
    comment::{CommentResponse, CreateCommentReport},
    lemmy_db_schema::newtypes::CommentId,
  },
  LemmyRequest,
};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn report_comment(id: CommentId, reason: String) -> Result<CommentResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .create_comment_report(LemmyRequest {
      body: CreateCommentReport {
        comment_id: id,
        reason,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_report_comment_action() -> ServerAction<ReportComment> {
  ServerAction::new()
}

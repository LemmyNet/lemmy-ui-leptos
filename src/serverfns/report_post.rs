use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    post::{CreatePostReport, PostReportResponse},
  },
  LemmyRequest,
};
use leptos::*;

#[server(prefix = "/serverfn")]
async fn report_post(post_id: PostId, reason: String) -> Result<PostReportResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .report_post(LemmyRequest {
      body: CreatePostReport { post_id, reason },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

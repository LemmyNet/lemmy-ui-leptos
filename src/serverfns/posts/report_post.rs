use lemmy_client::lemmy_api_common::{lemmy_db_schema::newtypes::PostId, post::PostReportResponse};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn report_post(id: PostId, reason: String) -> Result<PostReportResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  use lemmy_client::{lemmy_api_common::post::CreatePostReport, LemmyRequest};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .report_post(LemmyRequest {
      body: CreatePostReport {
        post_id: id,
        reason,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_report_post_action() -> ServerAction<ReportPost> {
  ServerAction::new()
}

use crate::{
  ui::components::common::content_actions::{ContentActionType, ContentActions},
  utils::GetJwt,
};
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    lemmy_db_views::structs::PostView,
    post::{CreatePostReport, PostReportResponse, PostResponse, SavePost as SavePostBody},
  },
  LemmyRequest,
};
use leptos::*;

#[server(prefix = "/serverfn")]
async fn save_post(id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::utils::get_client_and_session;
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .save_post(LemmyRequest {
      body: SavePostBody { post_id: id, save },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(prefix = "/serverfn")]
async fn report_post(post_id: PostId, reason: String) -> Result<PostReportResponse, ServerFnError> {
  use crate::utils::get_client_and_session;
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

#[component]
pub fn PostContentActions(
  #[prop(into)] id: MaybeSignal<i32>,
  #[prop(into)] saved: MaybeSignal<bool>,
  #[prop(into)] creator_id: MaybeSignal<i32>,
  #[prop(into)] comments: MaybeSignal<i64>,
  post_write_signal: WriteSignal<PostView>,
) -> impl IntoView {
  let save_action = Action::<SavePost, _>::server();

  Effect::new(move |_| {
    let response = save_action.value();

    with!(|response| {
      if let Some(response) = response.as_ref().and_then(|r| r.as_ref().ok()) {
        update!(|post_write_signal| {
          post_write_signal.saved = response.post_view.saved;
        });
      }
    });
  });

  let report_action = Action::<ReportPost, _>::server();

  view! {
    <ContentActions
      id=id
      creator_id=creator_id
      saved=saved
      save_action=save_action
      report_action=report_action
      content_action_type=ContentActionType::Post {
          comments,
      }
    />
  }
}

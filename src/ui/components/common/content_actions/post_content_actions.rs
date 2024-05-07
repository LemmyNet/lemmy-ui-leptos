// #[server(prefix = "/serverfn")]
// pub async fn save_post(post_id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
//   use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
//   let (client, session) = get_client_and_session().await?;

//   let jwt = session.get::<String>(AUTH_COOKIE)?;

//   client
//     .save_post(LemmyRequest {
//       body: SavePostBody { post_id, save },
//       jwt,
//     })
//     .await
//     .map_err(|e| ServerFnError::ServerError(e.to_string()))
// }

// #[server(prefix = "/serverfn")]
// pub async fn block_user(
//   person_id: PersonId,
//   block: bool,
// ) -> Result<BlockPersonResponse, ServerFnError> {
//   use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
//   let (client, session) = get_client_and_session().await?;

//   let jwt = session.get::<String>(AUTH_COOKIE)?;

//   client
//     .block_person(LemmyRequest {
//       body: BlockPerson { person_id, block },
//       jwt,
//     })
//     .await
//     .map_err(|e| ServerFnError::ServerError(e.to_string()))
// }

// #[server(prefix = "/serverfn")]
// pub async fn report_post(
//   post_id: PostId,
//   reason: String,
// ) -> Result<PostReportResponse, ServerFnError> {
//   use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
//   let (client, session) = get_client_and_session().await?;

//   let jwt = session.get::<String>(AUTH_COOKIE)?;

//   client
//     .report_post(LemmyRequest {
//       body: CreatePostReport { post_id, reason },
//       jwt,
//     })
//     .await
//     .map_err(|e| ServerFnError::ServerError(e.to_string()))
// }

use actix_session::Session;
use actix_web::web;
use lemmy_client::LemmyClient;
use leptos::{server_fn::error::NoCustomError, ServerFnError};
use leptos_actix::extract;

pub async fn get_client_and_session(
) -> Result<(web::Data<LemmyClient>, Session), ServerFnError<NoCustomError>> {
  let (client, session) = tokio::join!(extract(), extract());
  Ok((client?, session?))
}

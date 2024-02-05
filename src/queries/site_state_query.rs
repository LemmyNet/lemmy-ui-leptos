use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_session::Session;
  use actix_web::web;
  use leptos_actix::extract;

  let session = extract::<Session>().await?;
  let client = extract::<web::Data<awc::Client>>().await?;

  let jwt = session.get::<String>("jwt")?;

  client.get_site(jwt).await.map_err(Into::into)
}

pub fn use_site_state() -> QueryResult<Result<GetSiteResponse, ServerFnError>, impl RefetchFn> {
  use_query(
    || (),
    |_| async move { get_site().await },
    QueryOptions {
      resource_option: ResourceOption::Blocking,
      ..QueryOptions::default()
    },
  )
}

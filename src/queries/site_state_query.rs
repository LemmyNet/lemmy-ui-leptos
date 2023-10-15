use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::lemmy_client::{LemmyClient, LemmyRequest};
  use actix_session::Session;
  use actix_web::web;
  use leptos_actix::extract;

  Ok(
    extract(
      |session: Session, client: web::Data<awc::Client>| async move {
        let jwt = session.get::<String>("jwt")?;

        let res = client.get_site(jwt).await;

        res
      },
    )
    .await??,
  )
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

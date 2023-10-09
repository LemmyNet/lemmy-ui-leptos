use lemmy_api_common::site::GetSiteResponse;
use leptos::*;

#[server(GetSiteResource, "serverfn", "GetJson")]
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

pub type SiteStateContext = Resource<(), Result<GetSiteResponse, ServerFnError>>;

#[component]
pub fn SiteStateProvider(children: ChildrenFn) -> impl IntoView {
  let get_site_resource = create_resource(|| (), |_| async move { get_site().await });
  let children = store_value(children);
  let is_error = Signal::derive(move || {
    !(get_site_resource.loading()() || get_site_resource().is_some_and(|res| res.is_ok()))
  });

  provide_context(get_site_resource);

  view! {
      <Suspense fallback=|| view! { Loading... }>
          <Show
            when=move || !is_error()
            fallback=|| view!{ Error! }
            >
            {with!(|children| children())}
          </Show>
      </Suspense>
  }
}

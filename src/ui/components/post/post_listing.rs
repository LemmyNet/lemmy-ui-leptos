use crate::ui::components::common::{
  content_actions::PostContentActions,
  icon::{Icon, IconSize, IconType},
  vote_buttons::PostVoteButtons,
};
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::{PersonId, PostId},
    lemmy_db_views::structs::*,
    person::*,
    post::{SavePost as SavePostBody, *},
  },
  *,
};
use leptos::*;
use leptos_router::*;

#[server(prefix = "/serverfn")]
pub async fn save_post(post_id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .save_post(LemmyRequest {
      body: SavePostBody { post_id, save },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(prefix = "/serverfn")]
pub async fn block_user(
  person_id: PersonId,
  block: bool,
) -> Result<BlockPersonResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .block_person(LemmyRequest {
      body: BlockPerson { person_id, block },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(prefix = "/serverfn")]
pub async fn report_post(
  post_id: PostId,
  reason: String,
) -> Result<PostReportResponse, ServerFnError> {
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

#[component]
pub fn PostListing(#[prop(into)] post_view: MaybeSignal<PostView>) -> impl IntoView {
  let post_view = RwSignal::new(post_view.get());
  let id = Signal::derive(move || with!(|post_view| post_view.post.id.0));
  let post_name = Signal::derive(move || with!(|post_view| post_view.post.name.clone()));
  let my_vote = Signal::derive(move || with!(|post_view| post_view.my_vote));
  let score = Signal::derive(move || with!(|post_view| post_view.counts.score));
  let url =
    Signal::derive(move || with!(|post_view| post_view.post.url.as_ref().map(ToString::to_string)));
  let thumbnail_url = Signal::derive(move || {
    with!(|post_view| post_view
      .post
      .thumbnail_url
      .as_ref()
      .map(ToString::to_string))
  });

  let creator_id = Signal::derive(move || with!(|post_view| post_view.creator.id.0));
  let creator_name = Signal::derive(move || with!(|post_view| post_view.creator.name.clone()));
  let community_name = Signal::derive(move || with!(|post_view| post_view.community.name.clone()));

  let community_title =
    Signal::derive(move || with!(|post_view| post_view.community.title.clone()));
  let comments = Signal::derive(move || with!(|post_view| post_view.counts.comments));
  let saved = Signal::derive(move || with!(|post_view| post_view.saved));

  let is_on_post_page = use_route().path().starts_with("/post");

  view! {
    <article class="flex gap-x-3 items-center w-fit">
      <PostVoteButtons id=id my_vote=my_vote score=score post_write_signal=post_view.write_only() />
      {move || {
          with!(
              | thumbnail_url, url | thumbnail_url.as_ref().or(url.as_ref()).map(| thumbnail_url |
              view! { < img class = "w-24 aspect-square rounded" src = thumbnail_url /> }
              .into_view()).unwrap_or_else(|| view! { < A href = move || with!(| id |
              format!("/post/{id}")) class = "w-24" > < Icon icon = IconType::Comments class =
              "m-auto" size = IconSize::ExtraLarge /></ A > } .into_view())
          )
      }}

      <div>
        <Show
          when=move || is_on_post_page
          fallback=move || {
              view! {
                <h2 class="text-lg font-medium">
                  <A href=move || with!(| id | format!("/post/{id}"))>{post_name}</A>
                </h2>
              }
          }
        >

          <h1 class="text-xl font-bold">
            <A href=move || with!(| id | format!("/post/{id}"))>{post_name}</A>
          </h1>
        </Show>
        <div>
          <A
            href=move || with!(| creator_name | format!("/u/{creator_name}"))
            class="text-sm inline-block"
          >
            {creator_name}
          </A>
          " to "
          <A
            class="text-sm inline-block"
            href=move || with!(| community_name | format!("/c/{community_name}"))
          >
            {community_title}
          </A>
        </div>

        <PostContentActions
          id=id
          creator_id=creator_id
          saved=saved
          comments=comments
          post_write_signal=post_view.write_only()
        />
      </div>

    </article>
  }
}

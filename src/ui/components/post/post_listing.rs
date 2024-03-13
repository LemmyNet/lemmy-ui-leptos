use crate::ui::components::common::icon::{
  Icon,
  IconType::{Block, Comments, Crosspost, Downvote, Report, Save, Upvote, VerticalDots},
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
pub async fn vote_post(post_id: PostId, score: i16) -> Result<PostResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .like_post(LemmyRequest {
      body: CreatePostLike { post_id, score },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(prefix = "/serverfn")]
pub async fn save_post(post_id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

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
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

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
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

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
  let post_view = Signal::derive(move || post_view.get());
  let id = Signal::derive(move || with!(|post_view| post_view.post.id.0));
  let is_upvote =
    Signal::derive(move || with!(|post_view| post_view.my_vote.unwrap_or_default() == 1));
  let is_downvote =
    Signal::derive(move || with!(|post_view| post_view.my_vote.unwrap_or_default() == -1));
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

  let vote_action = Action::<VotePost, _>::server();

  let save_post_action = create_server_action::<SavePost>();

  let block_user_action = create_server_action::<BlockUser>();

  let report_post_action = create_server_action::<ReportPost>();

  let reason = RwSignal::new(String::new());

  view! {
    <tr class="flex sm:table-row">
      <td class="flex flex-col items-center text-center w-16 hidden sm:table-cell">
        <ActionForm action=vote_action>
          <input type="hidden" name="post_id" value=id/>
          <input
            type="hidden"
            name="score"
            value=move || with!(| is_upvote | if * is_upvote { 0 } else { 1 })
          />
          <button
            type="submit"
            class=move || {
                with!(
                    | is_upvote | { let mut class = String::from("align-bottom"); if * is_upvote {
                    class.push_str(" text-accent"); } class }
                )
            }

            title="Up vote"
          >
            <Icon icon=Upvote/>
          </button>
        </ActionForm>
        <span class="block text-sm">{score}</span>
        <ActionForm action=vote_action>
          <input type="hidden" name="post_id" value=id/>
          <input
            type="hidden"
            name="score"
            value=move || with!(| is_downvote | if * is_downvote { 0 } else { - 1 })
          />
          <button
            type="submit"
            class=move || {
                with!(
                    | is_downvote | { let mut class = String::from("align-top"); if * is_downvote {
                    class.push_str(" text-accent"); } class }
                )
            }

            title="Down vote"
          >
            <Icon icon=Downvote/>
          </button>
        </ActionForm>
      </td>
      <td class=move || {
          with!(
              | thumbnail_url | { let mut class =
              String::from("flex items-center sm:w-28 sm:table-cell"); if thumbnail_url.is_none() {
              class.push_str(" hidden") } class }
          )
      }>

        <A href=move || {
            with!(
                | url, id | url.as_ref().map(ToOwned::to_owned).unwrap_or_else(||
                format!("/post/{id}"))
            )
        }>
          {move || {
              with!(
                  | thumbnail_url | thumbnail_url.as_ref().map(| thumbnail_url | view! { < span
                  class = "block w-24 truncate" > < img class = "w-24" src = thumbnail_url /> </
                  span > })
              )
          }}

        </A>
      </td>
      <td class="w-full">
        <A href=move || with!(| id | format!("/post/{id}")) class="block">
          <span class="text-lg">{move || post_view.get().post.name}</span>
        </A>
        <span class="block">
          <A
            href=move || format!("/u/{}", post_view.get().creator.name)
            class="text-sm inline-block"
          >
            {post_view.get().creator.name}
          </A>
          " to "
          <A class="text-sm inline-block" href=format!("/c/{}", post_view.get().community.name)>
            {post_view.get().community.title}
          </A>
        </span>
        <span class="flex items-center gap-x-2">
          <ActionForm action=vote_action class="flex items-center sm:hidden">
            <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
            <input
              type="hidden"
              name="score"
              value=move || if Some(1) == post_view.get().my_vote { 0 } else { 1 }
            />
            <button
              type="submit"
              class=move || { if Some(1) == post_view.get().my_vote { " text-accent" } else { "" } }
              title="Up vote"
            >
              <Icon icon=Upvote/>
            </button>
          </ActionForm>
          <span class="block text-sm sm:hidden">{move || post_view.get().counts.score}</span>
          <ActionForm action=vote_action class="flex items-center sm:hidden">
            <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
            <input
              type="hidden"
              name="score"
              value=move || if Some(-1) == post_view.get().my_vote { 0 } else { -1 }
            />
            <button
              type="submit"
              class=move || {
                  if Some(-1) == post_view.get().my_vote { " text-accent" } else { "" }
              }

              title="Down vote"
            >
              <Icon icon=Downvote/>
            </button>
          </ActionForm>
          <span
            class="flex items-center"
            title=move || format!("{} comments", post_view.get().unread_comments)
          >
            <A
              href=move || { format!("/post/{}", post_view.get().post.id) }
              class="text-sm whitespace-nowrap"
            >
              <Icon icon=Comments class="inline".into()/>
              " "
              {post_view.get().unread_comments}
            </A>
          </span>
          <ActionForm action=save_post_action class="flex items-center">
            <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
            <input type="hidden" name="save" value=move || format!("{}", !post_view.get().saved)/>
            <button
              type="submit"
              title="Save post"
              class=move || if post_view.get().saved { " text-accent" } else { "" }
            >
              <Icon icon=Save/>
            </button>
          </ActionForm>
          <span title="Cross post">
            <A href="/create_post">
              <Icon icon=Crosspost/>
            </A>
          </span>
          <div class="dropdown hidden sm:block">
            <label tabindex="0">
              <Icon icon=VerticalDots/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                <ActionForm action=report_post_action>
                  <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
                  <input
                    type="text"
                    on:input=move |e| update!(| reason | * reason = event_target_value(& e))
                    name="reason"
                    placeholder="reason"
                  />
                  <button class="text-xs whitespace-nowrap" title="Report post" type="submit">
                    <Icon icon=Report class="inline-block".into()/>
                    " Report post"
                  </button>
                </ActionForm>
              </li>
              <li>
                <ActionForm action=block_user_action>
                  <input
                    type="hidden"
                    name="person_id"
                    value=format!("{}", post_view.get().creator.id.0)
                  />
                  <input type="hidden" name="block" value="true"/>
                  <button class="text-xs whitespace-nowrap" title="Block user" type="submit">
                    <Icon icon=Block class="inline-block".into()/>
                    " Block user"
                  </button>
                </ActionForm>
              </li>
            </ul>
          </div>
        </span>
      </td>
    </tr>
  }
}

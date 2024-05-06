use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::vote_buttons::PostVoteButtons,
  utils::derive_user_is_logged_in,
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
use phosphor_leptos::{Bookmark, ChatText, DotsThreeVertical, Flag, IntersectThree, Prohibit};

#[server(prefix = "/serverfn")]
pub async fn save_post(post_id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
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
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
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
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
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
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

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
  let unread_comments = Signal::derive(move || with!(|post_view| post_view.unread_comments));
  let saved = Signal::derive(move || with!(|post_view| post_view.saved));

  let save_post_action = Action::<SavePost, _>::server();
  Effect::new_isomorphic(move |_| {
    let version = save_post_action.version().get();

    if version > 0 {
      save_post_action.value().with(|value| {
        let new_post_view = &value.as_ref().unwrap().as_ref().unwrap().post_view;

        update!(|post_view| {
          post_view.saved = new_post_view.saved;
        });
      });
    }
  });

  let block_user_action = Action::<BlockUser, _>::server();

  let report_post_action = Action::<ReportPost, _>::server();

  let reason = RwSignal::new(String::new());

  view! {
    <article>
      <PostVoteButtons id=id my_vote=my_vote score=score post_write_signal=post_view.write_only()/>
      // <div >

      //   <A href=move || {
      //       with!(
      //           | url, id | url.as_ref().map(ToOwned::to_owned).unwrap_or_else(||
      //           format!("/post/{id}"))
      //       )
      //   }>
      //     {move || {
      //         with!(
      //             | thumbnail_url | thumbnail_url.as_ref().map(| thumbnail_url | view! { < span
      //             class = "block w-24 truncate" > < img class = "w-24" src = thumbnail_url /> </
      //             span > })
      //         )
      //     }}

      //   </A>
      // </div>
      <div>
        <A href=move || with!(| id | format!("/post/{id}")) class="block text-lg">
          {post_name}
        </A>
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
        <div class="flex items-center gap-x-2">
          <span
            class="flex items-center"
            title=move || format!("{} comments", unread_comments.get())
          >
            <A href=move || { format!("/post/{}", id.get()) } class="text-sm whitespace-nowrap">
              <ChatText class="size-6 inline"/>
              " "
              {unread_comments}
            </A>
          </span>
          <ActionForm action=save_post_action class="flex items-center">
            <input type="hidden" name="post_id" value=id/>
            <input type="hidden" name="save" value=saved/>
            <button
              type="submit"
              title="Save post"
              class=move || if post_view.get().saved { " text-accent" } else { "" }
              disabled=move || !user_is_logged_in.get() || save_post_action.pending().get()
            >
              <Bookmark class="size-6"/>
            </button>
          </ActionForm>
          <span title="Cross post">
            <A href="/create_post">
              <IntersectThree class="size-6"/>
            </A>
          </span>
          <div class="dropdown hidden sm:block">
            <label tabindex="0">
              <DotsThreeVertical class="size-6"/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                <ActionForm action=report_post_action>
                  <input type="hidden" name="post_id" value=id/>
                  <input
                    type="text"
                    on:input=move |e| update!(| reason | * reason = event_target_value(& e))
                    name="reason"
                    placeholder="reason"
                  />
                  <button class="text-xs whitespace-nowrap" title="Report post" type="submit">
                    <Flag class="size-6 inline-block"/>
                    " Report post"
                  </button>
                </ActionForm>
              </li>
              <li>
                <ActionForm action=block_user_action>
                  <input type="hidden" name="person_id" value=creator_id/>
                  <input type="hidden" name="block" value="true"/>
                  <button class="text-xs whitespace-nowrap" title="Block user" type="submit">
                    <Prohibit class="size-6 inline-block"/>
                    " Block user"
                  </button>
                </ActionForm>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </article>
  }
}

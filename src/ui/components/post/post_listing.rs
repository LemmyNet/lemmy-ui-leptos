use lemmy_api_common::{
  lemmy_db_schema::newtypes::*, //{PersonId, PostId},
  lemmy_db_views::structs::*,   //PostView,
  person::*,                    //{BlockPerson, BlockPersonResponse},
  post::*, //{CreatePostLike, CreatePostReport, PostReportResponse, PostResponse, SavePost},
};
use leptos::*;
use leptos_icons::*;
use leptos_router::*; //{ActionForm, A};

#[server(VotePostFn, "/serverfn")]
pub async fn vote_post_fn(post_id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_web::web;
  use leptos_actix::extract;

  let form = CreatePostLike {
    post_id: PostId(post_id),
    score,
  };

  Ok(extract(|client: web::Data<awc::Client>| async move { client.like_post(form).await }).await??)
}

#[server(SavePostFn, "/serverfn")]
pub async fn save_post_fn(post_id: i32, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_web::web;
  use leptos_actix::extract;

  let form = SavePost {
    post_id: PostId(post_id),
    save,
  };

  Ok(extract(|client: web::Data<awc::Client>| async move { client.save_post(form).await }).await??)
}

#[server(BlockUserFn, "/serverfn")]
pub async fn block_user_fn(
  person_id: i32,
  block: bool,
) -> Result<BlockPersonResponse, ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_web::web;
  use leptos_actix::extract;

  let form = BlockPerson {
    person_id: PersonId(person_id),
    block,
  };

  Ok(
    extract(|client: web::Data<awc::Client>| async move { client.block_user(form).await })
      .await??,
  )
}

#[server(ReportPostFn, "/serverfn")]
pub async fn report_post_fn(
  post_id: i32,
  reason: String,
) -> Result<PostReportResponse, ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_web::web;
  use leptos_actix::extract;

  let form = CreatePostReport {
    post_id: PostId(post_id),
    reason,
  };

  Ok(
    extract(|client: web::Data<awc::Client>| async move { client.report_post(form).await })
      .await??,
  )
}

#[component]
pub fn PostListing(
  post_view: MaybeSignal<PostView>,
  error: RwSignal<Option<String>>,
) -> impl IntoView {
  let post_view = create_rw_signal(post_view().clone());

  let vote_action = create_server_action::<VotePostFn>();

  create_effect(move |_| {
    error.set(None);
    match vote_action.value().get() {
      None => {}
      Some(Ok(o)) => {
        post_view.set(o.post_view);
      }
      Some(Err(e)) => {
        error.set(Some(e.to_string()));
      }
    }
  });

  let save_post_action = create_server_action::<SavePostFn>();

  create_effect(move |_| {
    error.set(None);
    match save_post_action.value().get() {
      None => {}
      Some(Ok(o)) => {
        post_view.set(o.post_view);
      }
      Some(Err(e)) => {
        error.set(Some(e.to_string()));
      }
    }
  });

  let block_user_action = create_server_action::<BlockUserFn>();

  create_effect(move |_| {
    error.set(None);
    match block_user_action.value().get() {
      None => {}
      Some(Ok(_o)) => {}
      Some(Err(e)) => {
        error.set(Some(e.to_string()));
      }
    }
  });

  let report_post_action = create_server_action::<BlockUserFn>();

  create_effect(move |_| {
    error.set(None);
    match report_post_action.value().get() {
      None => {}
      Some(Ok(_o)) => {}
      Some(Err(e)) => {
        error.set(Some(e.to_string()));
      }
    }
  });

  view! {
    <tr>
      <td class="flex flex-col text-center">
        <ActionForm action=vote_action>
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input
            type="hidden"
            name="score"
            value=move || if Some(1) == post_view().my_vote { 0 } else { 1 }
          />
          <button
            type="submit"
            class=move || if Some(1) == post_view().my_vote { " text-accent" } else { "" }
            title="Up vote"
          >
            <Icon icon=Icon::from(ChIcon::ChArrowUp) class="h-6 w-6"/>
          </button>
        </ActionForm>
        <span class="block text-sm">{move || post_view().counts.score}</span>
        <ActionForm action=vote_action>
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input
            type="hidden"
            name="score"
            value=move || if Some(-1) == post_view().my_vote { 0 } else { -1 }
          />
          <button
            type="submit"
            class=move || if Some(-1) == post_view().my_vote { " text-accent" } else { "" }
            title="Down vote"
          >
            <Icon icon=Icon::from(ChIcon::ChArrowDown) class="h-6 w-6"/>
          </button>
        </ActionForm>
      </td>
      <td>

        {move || {
            if let Some(d) = post_view().post.url {
                let u = d.inner().to_string();
                view! {
                  <span>
                    <a href=u>{move || format!("{:#?}", post_view().post.thumbnail_url)}</a>
                  </span>
                }
            } else {
                view! { <span>{move || format!("{:#?}", post_view().post.thumbnail_url)}</span> }
            }
        }}

      </td>
      <td>
        <A href=move || format!("/post/{}", post_view().post.id) class="block">
          <span class="text-lg">{move || post_view().post.name}</span>
        </A>
        <span class="block">
          <A href=move || format!("/u/{}", post_view().creator.name) class="text-sm inline-block">
            {post_view().creator.name}
          </A>
          " to "
          <A class="text-sm inline-block" href=format!("/c/{}", post_view().community.name)>
            {post_view().community.title}
          </A>
        </span>
        <span class="block">
          <span title=move || format!("{} comments", post_view().unread_comments)>
            <A
              href=move || format!("/post/{}?scrollToComments=true", post_view().post.id)
              class="text-xs inline-block whitespace-nowrap align-top"
            >

              <Icon icon=Icon::from(ChIcon::ChNotes) class="h-6 w-6 inline-block"/>
              " "
              {post_view().unread_comments}
            </A>
          </span>
          <ActionForm action=save_post_action class="inline-block align-top">
            <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
            <input type="hidden" name="save" value=move || format!("{}", !post_view().saved)/>
            <button
              type="submit"
              title="Save post"
              class=move || if post_view().saved { " text-accent" } else { "" }
            >
              <Icon icon=Icon::from(ChIcon::ChStar) class="h-6 w-6"/>
            </button>
          </ActionForm>
          <span title="Cross post">
            <A href="/create_post" class="inline-block align-top">
              <Icon icon=Icon::from(ChIcon::ChCopy) class="h-6 w-6"/>
            </A>
          </span>

          <div class="dropdown inline-block align-top">
            <label tabindex="0">
              <Icon icon=Icon::from(ChIcon::ChMenuKebab) class="h-6 w-6"/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                <ActionForm action=report_post_action>
                  <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
                  <input
                    class="input input-bordered"
                    type="text"
                    name="reason"
                    placeholder="reason"
                  />
                  <button title="Report post" type="submit">
                    <Icon icon=Icon::from(ChIcon::ChFlag) class="h-6 w-6"/>
                    "Report post"
                  </button>
                </ActionForm>
              </li>
              <li>
                <ActionForm action=block_user_action class="inline-block">
                  <input
                    type="hidden"
                    name="person_id"
                    value=format!("{}", post_view().creator.id.0)
                  />
                  <input type="hidden" name="block"/>
                  <button title="Block user" type="submit">
                    <Icon icon=Icon::from(ChIcon::ChBlock) class="h-6 w-6"/>
                    "Block user"
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

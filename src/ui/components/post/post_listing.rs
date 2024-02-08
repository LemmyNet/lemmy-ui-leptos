use crate::ui::components::common::icon::{
  Icon,
  IconType::{Block, Crosspost, Downvote, Report, Save, Upvote, VerticalDots},
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::newtypes::*, //{PersonId, PostId},
  lemmy_db_views::structs::*,   //PostView,
  person::*,                    //{BlockPerson, BlockPersonResponse},
  post::*, //{CreatePostLike, CreatePostReport, PostReportResponse, PostResponse, SavePost},
};
use leptos::*;
use leptos_router::*;

// #[server(VotePostFn, "/serverfn")]
// pub async fn vote_post_fn(post_id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = CreatePostLike {
//     post_id: PostId(post_id),
//     score,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.like_post(form).await.map_err(Into::into)
// }

// #[server(SavePostFn, "/serverfn")]
// pub async fn save_post_fn(post_id: i32, save: bool) -> Result<PostResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = SavePost {
//     post_id: PostId(post_id),
//     save,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.save_post(form).await.map_err(Into::into)
// }

// #[server(BlockUserFn, "/serverfn")]
// pub async fn block_user_fn(
//   person_id: i32,
//   block: bool,
// ) -> Result<BlockPersonResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = BlockPerson {
//     person_id: PersonId(person_id),
//     block,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.block_user(form).await.map_err(Into::into)
// }

// #[server(ReportPostFn, "/serverfn")]
// pub async fn report_post_fn(
//   post_id: i32,
//   reason: String,
// ) -> Result<PostReportResponse, ServerFnError> {
//   use actix_web::web;
//   use leptos_actix::extract;

//   let form = CreatePostReport {
//     post_id: PostId(post_id),
//     reason,
//   };
//   let client = extract::<web::Data<awc::Client>>().await?;
//   client.report_post(form).await.map_err(Into::into)
// }

#[component]
pub fn PostListing(
  post_view: MaybeSignal<PostView>,
  error: RwSignal<Option<String>>,
) -> impl IntoView {
  let post_view = create_rw_signal(post_view.get());

  // let vote_action = create_server_action::<VotePostFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match vote_action.value().get() {
  //     None => {}
  //     Some(Ok(o)) => {
  //       post_view.set(o.post_view);
  //     }
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  // let save_post_action = create_server_action::<SavePostFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match save_post_action.value().get() {
  //     None => {}
  //     Some(Ok(o)) => {
  //       post_view.set(o.post_view);
  //     }
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  // let block_user_action = create_server_action::<BlockUserFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match block_user_action.value().get() {
  //     None => {}
  //     Some(Ok(_o)) => {}
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  // let report_post_action = create_server_action::<BlockUserFn>();

  // create_effect(move |_| {
  //   error.set(None);
  //   match report_post_action.value().get() {
  //     None => {}
  //     Some(Ok(_o)) => {}
  //     Some(Err(e)) => {
  //       error.set(Some(e.to_string()));
  //     }
  //   }
  // });

  view! {
    <tr>
      // <td class="flex flex-col text-center">
      // <ActionForm action=vote_action>
      // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
      // <input
      // type="hidden"
      // name="score"
      // value=move || if Some(1) == post_view.get().my_vote { 0 } else { 1 }
      // />
      // <button
      // type="submit"
      // class=move || { if Some(1) == post_view.get().my_vote { " text-accent" } else { "" } }

      // title="Up vote"
      // >
      // <Icon icon=Upvote/>
      // </button>
      // </ActionForm>
      // <span class="block text-sm">{move || post_view.get().counts.score}</span>
      // <ActionForm action=vote_action>
      // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
      // <input
      // type="hidden"
      // name="score"
      // value=move || if Some(-1) == post_view.get().my_vote { 0 } else { -1 }
      // />
      // <button
      // type="submit"
      // class=move || { if Some(-1) == post_view.get().my_vote { " text-accent" } else { "" } }

      // title="Down vote"
      // >
      // <Icon icon=Downvote/>
      // </button>
      // </ActionForm>
      // </td>
      <td>

        {move || {
            if let Some(d) = post_view.get().post.url {
                let u = d.inner().to_string();
                view! {
                  <span>
                    <a href=u>{move || format!("{:#?}", post_view.get().post.thumbnail_url)}</a>
                  </span>
                }
            } else {
                view! {
                  <span>{move || format!("{:#?}", post_view.get().post.thumbnail_url)}</span>
                }
            }
        }}

      </td>
      <td>
        <A href=move || format!("/post/{}", post_view.get().post.id) class="block">
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
        <span class="block">
          <span title=move || format!("{} comments", post_view.get().unread_comments)>
            <A
              href=move || { format!("/post/{}?scrollToComments=true", post_view.get().post.id) }

              class="text-xs inline-block whitespace-nowrap align-top"
            >
              {post_view.get().unread_comments}
            </A>
          </span>
          // <ActionForm action=save_post_action class="inline-block align-top">
          // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
          // <input type="hidden" name="save" value=move || format!("{}", !post_view.get().saved)/>
          // <button
          // type="submit"
          // title="Save post"
          // class=move || if post_view.get().saved { " text-accent" } else { "" }
          // >
          // <Icon icon=Save/>
          // </button>
          // </ActionForm>
          <span title="Cross post">
            <A href="/create_post" class="inline-block align-top">
              <Icon icon=Crosspost/>
            </A>
          </span>

          <div class="dropdown inline-block align-top">
            <label tabindex="0">
              <Icon icon=VerticalDots/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                // <ActionForm action=report_post_action>
                // <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
                // <input
                // class="input input-bordered"
                // type="text"
                // name="reason"
                // placeholder="reason"
                // />
                // <button title="Report post" type="submit">
                // <Icon icon=Report/>
                // "Report post"
                // </button>
                // </ActionForm>
                placeholder
              </li>
              // <ActionForm action=block_user_action class="inline-block">
              // <input
              <li>placeholder// type="hidden"
              // name="person_id"
              // value=format!("{}", post_view.get().creator.id.0)
              // />
              // <input type="hidden" name="block"/>
              // <button title="Block user" type="submit">
              // <Icon icon=Block/>
              // "Block user"
              // </button>
              // </ActionForm>
              </li>
            </ul>
          </div>
        </span>
      </td>
    </tr>
  }
}

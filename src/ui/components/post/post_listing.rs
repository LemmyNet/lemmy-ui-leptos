use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
};
use lemmy_api_common::{
  lemmy_db_schema::{
    newtypes::{PersonId, PostId},
    source::post_report::PostReport,
  },
  lemmy_db_views::structs::PostView,
  person::{BlockPerson, BlockPersonResponse},
  post::{
    CreatePostLike,
    CreatePostReport,
    GetPost,
    GetPostResponse,
    GetPosts,
    GetPostsResponse,
    PostReportResponse,
    PostResponse,
    SavePost,
  },
};
use leptos::{logging::log, *};
use leptos_icons::*;
use leptos_router::{ActionForm, A};

pub async fn report_post(form: &CreatePostReport) -> Result<PostReportResponse, LemmyAppError> {
  api_wrapper::<PostReportResponse, CreatePostReport>(HttpType::Post, "post/report", form).await
}

pub async fn block_user(form: &BlockPerson) -> Result<BlockPersonResponse, LemmyAppError> {
  api_wrapper::<BlockPersonResponse, BlockPerson>(HttpType::Post, "user/block", form).await
}

pub async fn save_post(form: &SavePost) -> Result<PostResponse, LemmyAppError> {
  api_wrapper::<PostResponse, SavePost>(HttpType::Put, "post/save", form).await
}

pub async fn like_post(form: &CreatePostLike) -> Result<PostResponse, LemmyAppError> {
  api_wrapper::<PostResponse, CreatePostLike>(HttpType::Post, "post/like", form).await
}

#[server(VotePostFn, "/serverfn")]
pub async fn vote_post_fn(post_id: i32, score: i16) -> Result<PostResponse, ServerFnError> {
  let c = CreatePostLike {
    post_id: PostId(post_id),
    score,
  };

  let thing = like_post(&c).await;

  match thing {
    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    Ok(p) => Ok(p),
  }
}

#[server(SavePostFn, "/serverfn")]
pub async fn save_post_fn(post_id: i32, save: bool) -> Result<PostResponse, ServerFnError> {
  let form = SavePost {
    post_id: PostId(post_id),
    save,
  };

  let save_result = save_post(&form).await;

  match save_result {
    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    Ok(p) => Ok(p),
  }
}

#[server(BlockUserFn, "/serverfn")]
pub async fn block_user_fn(
  person_id: i32,
  block: bool,
) -> Result<BlockPersonResponse, ServerFnError> {
  let form = BlockPerson {
    person_id: PersonId(person_id),
    block,
  };

  let save_result = block_user(&form).await;

  match save_result {
    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    Ok(p) => Ok(p),
  }
}

#[server(ReportPostFn, "/serverfn")]
pub async fn report_post_fn(
  post_id: i32,
  reason: String,
) -> Result<PostReportResponse, ServerFnError> {
  let form = CreatePostReport {
    post_id: PostId(post_id),
    reason,
  };

  let save_result = report_post(&form).await;

  match save_result {
    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    Ok(p) => Ok(p),
  }
}

#[component]
pub fn PostListing(post_view: MaybeSignal<PostView>) -> impl IntoView {
  let pv = post_view().clone();
  let link = format!("post/{}", pv.post.id);

  let error = create_rw_signal::<Option<String>>(None);

  let post_view = create_rw_signal(pv.clone());

  let vote_action = create_server_action::<VotePostFn>();

  create_effect(move |_| match vote_action.value().get() {
    None => {}
    Some(Ok(o)) => {
      post_view.set(o.post_view);
    }
    Some(Err(e)) => {
      error.set(Some(e.to_string()));
    }
  });

  let save_post_action = create_server_action::<SavePostFn>();

  create_effect(move |_| match save_post_action.value().get() {
    None => {}
    Some(Ok(o)) => {
      post_view.set(o.post_view);
    }
    Some(Err(e)) => {
      error.set(Some(e.to_string()));
    }
  });

  let block_user_action = create_server_action::<BlockUserFn>();

  let report_post_action = create_server_action::<BlockUserFn>();

  // let clicky = move |_| {
  //   spawn_local(async move {
  //     let c = CreatePostLike {
  //       post_id: post_view().post.id,
  //       score: 1,
  //     };

  //     let thing = like_post(&c).await;

  //     match thing {
  //       Err(e) => {
  //         log!("should do {:#?}", e);
  //         // Err(ServerFnError::ServerError(e.to_string()))
  //       }
  //       Ok(p) => {
  //         log!("should do {:#?}", p);
  //         // Ok(p),
  //       }
  //     }
  //   });
  // };

  // let is_upvote = move || {
  //   match post_view().my_vote {
  //     Some(1) => {
  //       true
  //     },
  //     Some(-1) => {
  //       false
  //     },
  //     Some(0) => {
  //       false
  //     },
  //     None => {
  //       false
  //     },
  //     _ => {
  //       log!("error");
  //       false
  //     },
  //   }
  // };

  // let thing = move || {
  //   if let Some(d) = post_view().post.url {
  //     d.inner().to_string()
  //   } else {
  //     "#".to
  //   }
  // };

  view! {
    <tr>
      <td class="flex flex-col text-center">

        {move || {
            error
                .get()
                .map(|err| {
                    view! {
                      <div class="alert alert-error">
                        <span>{err}</span>
                      </div>
                    }
                })
        }}
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
          >
            <Icon icon=Icon::from(ChIcon::ChArrowUp) class="h-6 w-6"/>
          </button>
        </ActionForm>
        // <button on:click=clicky><Icon icon=Icon::from(ChIcon::ChNotes) class="h-6 w-6"/></button>
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
        <A
          href=move || format!("post/{}", post_view().post.id)
          class="block"
        >
          <span class="text-lg">{move || post_view().post.name}</span>
        </A>
        <span class="block">
          <A
            href=move || format!("/u/{}", post_view().creator.name)
            class="text-sm inline-block"
          >
            {post_view().creator.name}
          </A>
          " to "
          <A class="text-sm inline-block" href=format!("/c/{}", post_view().community.name)>{post_view().community.title}</A>
        </span>
        <span class="block">
          <A
            href=move || format!("post/{}?scrollToComments=true", post_view().post.id)
            class="text-xs inline-block whitespace-nowrap"
          >
            <Icon icon=Icon::from(ChIcon::ChNotes) class="h-6 w-6 inline-block"/>
            " "
            {post_view().unread_comments}
          </A>
          <ActionForm action=save_post_action class="inline-block align-bottom">
            <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
            <input
              type="hidden"
              name="save"
              value=move || format!("{}", if post_view().saved { false } else { true })
            />
            <button type="submit" class=move || if post_view().saved { " text-accent" } else { "" }>
              <Icon icon=Icon::from(ChIcon::ChStar) class="h-6 w-6 align-bottom"/>
            </button>
          </ActionForm>
          <A href="/create_post" class="inline-block align-bottom">
            <Icon icon=Icon::from(ChIcon::ChCopy) class="h-6 w-6"/>
          </A>
          <a href="#" class="inline-block align-bottom">
            <Icon icon=Icon::from(ChIcon::ChMenuKebab) class="h-6 w-6"/>
          </a>
          <ActionForm action=report_post_action class="inline-block align-bottom">
            <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
            <input class="input input-bordered" type="text" name="reason" placeholder="reason"/>
            <button type="submit">
              <Icon icon=Icon::from(ChIcon::ChFlag) class="h-6 w-6 align-bottom"/>
            </button>
          </ActionForm>
          <ActionForm action=block_user_action class="inline-block align-bottom">
            <input type="hidden" name="person_id" value=format!("{}", post_view().creator.id.0)/>
            <input
              type="hidden"
              name="block"
              // refresh page? blank hing  check voyager
              // value=move || format!("{}", if post_view().saved { false } else { true })
            />
            <button type="submit">
              <Icon icon=Icon::from(ChIcon::ChBlock) class="h-6 w-6"/>
            </button>
          </ActionForm>
        </span>
      </td>
    </tr>
  }
}

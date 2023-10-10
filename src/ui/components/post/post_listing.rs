use leptos_icons::*;
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

// pub async fn list_posts(form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
//   api_wrapper::<GetPostsResponse, GetPosts>(HttpType::Get, "post/list", form).await
// }

// pub async fn get_post(form: &GetPost) -> Result<GetPostResponse, LemmyAppError> {
//   api_wrapper::<GetPostResponse, GetPost>(HttpType::Get, "post", form).await
// }

// pub async fn get_content_head(form: &String) -> Result<GetPostResponse, LemmyAppError> {
//   api_wrapper::<GetPostResponse, String>(HttpType::Head, form, form).await
// }

// #[server(DownVote, "/serverfn")]
// pub async fn down_vote(auth: String, post_id: i32, score: i16) -> Result<PostResponse, ServerFnError> {

//   use crate::api::get_cookie_wrapper;

//   let frog = get_cookie_wrapper("biscuits").await;

//   match frog {
//       Ok(a) => {
//         log!("BIZZY COOKIE OK {:#?}", a);
//       },
//       Err(e) => {
//         log!("BIZZY COOKIE {:#?}", e);
//       },
//   }

//     // let res = create_resource(url, move |url| async move {

//       use leptos_actix::{ResponseOptions, extract};

//       use actix_web::HttpRequest;

//       // let s = extract(|req: HttpRequest| async move {
//       //   log!("COOK LOG {:?}", req.cookie("jwt"));
//       //   log!("COOK BICCY {:?}", req.cookie("biscuits"));
//       //   format!("COOK {:?}", req.cookie("jwt"))
//       // })
//       // .await
//       // .map_err(|e| ServerFnError::ServerError("Could not extract cookie...".to_string()));

//       // use actix_web::{cookie::Cookie, cookie::time::{Duration, OffsetDateTime}, http::header, http::header::HeaderValue};

//       // let response = expect_context::<ResponseOptions>(cx);

//       // let mut cookie = Cookie::build("biscuits", post_id.to_string()).finish();
//       // let mut now = OffsetDateTime::now_utc();
//       // now += Duration::weeks(52);
//       // cookie.set_expires(now);
//       // cookie.set_path("/");

//       // if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
//       //   response.insert_header(header::SET_COOKIE, cookie);
//       // }

//       // let mut cookie = Cookie::build("cheese", post_id.to_string()).finish();
//       // let mut now = OffsetDateTime::now_utc();
//       // now += Duration::weeks(52);
//       // cookie.set_expires(now);
//       // cookie.set_path("/");

//       // if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
//       //   response.insert_header(header::SET_COOKIE, cookie);
//       // }

//       let c = CreatePostLike { /*auth: auth.into(),*/ post_id: PostId(post_id), score };
//       log!("FORMFORM {:#?}", c);
//       let thing = like_post(&c).await;

//       match thing {
//           Err(e) => {
//             log!("serve FN ERROR {:#?}", e);
//             Err(ServerFnError::ServerError(e.to_string()))
//           },
//           Ok(p) => {
//             log!("POST {:#?}", p);
//             Ok(p)
//           }
//       }

//       // let client = reqwest::Client::new();
//       // let thing = client.head(url).send().await;
//       // // let heady = thing;
//       // log!("HEAD passed: {:#?}", thing);
//     // });

//     // Ok(())
// }

#[server(VotePostFn, "/serverfn")]
pub async fn vote_post(
  post_id: i32,
  score: i16,
) -> Result<PostResponse, ServerFnError> {

  // log::log!("sdfsdfsdf");
  leptos::logging::error!("VOTE");

  let c = CreatePostLike {
    post_id: PostId(post_id),
    score,
  };
  let thing = like_post(&c).await;

  leptos::logging::error!("VOTE {:#?}", thing);

  match thing {
    Err(e) => {
      Err(ServerFnError::ServerError(e.to_string()))
    }
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
    Err(e) => {
      log!("serve FN ERROR {:#?}", e);
      Err(ServerFnError::ServerError(e.to_string()))
    }
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
    Err(e) => {
      log!("serve FN ERROR {:#?}", e);
      Err(ServerFnError::ServerError(e.to_string()))
    }
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
    Err(e) => {
      log!("serve FN ERROR {:#?}", e);
      Err(ServerFnError::ServerError(e.to_string()))
    }
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
      log!("{:#?}", e);
      error.set(Some(e.to_string()));
    }
  });

  let save_post_action = create_server_action::<SavePostFn>();

  // create_effect(move |_| match save_post_action.value().get() {
  //   None => {}
  //   Some(Ok(o)) => {
  //     post_view.set(o.post_view);
  //   }
  //   Some(Err(e)) => {
  //     log!("{:#?}", e);
  //     error.set(Some(e.to_string()));
  //   }
  // });

  let block_user_action = create_server_action::<BlockUserFn>();

  let report_post_action = create_server_action::<BlockUserFn>();

  let clicky = move |_| {
    spawn_local(async move {
      let c = CreatePostLike {
        post_id: post_view().post.id,
        score: 1,
      };
  
      let thing = like_post(&c).await;
    
      match thing {
        Err(e) => {
          log!("should do {:#?}", e);
          // Err(ServerFnError::ServerError(e.to_string()))
        }
        Ok(p) => {
          log!("should do {:#?}", p);
          // Ok(p),
        }
      }  
    });
  };

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

  view! {
    <tr>
      <td>
        <ActionForm action=vote_action>
          // <span>{ is_upvote }</span>
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input
            type="hidden"
            name="score"
            value=move || if Some(1) == post_view().my_vote { 0 } else { 1 }
          />
          <button type="submit"><Icon icon=Icon::from(ChIcon::ChArrowUp) class="h-6 w-6"/></button>
        </ActionForm>
        <button on:click=clicky><Icon icon=Icon::from(ChIcon::ChNotes) class="h-6 w-6"/></button>
        <span class="block text-sm">{ move || post_view().counts.score }</span>
        <ActionForm action=vote_action>
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input
            type="hidden"
            name="score"
            value=move || if Some(-1) == post_view().my_vote { 0 } else { -1 }
          />
          <button type="submit"><Icon icon=Icon::from(ChIcon::ChArrowDown) class="h-6 w-6"/></button>
        </ActionForm>
      </td>
      <td>
        <A href=move || format!("{:#?}", pv.post.url)>{move || format!("{:#?}", pv.post.thumbnail_url)}</A><br />
      </td>
      <td>
        <A href={ move || format!("post/{}", post_view().post.id) } class="text-sm inline-block align-bottom">{ move || post_view().post.name }</A><br />
        <A href=move || format!("/u/{}", post_view().creator.name) class="text-sm inline-block align-bottom">{post_view().creator.name}</A> " to " <A href=format!("/c/{}", post_view().community.name)>{post_view().community.title}</A><br />
        <A href=move || format!("post/{}?scrollToComments=true", post_view().post.id) class="text-xs inline-block align-bottom"><Icon icon=Icon::from(ChIcon::ChNotes) class="h-6 w-6"/> " " {post_view().unread_comments}</A>
        <ActionForm action=save_post_action class="inline-block align-bottom">
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input type="hidden" name="save" value={ move || format!("{}", if post_view().saved { false } else { true } ) } />
          <button type="submit"><Icon icon=Icon::from(ChIcon::ChStar) class="h-6 w-6"/></button>
        </ActionForm>
        <A href="/create_post" class="inline-block align-bottom"><Icon icon=Icon::from(ChIcon::ChCopy) class="h-6 w-6"/></A>
        <a href="#" class="inline-block align-bottom"><Icon icon=Icon::from(ChIcon::ChMenuKebab) class="h-6 w-6"/></a>
        <ActionForm action=report_post_action class="inline-block align-bottom">
          <input type="hidden" name="post_id" value=format!("{}", post_view().post.id)/>
          <input class="input input-bordered" type="text" name="reason" placeholder="reason" />
          <button type="submit"><Icon icon=Icon::from(ChIcon::ChFlag) class="h-6 w-6"/></button>
        </ActionForm>
        <ActionForm action=block_user_action class="inline-block align-bottom">
          <input type="hidden" name="person_id" value=format!("{}", post_view().creator.id.0)/>
          <input type="hidden" name="block" value={ move || format!("{}", if post_view().saved { false } else { true } ) } />
          <button type="submit"><Icon icon=Icon::from(ChIcon::ChBlock) class="h-6 w-6"/></button>
        </ActionForm>
      </td>
    </tr>
  }
}

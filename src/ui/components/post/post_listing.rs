use crate::{
  errors::{message_from_error, LemmyAppError, LemmyAppErrorType},
  lemmy_client::*,
};
use lemmy_api_common::{lemmy_db_views::structs::*, person::*, post::*};
use leptos::*;
use leptos_router::*;
use phosphor_leptos::{ArrowDown, ArrowUp, Copy, DotsThreeVertical, Flag, Note, Prohibit, Star};
use web_sys::SubmitEvent;

#[server(VotePostFn, "/serverfn")]
pub async fn vote_post_fn(post_id: i32, score: i16) -> Result<Option<PostResponse>, ServerFnError> {
  use lemmy_api_common::lemmy_db_schema::newtypes::PostId;

  let form = CreatePostLike {
    post_id: PostId(post_id),
    score,
  };
  let result = LemmyClient.like_post(form).await;

  use leptos_actix::redirect;

  match result {
    Ok(o) => Ok(Some(o)),
    Err(e) => {
      redirect(&format!("/?error={}", serde_json::to_string(&e)?)[..]);
      Ok(None)
    }
  }
}

#[server(SavePostFn, "/serverfn")]
pub async fn save_post_fn(post_id: i32, save: bool) -> Result<Option<PostResponse>, ServerFnError> {
  use lemmy_api_common::lemmy_db_schema::newtypes::PostId;

  let form = SavePost {
    post_id: PostId(post_id),
    save,
  };
  let result = LemmyClient.save_post(form).await;

  use leptos_actix::redirect;

  match result {
    Ok(o) => Ok(Some(o)),
    Err(e) => {
      redirect(&format!("/?error={}", serde_json::to_string(&e)?)[..]);
      Ok(None)
    }
  }
}

#[server(BlockUserFn, "/serverfn")]
pub async fn block_user_fn(
  person_id: i32,
  block: bool,
) -> Result<Option<BlockPersonResponse>, ServerFnError> {
  use lemmy_api_common::lemmy_db_schema::newtypes::PersonId;

  let form = BlockPerson {
    person_id: PersonId(person_id),
    block,
  };
  let result = LemmyClient.block_user(form).await;

  use leptos_actix::redirect;

  match result {
    Ok(o) => Ok(Some(o)),
    Err(e) => {
      redirect(&format!("/?error={}", serde_json::to_string(&e)?)[..]);
      Ok(None)
    }
  }
}

fn validate_report(form: &CreatePostReport) -> Option<LemmyAppErrorType> {
  if form.reason.is_empty() {
    return Some(LemmyAppErrorType::MissingReason);
  }
  None
}

async fn try_report(form: CreatePostReport) -> Result<PostReportResponse, LemmyAppError> {
  let val = validate_report(&form);

  match val {
    None => {
      let result = LemmyClient.report_post(form).await;

      match result {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
      }
    }
    Some(e) => Err(LemmyAppError {
      error_type: e.clone(),
      content: format!("{}", form.post_id.0),
    }),
  }
}

#[server(ReportPostFn, "/serverfn")]
pub async fn report_post_fn(
  post_id: i32,
  reason: String,
) -> Result<Option<PostReportResponse>, ServerFnError> {
  use lemmy_api_common::lemmy_db_schema::newtypes::PostId;

  let form = CreatePostReport {
    post_id: PostId(post_id),
    reason,
  };
  let result = try_report(form).await;

  use leptos_actix::redirect;

  match result {
    Ok(o) => Ok(Some(o)),
    Err(e) => {
      redirect(&format!("/?error={}", serde_json::to_string(&e)?)[..]);
      Ok(None)
    }
  }
}

#[component]
pub fn PostListing(post_view: MaybeSignal<PostView>) -> impl IntoView {
  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

  let post_view = create_rw_signal(post_view.get());

  let vote_action = create_server_action::<VotePostFn>();

  let on_vote_submit = move |ev: SubmitEvent, score: i16| {
    ev.prevent_default();

    create_resource(
      move || (),
      move |()| async move {
        let form = CreatePostLike {
          post_id: post_view.get().post.id,
          score,
        };

        let result = LemmyClient.like_post(form).await;

        match result {
          Ok(o) => {
            post_view.set(o.post_view);
          }
          Err(e) => {
            error.set(Some(e));
          }
        }
      },
    );
  };

  let on_up_vote_submit = move |ev: SubmitEvent| {
    let score = if Some(1) == post_view.get().my_vote {
      0
    } else {
      1
    };
    on_vote_submit(ev, score);
  };

  let on_down_vote_submit = move |ev: SubmitEvent| {
    let score = if Some(-1) == post_view.get().my_vote {
      0
    } else {
      -1
    };
    on_vote_submit(ev, score);
  };

  let save_post_action = create_server_action::<SavePostFn>();

  let on_save_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_resource(
      move || (),
      move |()| async move {
        let form = SavePost {
          post_id: post_view.get().post.id,
          save: !post_view.get().saved,
        };

        let result = LemmyClient.save_post(form).await;

        match result {
          Ok(o) => {
            post_view.set(o.post_view);
          }
          Err(e) => {
            error.set(Some(e));
          }
        }
      },
    );
  };

  let block_user_action = create_server_action::<BlockUserFn>();

  let on_block_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_resource(
      move || (),
      move |()| async move {
        let form = BlockPerson {
          person_id: post_view.get().creator.id,
          block: true,
        };

        let result = LemmyClient.block_user(form).await;

        match result {
          Ok(_o) => {}
          Err(e) => {
            error.set(Some(e));
          }
        }
      },
    );
  };

  let report_post_action = create_server_action::<ReportPostFn>();
  let report_validation = create_rw_signal::<String>("".into());

  let query = use_query_map();
  let ssr_error = move || query.with(|params| params.get("error").cloned());

  if let Some(e) = ssr_error() {
    let le = serde_json::from_str::<LemmyAppError>(&e[..]);

    match le {
      Ok(e) => match e {
        LemmyAppError {
          error_type: LemmyAppErrorType::MissingReason,
          content: c,
        } => {
          let id = format!("{}", post_view.get().post.id);
          if c.eq(&id) {
            report_validation.set("input-error".to_string());
          }
        }
        _ => {
          report_validation.set("".to_string());
        }
      },
      Err(_) => {
        logging::log!("error decoding error - log and ignore in UI?");
      }
    }
  }

  let reason = RwSignal::new(String::new());

  let on_report_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_resource(
      move || (),
      move |()| async move {
        let form = CreatePostReport {
          post_id: post_view.get().post.id,
          reason: reason.get(),
        };

        let result = try_report(form).await;

        match result {
          Ok(_o) => {}
          Err(e) => {
            error.set(Some(e.clone()));

            let _id = format!("{}", post_view.get().post.id);

            match e {
              LemmyAppError {
                error_type: LemmyAppErrorType::MissingReason,
                content: _id,
              } => {
                report_validation.set("input-error".to_string());
              }
              _ => {
                report_validation.set("".to_string());
              }
            }
          }
        }
      },
    );
  };

  view! {
    <tr>
      <td class="flex flex-col text-center w-12">
        <ActionForm action=vote_action on:submit=on_up_vote_submit>
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
            <ArrowUp/>
          </button>
        </ActionForm>
        <span class="block text-sm">{move || post_view.get().counts.score}</span>
        <ActionForm action=vote_action on:submit=on_down_vote_submit>
          <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
          <input
            type="hidden"
            name="score"
            value=move || if Some(-1) == post_view.get().my_vote { 0 } else { -1 }
          />
          <button
            type="submit"
            class=move || { if Some(-1) == post_view.get().my_vote { " text-accent" } else { "" } }
            title="Down vote"
          >
            <ArrowDown/>
          </button>
        </ActionForm>
      </td>
      <td class="w-28">
        <a href=move || {
            if let Some(d) = post_view.get().post.url {
                d.inner().to_string()
            } else {
                format!("/post/{}", post_view.get().post.id)
            }
        }>
          {move || {
              if let Some(t) = post_view.get().post.thumbnail_url {
                  let h = t.inner().to_string();
                  view! {
                    <span class="block w-24 truncate">
                      <img class="w-24" src=h/>
                    </span>
                  }
              } else {
                  view! {
                    <span class="block w-24 truncate">
                      <img class="w-24"/>
                    </span>
                  }
              }
          }}

        </a>
      </td>
      <td class="w-full">
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
              class="text-xs whitespace-nowrap align-top"
            >
              <Note class="inline-block"/>
              " "
              {post_view.get().unread_comments}
            </A>
          </span>
          <ActionForm
            action=save_post_action
            on:submit=on_save_submit
            class="inline-block align-top"
          >
            <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
            <input type="hidden" name="save" value=move || format!("{}", !post_view.get().saved)/>
            <button
              type="submit"
              title="Save post"
              class=move || if post_view.get().saved { " text-accent" } else { "" }
            >
              <Star/>
            </button>
          </ActionForm>
          <span title="Cross post">
            <A href="/create_post" class="inline-block align-top">
              <Copy/>
            </A>
          </span>
          <div class="dropdown inline-block align-top">
            <label tabindex="0">
              <DotsThreeVertical/>
            </label>
            <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
              <li>
                <ActionForm class="block" action=report_post_action on:submit=on_report_submit>
                  <input type="hidden" name="post_id" value=format!("{}", post_view.get().post.id)/>
                  <input
                    class=move || format!("input input-bordered {}", report_validation.get())
                    type="text"
                    on:input=move |e| update!(| reason | * reason = event_target_value(& e))
                    name="reason"
                    placeholder="reason"
                  />
                  <button
                    class="text-xs whitespace-nowrap align-top"
                    title="Report post"
                    type="submit"
                  >
                    <Flag class="inline-block"/>
                    " Report post"
                  </button>
                </ActionForm>
              </li>
              <li>
                <ActionForm action=block_user_action on:submit=on_block_submit>
                  <input
                    type="hidden"
                    name="person_id"
                    value=format!("{}", post_view.get().creator.id.0)
                  />
                  <input type="hidden" name="block" value="true"/>
                  <button
                    class="text-xs whitespace-nowrap align-top"
                    title="Block user"
                    type="submit"
                  >
                    <Prohibit class="inline-block"/>
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

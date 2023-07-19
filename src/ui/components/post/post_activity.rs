use crate::{
  api::{comment::get_comments, post::get_post},
  ui::components::{comment::comment_nodes::CommentNodes, post::post_listing::PostListing},
};
use lemmy_api_common::{comment::GetComments, lemmy_db_schema::newtypes::PostId, post::GetPost};
use leptos::*;
use leptos_router::{use_params_map, ParamsMap};
use std::num::ParseIntError;

#[component]
pub fn PostActivity(cx: Scope) -> impl IntoView {
  let params = use_params_map(cx);
  let post = create_resource(
    cx,
    move || post_id_from_params(params),
    move |id| async move {
      match id {
        Err(_) => None,
        Ok(id) => {
          let form = GetPost {
            id: Some(PostId(id)),
            comment_id: None,
            auth: None,
          };
          get_post(cx, &form).await.ok()
        }
      }
    },
  );

  let comments = create_resource(
    cx,
    move || post_id_from_params(params),
    move |id| async move {
      match id {
        Err(_) => None,
        Ok(id) => {
          let form = GetComments {
            post_id: Some(PostId(id)),
            community_id: None,
            type_: None,
            sort: None,
            max_depth: Some(8),
            page: None,
            limit: None,
            community_name: None,
            parent_id: None,
            saved_only: None,
            auth: None,
          };
          get_comments(cx, &form).await.ok()
        }
      }
    },
  );

  let err_msg = " Error loading this post.";

  view! { cx,
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Post page"</h2>
      <Suspense fallback=|| {
          view! { cx, "Loading..." }
      }>
        {move || {
            post.read(cx)
                .map(|res| match res {
                    None => {
                        view! { cx, <div>{err_msg}</div> }
                    }
                    Some(res) => {
                        view! { cx,
                          <div>
                            <PostListing post_view=res.post_view.into()/>
                          </div>
                        }
                    }
                })
        }}
        {move || {
            comments
                .read(cx)
                .map(|res| match res {
                    None => {
                        view! { cx, <div>{err_msg}</div> }
                    }
                    Some(res) => {
                        view! { cx,
                          <div>
                            <CommentNodes comments=res.comments.into()/>
                          </div>
                        }
                    }
                })
        }}

      </Suspense>
    </main>
  }
}

fn post_id_from_params(params: Memo<ParamsMap>) -> Result<i32, ParseIntError> {
  params()
    .get("id")
    .cloned()
    .unwrap_or_default()
    .parse::<i32>()
}

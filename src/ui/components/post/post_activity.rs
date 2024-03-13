use crate::{
  queries::{comments_list_query::use_comments_scope, post_query::use_post},
  ui::components::{
    comment::comment_nodes::CommentNodes,
    common::unpack::Unpack,
    post::post_listing::PostListing,
  },
};
use lemmy_client::lemmy_api_common::{
  comment::GetComments,
  lemmy_db_schema::newtypes::PostId,
  post::GetPost,
};
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::use_params_map;

#[component]
pub fn PostActivity() -> impl IntoView {
  let params = use_params_map();

  let post_id = Signal::derive(move || {
    with!(|params| params
      .get("id")
      .and_then(|post_id| Some(PostId(post_id.as_str().parse().ok()?)))
      .unwrap_or_default())
  });

  let QueryResult {
    data: post_response,
    ..
  } = use_post().use_query(move || {
    with!(|post_id| GetPost {
      id: Some(*post_id),
      comment_id: None
    })
  });

  let QueryResult {
    data: list_comments_response,
    ..
  } = use_comments_scope().use_query(move || {
    with!(|post_id| GetComments {
      post_id: Some(*post_id),
      max_depth: Some(8),
      ..Default::default()
    })
  });

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Post page"</h2>
      <Unpack item=post_response let:res>
        <div>
          <PostListing post_view=res.post_view.clone()/>
        </div>
      </Unpack>

      <Unpack item=list_comments_response let:res>
        <div>
          <CommentNodes comments=res.comments.clone()/>
        </div>
      </Unpack>
    </main>
  }
}

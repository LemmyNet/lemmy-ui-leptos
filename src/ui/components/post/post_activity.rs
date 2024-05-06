use crate::{
  serverfns::{get_post::get_post, list_comments::list_comments},
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

  let post_resource = create_blocking_resource(
    move || {
      with!(|post_id| GetPost {
        id: Some(*post_id),
        comment_id: None
      })
    },
    get_post,
  );

  let list_comments_resource = create_resource(
    move || {
      with!(|post_id| GetComments {
        post_id: Some(*post_id),
        max_depth: Some(8),
        ..Default::default()
      })
    },
    list_comments,
  );

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Post page"</h2>
      <Unpack item=post_resource let:res>
        <div>
          <PostListing post_view=res.post_view.clone()/>
        </div>
      </Unpack>

      <Unpack item=list_comments_resource let:res>
        <div>
          <CommentNodes comments=res.comments.clone()/>
        </div>
      </Unpack>
    </main>
  }
}

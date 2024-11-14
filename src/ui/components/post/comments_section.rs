use lemmy_client::lemmy_api_common::{comment::GetComments, lemmy_db_schema::newtypes::PostId};
use leptos::*;

use crate::{
  serverfns::comments::list_comments, ui::components::common::unpack::Unpack,
  utils::derive_query_signal,
};

#[component]
pub fn CommentsSection(post_id: PostId) -> impl IntoView {
  let comments_resource = create_blocking_resource(
    move || GetComments {
      post_id: Some(post_id),
      max_depth: Some(8),
      ..Default::default()
    },
    list_comments,
  );

  let comments = derive_query_signal(comments_resource, |r| r.comments.clone());

  view! {
      <section>
          <h2 class="sr-only">Comments Section</h2>
          <Transition>
            <Unpack item=comments let:comments>
                <div id/>
            </Unpack>
          </Transition>
      </section>
  }
}

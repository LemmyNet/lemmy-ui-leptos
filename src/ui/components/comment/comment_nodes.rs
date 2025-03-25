use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::prelude::*;

#[component]
pub fn CommentNodes(comments: Vec<CommentView>) -> impl IntoView {
  view! {
    <ul>
      {comments
        .into_iter()
        .map(|cv| {
          view! {
            <li>
              <CommentNode comment_view=cv />
            </li>
          }
        })
        .collect_view()}
    </ul>
  }
}

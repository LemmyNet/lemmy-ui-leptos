use crate::{ui::components::comment::comment_node::CommentNode, utils::CommentList};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::prelude::*;

#[component]
pub fn CommentNodes(comments: Vec<CommentView>) -> impl IntoView {
  let comments = CommentList::new(comments);
  leptos::logging::log!("{comments:#?}");

  view! {"Placeholder"}
  // view! {
  //   <ul>
  //     {comments
  //       .into_iter()
  //       .map(|cv| {
  //         view! {
  //           <li>
  //             <CommentNode comment_view=cv />
  //           </li>
  //         }
  //       })
  //       .collect_view()}
  //   </ul>
  // }
}

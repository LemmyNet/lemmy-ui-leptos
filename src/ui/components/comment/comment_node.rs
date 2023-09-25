use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(comment_view: MaybeSignal<CommentView>) -> impl IntoView {
  let cv = comment_view();
  view! { <div>{cv.creator.name} - {cv.comment.content}</div> }
}

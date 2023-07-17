use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(cx: Scope, comment_view: MaybeSignal<CommentView>) -> impl IntoView {
  let cv = comment_view();
  view! { cx, <div>{cv.creator.name} - {cv.comment.content}</div> }
}

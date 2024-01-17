use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(comment_view: MaybeSignal<CommentView>) -> impl IntoView {
  view! {
    <div>
      {with!(
          | comment_view | format!("{} - {}", comment_view.creator.name, comment_view.comment
          .content)
      )}
    </div>
  }
}

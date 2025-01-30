use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::prelude::*;

#[component]
pub fn CommentNode(#[prop(into)] comment_view: Signal<CommentView>) -> impl IntoView {
  view! {
    <div>
      {move || {
        format!("{} - {}", comment_view.get().creator.name, comment_view.get().comment.content)
      }}

    </div>
  }
}

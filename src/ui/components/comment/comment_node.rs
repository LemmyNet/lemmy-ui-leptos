use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::prelude::*;

#[component]
pub fn CommentNode(#[prop(into)] comment_view: CommentView) -> impl IntoView {
  view! {
    <div>
      "Foo"

    </div>
  }
}

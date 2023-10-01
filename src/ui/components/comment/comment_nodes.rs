use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  view! {
    <ul>
      <For
        each=comments
        key=|cv| cv.comment.id
        children=move |cv| {
            view! {
              <li>
                <CommentNode comment_view=cv.into()/>
              </li>
            }
        }
      />

    </ul>
  }
}

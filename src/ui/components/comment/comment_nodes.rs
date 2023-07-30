use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(cx: Scope, comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  view! { cx,
    <ul>
      <For
        each=comments
        key=|cv| cv.comment.id
        view=move |cx, cv| {
            view! { cx,
              <li>
                <CommentNode comment_view=cv.into()/>
              </li>
            }
        }
      />

    </ul>
  }
}

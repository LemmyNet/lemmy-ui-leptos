use crate::{
  serverfns::posts::{create_report_post_action, create_save_post_action},
  ui::components::common::content_actions::{ContentActionType, ContentActions},
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostContentActions(
  #[prop(into)] id: MaybeSignal<i32>,
  #[prop(into)] saved: MaybeSignal<bool>,
  #[prop(into)] creator_id: MaybeSignal<i32>,
  #[prop(into)] comments: MaybeSignal<i64>,
  post_write_signal: WriteSignal<PostView>,
) -> impl IntoView {
  let save_action = create_save_post_action();

  Effect::new(move |_| {
    let response = save_action.value();

    with!(|response| {
      if let Some(response) = response.as_ref().and_then(|r| r.as_ref().ok()) {
        update!(|post_write_signal| {
          post_write_signal.saved = response.post_view.saved;
        });
      }
    });
  });

  let report_action = create_report_post_action();

  view! {
    <ContentActions
      id=id
      creator_id=creator_id
      saved=saved
      save_action=save_action
      report_action=report_action
      content_action_type=ContentActionType::Post {
          comments,
      }
    />
  }
}

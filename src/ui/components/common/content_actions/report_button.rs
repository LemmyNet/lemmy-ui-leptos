use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::types::{PostOrCommentId, ReportModalData, ReportModalNode},
};
use leptos::*;

fn report_content(
  creator_name: String,
  post_or_comment_id: PostOrCommentId,
  creator_actor_id: String,
) {
  let set_report_modal_data = expect_context::<WriteSignal<ReportModalData>>();
  let report_modal = expect_context::<ReportModalNode>().0;

  set_report_modal_data.set(ReportModalData {
    post_or_comment_id,
    creator_actor_id,
    creator_name,
  });
  let _ = report_modal
    .get_untracked()
    .expect("Report dialog should exist")
    .show_modal();
}

#[component]
pub fn ReportButton(
  creator_name: StoredValue<String>,
  post_or_comment_id: PostOrCommentId,
  creator_actor_id: StoredValue<String>,
) -> impl IntoView {
  let report_content_label = if matches!(post_or_comment_id, PostOrCommentId::Comment(_)) {
    "Report comment"
  } else {
    "Report post"
  };

  view! {
    <button
      class="text-xs whitespace-nowrap"
      type="button"
      on:click=move |_| report_content(
          creator_name.get_value(),
          post_or_comment_id,
          creator_actor_id.get_value(),
      )
    >
      <Icon icon=IconType::Report class="inline-block" />
      " "
      {report_content_label}
    </button>
  }
}
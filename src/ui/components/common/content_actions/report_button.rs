use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::types::{ContentActionType, ReportModalData, ReportModalNode},
};
use leptos::*;

fn report_content(id: i32, content_type: ContentActionType, creator_actor_id: String) {
  let set_report_modal_data = expect_context::<WriteSignal<ReportModalData>>();
  let report_modal = expect_context::<ReportModalNode>().0;

  set_report_modal_data.set(ReportModalData {
    id,
    content_type,
    creator_actor_id,
  });
  let _ = report_modal
    .get_untracked()
    .expect("Report dialog should exist")
    .show_modal();
}

#[component]
pub fn ReportButton(
  id: i32,
  content_action_type: ContentActionType,
  creator_actor_id: StoredValue<String>,
) -> impl IntoView {
  let report_content_label = if content_action_type == ContentActionType::Comment {
    "Report comment"
  } else {
    "Report post"
  };

  view! {
    <button
      class="text-xs whitespace-nowrap"
      type="button"
      on:click=move |_| report_content(id, content_action_type, creator_actor_id.get_value())
    >
      <Icon icon=IconType::Report class="inline-block" />
      " "
      {report_content_label}
    </button>
  }
}

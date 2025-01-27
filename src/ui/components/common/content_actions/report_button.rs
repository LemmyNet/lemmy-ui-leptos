use crate::{
  ui::components::common::icon::{Icon, IconType},
  utils::types::{PostOrCommentId, ReportModalData, ReportModalNode},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::prelude::*;
use leptos_fluent::move_tr;

fn report_content<'a>(creator: &'a Person, post_or_comment_id: PostOrCommentId) {
  let set_report_modal_data = expect_context::<WriteSignal<ReportModalData>>();
  let report_modal = expect_context::<ReportModalNode>().0;

  set_report_modal_data.set(ReportModalData {
    post_or_comment_id,
    creator_actor_id: creator.actor_id.inner().as_str().to_owned(),
    creator_name: creator.name.clone(),
  });
  let _ = report_modal
    .get_untracked()
    .expect("Report dialog should exist")
    .show_modal();
}

#[component]
pub fn ReportButton(
  #[prop(into)] creator: Signal<Person>,
  post_or_comment_id: PostOrCommentId,
) -> impl IntoView {
  let report_content_label = if matches!(post_or_comment_id, PostOrCommentId::Comment(_)) {
    move_tr!("report-comment")
  } else {
    move_tr!("report-post")
  };
  let onclick = move |_| report_content(&creator.read_untracked(), post_or_comment_id);

  view! {
    <button class="text-xs whitespace-nowrap" type="button" on:click=onclick>
      <Icon icon=IconType::Report class="inline-block" />
      " "
      {report_content_label}
    </button>
  }
}

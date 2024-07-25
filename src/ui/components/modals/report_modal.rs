use crate::{
  serverfns::{comments::create_report_comment_action, posts::create_report_post_action},
  ui::components::common::text_input::TextInput,
  utils::{
    create_user_apub_name,
    types::{ContentActionType, ReportModalData},
  },
};
use html::Dialog;
use leptos::*;
use leptos_router::ActionForm;

#[component]
fn ReportForm(
  id: Signal<i32>,
  creator_actor_id: StoredValue<String>,
  content_action_type: Signal<ContentActionType>,
) -> impl IntoView {
  let content_type_str = Signal::derive(move || {
    if content_action_type.get() == ContentActionType::Post {
      "post"
    } else {
      "comment"
    }
  });
  view! {
    <h2>{move || { format!("Report {}", content_type_str.get()) }}</h2>
    <div>
      <span>{move || format!("Creator of {}", content_type_str.get())}</span>
      :
      <span>{create_user_apub_name(creator_actor_id.get_value().as_str())}</span>
    </div>
    <input type="hidden" name="id" value=id />
    <TextInput required=true id="report_reason_id" name="reason" label="Reason" />
  }
}

#[component]
pub fn ReportModal(
  #[allow(unused_variables)] // marked as unused despite being used
  node_ref: NodeRef<Dialog>,
  modal_data: ReadSignal<ReportModalData>,
) -> impl IntoView {
  let id = Signal::derive(move || with!(|modal_data| modal_data.id));
  let content_action_type = Signal::derive(move || with!(|modal_data| modal_data.content_type));
  let creator_actor_id =
    StoredValue::new(modal_data.with_untracked(|data| data.creator_actor_id.clone()));

  let report_post_action = create_report_post_action();
  let report_comment_action = create_report_comment_action();

  view! {
    <dialog _ref=node_ref>
      <Show
        when=move || content_action_type.get() == ContentActionType::Post
        fallback=move || {
            view! {
              <ActionForm action=report_comment_action>
                <ReportForm
                  id=id
                  creator_actor_id=creator_actor_id
                  content_action_type=content_action_type
                />
              </ActionForm>
            }
        }
      >
        <ActionForm action=report_post_action>
          <ReportForm
            id=id
            creator_actor_id=creator_actor_id
            content_action_type=content_action_type
          />
        </ActionForm>
      </Show>
    </dialog>
  }
}

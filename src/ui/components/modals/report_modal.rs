use crate::{
  serverfns::{comments::create_report_comment_action, posts::create_report_post_action},
  ui::components::common::{
    icon::{Icon, IconType},
    text_input::TextInput,
  },
  utils::{
    create_user_apub_name,
    types::{PostOrCommentId, ReportModalData},
  },
};
use html::Dialog;
use leptos::*;
use leptos_router::ActionForm;

#[component]
fn ReportForm(
  creator_actor_id: Signal<String>,
  content_id: Signal<PostOrCommentId>,
) -> impl IntoView {
  let content_type_str = Signal::derive(move || {
    if matches!(content_id.get(), PostOrCommentId::Post(_)) {
      "post"
    } else {
      "comment"
    }
  });

  view! {
    <button
      formmethod="dialog"
      formnovalidate=true
      class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
    >
      <Icon icon=IconType::X />
    </button>
    <h2 class="text-xl font-bold mb-2">
      {move || { format!("Report {}", content_type_str.get()) }}
    </h2>
    <div>
      <strong class="font-semibold">
        {move || format!("Creator of {}", content_type_str.get())}
      </strong>
      ": "
      <span>{move || with!(|creator_actor_id| create_user_apub_name(creator_actor_id))}</span>
    </div>
    <input type="hidden" name="id" value=move || content_id.get().get_id() />
    <TextInput required=true id="report_reason_id" name="reason" label="Reason" autofocus=true />
    <div class="modal-action">
      <button formmethod="dialog" formnovalidate=true class="btn btn-outline">
        Cancel
      </button>
      <button type="submit" class="btn btn-error">
        Submit report
      </button>
    </div>
  }
}

#[component]
pub fn ReportModal(
  dialog_ref: NodeRef<Dialog>,
  modal_data: ReadSignal<ReportModalData>,
) -> impl IntoView {
  let content_id = Signal::derive(move || with!(|modal_data| modal_data.content_id));
  let creator_actor_id =
    Signal::derive(move || modal_data.with(|data| data.creator_actor_id.clone()));

  let form_ref = create_node_ref::<html::Form>();
  let close = move |_| {
    if let (Some(form_ref), Some(dialog_ref)) = (form_ref.get(), dialog_ref.get()) {
      form_ref.reset();
      dialog_ref.close();
    }
  };

  let report_post_action = create_report_post_action();
  Effect::new(move |_| {
    if report_post_action.value().get().is_some() {
      close(());
    }
  });

  let report_comment_action = create_report_comment_action();
  Effect::new(move |_| {
    if report_comment_action.value().get().is_some() {
      close(());
    }
  });

  view! {
    <dialog
      _ref=dialog_ref
      class="modal"
      on:close=move |_| {
          if let Some(form_ref) = form_ref.get() {
              form_ref.reset();
          }
      }
    >
      <Show
        when=move || matches!(content_id.get(), PostOrCommentId::Post(_))
        fallback=move || {
            view! {
              <ActionForm node_ref=form_ref action=report_comment_action class="modal-box">
                <ReportForm creator_actor_id=creator_actor_id content_id=content_id />
              </ActionForm>
            }
        }
      >
        <ActionForm node_ref=form_ref action=report_post_action class="modal-box">
          <ReportForm creator_actor_id=creator_actor_id content_id=content_id />
        </ActionForm>
      </Show>
    </dialog>
  }
}

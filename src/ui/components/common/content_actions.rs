use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::{posts::HidePostAction, users::create_block_user_action},
  ui::components::common::{
    fedilink::Fedilink,
    icon::{Icon, IconType},
  },
  utils::{
    derive_user_is_logged_in,
    traits::ToStr,
    types::{
      Comments,
      ContentActionType,
      Hidden,
      ReportModalData,
      ReportModalNode,
      ServerAction,
      ServerActionFn,
    },
  },
};
use leptos::*;
use leptos_router::{ActionForm, A};
use tailwind_fuse::tw_join;

#[component]
pub fn ContentActions<SA>(
  content_action_type: ContentActionType,
  id: i32,
  saved: Signal<bool>,
  save_action: ServerAction<SA>,
  creator_id: i32,
  creator_actor_id: String,
  apub_link: String,
) -> impl IntoView
where
  SA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let creator_actor_id = StoredValue::new(creator_actor_id);
  let logged_in_user_id = Signal::derive(move || {
    with!(|site_resource| site_resource
      .as_ref()
      .and_then(|data| data
        .as_ref()
        .ok()
        .map(|data| data
          .my_user
          .as_ref()
          .map(|data| data.local_user_view.person.id.0))))
    .flatten()
  });

  let block_user_action = create_block_user_action();

  let save_content_label = if content_action_type == ContentActionType::Comment {
    "Save comment"
  } else {
    "Save post"
  };
  let save_icon = Signal::derive(move || {
    if saved.get() {
      IconType::SaveFilled
    } else {
      IconType::Save
    }
  });
  let crosspost_label = "Crosspost";

  view! {
    <div class="flex items-center gap-x-2">
      {(content_action_type == ContentActionType::Post).then(|| view! { <CommentCount id=id /> })}
      <Fedilink href=apub_link /> <Show when=move || user_is_logged_in.get()>
        <ActionForm action=save_action class="flex items-center">
          <input type="hidden" name="id" value=id />
          <input type="hidden" name="save" value=move || (!saved.get()).to_str() />
          <button
            type="submit"
            title=save_content_label
            aria-label=save_content_label

            class=move || {
                tw_join!(
                    "disabled:cursor-not-allowed disabled:text-neutral-content", saved.get()
                    .then_some("text-accent")
                )
            }

            disabled=move || save_action.pending().get()
          >
            <Icon icon=save_icon />

          </button>
        </ActionForm>
        {(content_action_type == ContentActionType::Post)
            .then(|| {
                view! {
                  <A href="/create_post" attr:title=crosspost_label attr:aria-label=crosspost_label>
                    <Icon icon=IconType::Crosspost />
                  </A>
                }
            })}

        <div class="dropdown">
          <div tabindex="0" role="button">
            <Icon icon=IconType::VerticalDots />
          </div>
          <menu tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
            <Show when=move || {
                logged_in_user_id.get().map(|id| id != creator_id).unwrap_or(false)
            }>
              {(content_action_type == ContentActionType::Post)
                  .then(|| view! { <HidePostButton id=id /> })} <li>
                <ReportButton
                  id=id
                  content_action_type=content_action_type
                  creator_actor_id=creator_actor_id
                />
              </li> <li>
                <ActionForm action=block_user_action>
                  <input type="hidden" name="id" value=creator_id />
                  <input type="hidden" name="block" value="true" />
                  <button class="text-xs whitespace-nowrap" type="submit">
                    <Icon icon=IconType::Block class="inline-block" />
                    " Block user"
                  </button>
                </ActionForm>
              </li>
            </Show>
          </menu>
        </div>
      </Show>
    </div>
  }
}

#[component]
fn HidePostButton(id: i32) -> impl IntoView {
  let hide_post_action = expect_context::<HidePostAction>();
  let hidden = expect_context::<Signal<Hidden>>();
  let icon = Signal::derive(move || {
    if hidden.get().0 {
      IconType::Eye
    } else {
      IconType::EyeSlash
    }
  });

  view! {
    <li>
      <ActionForm action=hide_post_action>
        <input type="hidden" name="id" value=id />
        <input type="hidden" name="hide" value=move || (!hidden.get().0).to_str() />
        <button class="text-xs whitespace-nowrap" type="submit">
          <Icon icon=icon class="inline-block" />
          " "
          {move || if hidden.get().0 { "Unhide post" } else { "Hide post" }}
        </button>
      </ActionForm>
    </li>
  }
}

#[component]
fn CommentCount(id: i32) -> impl IntoView {
  let comments = expect_context::<Signal<Comments>>();
  let num_comments_label = Signal::derive(move || format!("{} comments", comments.get().0));

  view! {
    <A
      href=move || { format!("/post/{id}") }
      class="text-sm whitespace-nowrap"
      attr:title=num_comments_label
      attr:aria-label=num_comments_label
    >
      <Icon icon=IconType::Comment class="inline align-baseline" />
      " "
      <span class="align-sub">{move || comments.get().0}</span>
    </A>
  }
}

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
fn ReportButton(
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

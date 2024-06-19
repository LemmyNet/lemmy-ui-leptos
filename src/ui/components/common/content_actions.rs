use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::users::create_block_user_action,
  ui::components::common::icon::{Icon, IconType},
  utils::{
    derive_user_is_logged_in,
    types::{ServerAction, ServerActionFn},
  },
};
use leptos::*;
use leptos_router::{ActionForm, A};

mod post_content_actions;
pub use post_content_actions::PostContentActions;
use tailwind_fuse::tw_join;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentActionType {
  Post {
    comments: MaybeSignal<i64>,
  },
  #[allow(dead_code)]
  Comment,
}

#[component]
fn ContentActions<SA, RA>(
  content_action_type: ContentActionType,
  #[prop(into)] id: MaybeSignal<i32>,
  save_action: ServerAction<SA>,
  #[prop(into)] saved: MaybeSignal<bool>,
  report_action: ServerAction<RA>,
  #[prop(into)] creator_id: MaybeSignal<i32>,
) -> impl IntoView
where
  SA: ServerActionFn,
  RA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  let block_user_action = create_block_user_action();

  view! {
    <div class="flex items-center gap-x-2">
      {move || {
          if let ContentActionType::Post { comments } = content_action_type {
              Some(
                  view! {
                    <A
                      href=move || { format!("/post/{}", id.get()) }
                      class="text-sm whitespace-nowrap"
                      attr:title=move || format!("{} comments", comments.get())
                    >
                      <Icon icon=IconType::Comment class="inline align-baseline"/>
                      " "
                      <span class="align-sub">{comments}</span>
                    </A>
                  },
              )
          } else {
              None
          }
      }}
      <Show when=move || user_is_logged_in.get()>
        <ActionForm action=save_action class="flex items-center">
          <input type="hidden" name="id" value=id/>
          <input type="hidden" name="save" value=move || (!saved.get()).to_string()/>
          <button
            type="submit"
            title=if matches!(content_action_type, ContentActionType::Comment) {
                "Save comment"
            } else {
                "Save post"
            }

            class=move || {
                tw_join!(
                    "disabled:cursor-not-allowed disabled:text-neutral-content", if saved.get() {
                    Some("text-accent") } else { None }
                )
            }

            disabled=move || save_action.pending().get()
          >
            <Show when=move || saved.get() fallback=move || view! { <Icon icon=IconType::Save/> }>
              <Icon icon=IconType::SaveFilled/>
            </Show>
          </button>
        </ActionForm>
        <Show when=move || matches!(content_action_type, ContentActionType::Post { .. })>
          <A href="/create_post">
            <Icon icon=IconType::Crosspost/>
          </A>
        </Show>
        <div class="dropdown hidden sm:block">
          <label tabindex="0">
            <Icon icon=IconType::VerticalDots/>
          </label>
          <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
            <li>
              <ActionForm action=report_action>
                <input type="hidden" name="id" value=id/>
                <input type="text" name="reason" placeholder="reason"/>
                <button class="text-xs whitespace-nowrap" title="Report post" type="submit">
                  <Icon icon=IconType::Report class="inline-block"/>
                  {if matches!(content_action_type, ContentActionType::Comment) {
                      " Report comment"
                  } else {
                      " Report post"
                  }}

                </button>
              </ActionForm>
            </li>
            <li>
              <ActionForm action=block_user_action>
                <input type="hidden" name="id" value=creator_id/>
                <input type="hidden" name="block" value="true"/>
                <button class="text-xs whitespace-nowrap" title="Block user" type="submit">
                  <Icon icon=IconType::Block class="inline-block"/>
                  " Block user"
                </button>
              </ActionForm>
            </li>
          </ul>
        </div>
      </Show>
    </div>
  }
}

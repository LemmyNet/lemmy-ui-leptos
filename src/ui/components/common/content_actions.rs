use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::{posts::HidePostAction, users::create_block_user_action},
  ui::components::common::{
    fedilink::Fedilink,
    icon::{Icon, IconType},
  },
  utils::{
    derive_user_is_logged_in, traits::ToStr, types::{ServerAction, ServerActionFn}
  },
};
use leptos::*;
use leptos_router::{ActionForm, A};

mod post_content_actions;
pub use post_content_actions::PostContentActions;
use tailwind_fuse::tw_join;

#[derive(Clone, Copy)]
enum ContentActionType {
  Post {
    comments: MaybeSignal<i64>,
    hide_post_action: HidePostAction,
    hidden: MaybeSignal<bool>,
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
  apub_link: TextProp,
) -> impl IntoView
where
  SA: ServerActionFn,
  RA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
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

  let save_content_label = if matches!(content_action_type, ContentActionType::Comment) {
    "Save comment"
  } else {
    "Save post"
  };
  let crosspost_label = "Crosspost";
  let report_content_label = if matches!(content_action_type, ContentActionType::Comment) {
    "Report comment"
  } else {
    "Report post"
  };

  view! {
    <div class="flex items-center gap-x-2">
      {move || {
          if let ContentActionType::Post { comments, .. } = content_action_type {
              let num_comments_label = format!("{} comments", comments.get());
              Some(
                  view! {
                    <A
                      href=move || { format!("/post/{}", id.get()) }
                      class="text-sm whitespace-nowrap"
                      attr:title=num_comments_label.clone()
                      attr:aria-label=num_comments_label
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
      <Fedilink href=apub_link/> <Show when=move || user_is_logged_in.get()>
        <ActionForm action=save_action class="flex items-center">
          <input type="hidden" name="id" value=id/>
          <input type="hidden" name="save" value=move || (!saved.get()).to_str()/>
          <button
            type="submit"
            title=save_content_label
            aria-label=save_content_label

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
          <A href="/create_post" attr:title=crosspost_label attr:aria-label=crosspost_label>
            <Icon icon=IconType::Crosspost/>
          </A>
        </Show>
        <div class="dropdown">
          <div tabindex="0" role="button">
            <Icon icon=IconType::VerticalDots/>
          </div>
          <menu tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
            <Show when=move || {
                logged_in_user_id.get().map(|id| id != creator_id.get()).unwrap_or(false)
            }>
              {move || {
                  if let ContentActionType::Post { hide_post_action, hidden, .. } = content_action_type {
                      let icon = Signal::derive(move || {
                          if hidden.get() { IconType::EyeSlash } else { IconType::Eye }
                      });
                      let hide_post_label = Signal::derive(move || {
                          if hidden.get() { "Unhide Post" } else { "Hide Post" }
                      });

                      
                      Some(
                          view! {
                            <li>
                              <ActionForm action=hide_post_action>
                                <input type="hidden" name="id" value=id/>
                                <input type="hidden" name="hide" value=move || (!hidden.get()).to_str()/>
                                <button class="text-xs whitespace-nowrap" type="submit">
                                  <Icon icon=icon class="inline-block"/>
                                  " "
                                  {hide_post_label}
                                </button>
                              </ActionForm>
                            </li>
                          },
                      )
                  } else {
                      None
                  }
              }}
              <li>
                <ActionForm action=report_action>
                  <input type="hidden" name="id" value=id/>
                  <button class="text-xs whitespace-nowrap" type="submit">
                    <Icon icon=IconType::Report class="inline-block"/>
                    " "
                    {report_content_label}
                  </button>
                </ActionForm>
              </li> <li>
                <ActionForm action=block_user_action>
                  <input type="hidden" name="id" value=creator_id/>
                  <input type="hidden" name="block" value="true"/>
                  <button class="text-xs whitespace-nowrap" type="submit">
                    <Icon icon=IconType::Block class="inline-block"/>
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

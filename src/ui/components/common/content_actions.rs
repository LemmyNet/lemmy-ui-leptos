use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::users::create_block_user_action,
  ui::components::common::{
    fedilink::Fedilink,
    icon::{Icon, IconType},
  },
  utils::{
    traits::ToStr,
    types::{PostOrCommentId, ServerActionFn},
  },
};
use hide_post_button::HidePostButton;
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;
use report_button::ReportButton;
use tailwind_fuse::tw_join;

mod hide_post_button;
mod report_button;

#[component]
pub fn ContentActions<SA>(
  post_or_comment_id: PostOrCommentId,
  saved: Signal<bool>,
  save_action: ServerAction<SA>,
  #[prop(into)] creator: Signal<Person>,
  ap_id: String,
) -> impl IntoView
where
  SA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();

  let save_content_label = if matches!(post_or_comment_id, PostOrCommentId::Post(_)) {
    move_tr!("save-post")
  } else {
    move_tr!("save-comment")
  };
  let save_icon = Signal::derive(move || {
    if saved.get() {
      IconType::SaveFilled
    } else {
      IconType::Save
    }
  });
  let crosspost_label = move_tr!("crosspost");

  let block_user_action = create_block_user_action();

  view! {
    <Fedilink href=ap_id.to_string() />
    <Transition>
      {move || Suspend::new(async move {
        site_resource
          .await
          .map(|site| {
            let logged_in_user_id = site.my_user.map(|u| u.local_user_view.person.id);
            logged_in_user_id
              .map(|user_id| {

                view! {
                  <ActionForm action=save_action attr:class="flex items-center">
                    <input type="hidden" name="id" value=post_or_comment_id.get_id() />
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
                  {(matches!(post_or_comment_id, PostOrCommentId::Post(_)))
                    .then(|| {
                      view! {
                        <A
                          href="/create_post"
                          attr:title=crosspost_label
                          attr:aria-label=crosspost_label
                        >
                          <Icon icon=IconType::Crosspost />
                        </A>
                      }
                    })}

                  <div class="dropdown">
                    <div tabindex="0" role="button">
                      <Icon icon=IconType::VerticalDots />
                    </div>
                    <menu
                      tabindex="0"
                      class="menu dropdown-content z-1 bg-base-100 rounded-box shadow-sm"
                    >
                      <Show when=move || {
                        user_id == creator.read_untracked().id
                      }>
                        {if let PostOrCommentId::Post(id) = post_or_comment_id {
                          Some(view! { <HidePostButton id=id /> })
                        } else {
                          None
                        }} <li>
                          <ReportButton creator=creator post_or_comment_id=post_or_comment_id />
                        </li> <li>
                          <ActionForm action=block_user_action>
                            <input type="hidden" name="id" value=creator.read_untracked().id.0 />
                            <input type="hidden" name="block" value="true" />
                            <button class="text-xs whitespace-nowrap" type="submit">
                              <Icon icon=IconType::Block class="inline-block" />
                              " "
                              {move_tr!("block-user")}
                            </button>
                          </ActionForm>
                        </li>
                      </Show>
                    </menu>
                  </div>
                }
              })
          })
      })}
    </Transition>
  }
}

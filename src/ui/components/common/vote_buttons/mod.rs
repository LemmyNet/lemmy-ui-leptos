use crate::{contexts::site_resource_context::SiteResource, utils::derive_user_is_logged_in};
use leptos::{
  server_fn::{client::Client, codec::PostUrl, request::ClientReq, ServerFn},
  *,
};
use leptos_router::ActionForm;
use phosphor_leptos::{ArrowDown, ArrowUp};
use serde::de::DeserializeOwned;
use web_sys::FormData;

mod comment_vote_buttons;
mod post_vote_buttons;

pub use comment_vote_buttons::CommentVoteButtons;
pub use post_vote_buttons::PostVoteButtons;

#[component]
fn VoteButtons<ServeFn>(
  #[prop(into)] my_vote: MaybeProp<i16>,
  #[prop(into)] id: MaybeSignal<i32>,
  #[prop(into)] score: MaybeSignal<i64>,
  vote_action: Action<ServeFn, Result<ServeFn::Output, ServerFnError<ServeFn::Error>>>,
) -> impl IntoView
where
  ServeFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
  <<ServeFn::Client as Client<ServeFn::Error>>::Request as ClientReq<ServeFn::Error>>::FormData:
    From<FormData>,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let is_upvote = Signal::derive(move || my_vote.get().unwrap_or_default() == 1);
  let is_downvote = Signal::derive(move || my_vote.get().unwrap_or_default() == -1);

  view! {
    <div>
      <ActionForm action=vote_action>
        <input type="hidden" name="id" value=id/>
        <input
          type="hidden"
          name="score"
          value=move || with!(| is_upvote | if * is_upvote { 0 } else { 1 })
        />
        <button
          type="submit"
          class=move || {
              with!(
                  | is_upvote | { let mut class = String::from("align-bottom"); if * is_upvote {
                  class.push_str(" text-accent"); } class }
              )
          }

          title="Up vote"
          disabled=move || !user_is_logged_in.get() || vote_action.pending().get()
        >

          <ArrowUp class="size-6"/>
        </button>
      </ActionForm>
      <span class="block text-sm">{score}</span>
      <ActionForm action=vote_action>
        <input type="hidden" name="id" value=id/>
        <input
          type="hidden"
          name="score"
          value=move || with!(| is_downvote | if * is_downvote { 0 } else { - 1 })
        />
        <button
          type="submit"
          class=move || {
              with!(
                  | is_downvote | { let mut class = String::from("align-top"); if * is_downvote {
                  class.push_str(" text-accent"); } class }
              )
          }

          title="Down vote"

          disabled=move || !user_is_logged_in.get() || vote_action.pending().get()
        >
          <ArrowDown class="size-6"/>
        </button>
      </ActionForm>
    </div>
  }
}

use std::rc::Rc;

use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::{
    creator_listing::CreatorListing,
    icon::{Icon, IconSize, IconType},
    time_since::TimeSince,
  },
};

#[component]
pub fn CommentNode<'a>(comment_view: &'a CommentView) -> impl IntoView {
  let comment_state = RwSignal::new(comment_view.clone());
  let content = Memo::new(move |_| {
    with!(|comment_state| Rc::<str>::from(comment_state.comment.content.as_str()))
  });
  let published = Memo::new(move |_| with!(|comment_state| comment_state.comment.published));

  let site_resource = expect_context::<SiteResource>();
  let comment_language = Memo::new(move |_| {
    with!(|site_resource, comment_state| site_resource
      .as_ref()
      .and_then(|r| r.as_ref().ok())
      .and_then(|site_resource| (comment_state.comment.language_id.0 != 0)
        .then(|| site_resource
          .all_languages
          .iter()
          .find(|l| l.id == comment_state.comment.language_id)
          .map(|l| l.name.clone()))
        .flatten()))
  });

  view! {
    <article>
      <details class="group" open>
        <summary class="flex items-center gap-1.5 list-none marker:hidden p-2 cursor-pointer">
          <Icon class="group-[&:not([open])]:-rotate-90" icon=IconType::DropdownCaret />
          <CreatorListing creator=&comment_view.creator/>
          {move || {
            with!(
              |comment_language| comment_language.as_ref().map(|lang| view! {
                <div class="text-xs badge badge-ghost gap-x-0.5">
                  <Icon icon=IconType::Language size=IconSize::Small />
                  {lang}
                </div>
          })
            )
          }}
          <TimeSince datetime=published/>
        </summary>
        {content}
      </details>
    </article>
  }
}

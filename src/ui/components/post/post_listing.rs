use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::posts::{create_hide_post_action, create_save_post_action, create_vote_post_action},
  ui::components::common::{
    community_listing::CommunityListing,
    content_actions::ContentActions,
    creator_listing::CreatorListing,
    icon::{Icon, IconSize, IconType},
    vote_buttons::VoteButtons,
  },
  utils::{
    get_time_since,
    is_image,
    markdown_to_html,
    types::{Hidden, PostOrCommentId},
  },
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::*;
use leptos::*;
use leptos_router::*;
use std::rc::Rc;
use thumbnail::Thumbnail;

mod thumbnail;

#[component]
pub fn PostListing<'a>(post_view: &'a PostView) -> impl IntoView {
  // These post fields cannot change, so no need for signals
  let id = post_view.post.id;
  let ap_id = post_view.post.ap_id.inner().as_str();
  let creator = &post_view.creator;
  let community = &post_view.community;

  let post_state = RwSignal::new(post_view.clone());

  let post_body = Memo::new(move |_| with!(|post_state| post_state.post.body.clone()));

  let post_url = Memo::new(move |_| {
    with!(|post_state| post_state
      .post
      .url
      .as_deref()
      .map(AsRef::as_ref)
      .map(Rc::<str>::from))
  });

  let image_url = Memo::new(move |_| {
    with!(
      |post_state| post_state
        .post
        .thumbnail_url
        .as_deref()
        .map(AsRef::as_ref)
        .map(Rc::from)
        .or_else(|| post_url.get().filter(|url| is_image(url.as_ref()))) // Fall back to post url if no thumbnail, but only if it is an image url
    )
  });

  let embed_video_url = Memo::new(move |_| {
    with!(|post_state| post_state
      .post
      .embed_video_url
      .as_ref()
      .map(ToString::to_string))
  });

  let time_since_post =
    Memo::new(move |_| with!(|post_state| get_time_since(&post_state.post.published)));
  let site_resource = expect_context::<SiteResource>();
  let post_language = Memo::new(move |_| {
    with!(|site_resource, post_state| site_resource
      .as_ref()
      .and_then(|r| r.as_ref().ok())
      .and_then(|site_resource| (post_state.post.language_id.0 != 0)
        .then(|| site_resource
          .all_languages
          .iter()
          .find(|l| l.id == post_state.post.language_id)
          .map(|l| l.name.clone()))
        .flatten()))
  });

  // TODO: These fields will need to be updateable once editing posts is supported
  let (post_name, _set_post_name) = slice!(post_state.post.name);

  // TODO: Will need setter once creating comments is supported
  let (comments, _set_comments) = slice!(post_state.counts.comments);
  let num_comments_label = Signal::derive(move || format!("{} comments", comments.get()));

  let (my_vote, set_my_vote) = slice!(post_state.my_vote);
  let (score, set_score) = slice!(post_state.counts.score);
  let (_upvotes, set_upvotes) = slice!(post_state.counts.upvotes);
  let (_downvotes, set_downvotes) = slice!(post_state.counts.downvotes);

  let vote_action = create_vote_post_action();
  Effect::new(move |_| {
    let response = vote_action.value();

    with!(|response| {
      if let Some(response) = response.as_ref().and_then(|r| r.as_ref().ok()) {
        set_my_vote.set(response.post_view.my_vote);
        set_score.set(response.post_view.counts.score);
        set_upvotes.set(response.post_view.counts.upvotes);
        set_downvotes.set(response.post_view.counts.downvotes);
      }
    });
  });

  let (saved, set_saved) = slice!(post_state.saved);
  let save_action = create_save_post_action();
  Effect::new(move |_| {
    let response = save_action.value();

    with!(|response| {
      if let Some(response) = response.as_ref().and_then(|r| r.as_ref().ok()) {
        set_saved.set(response.post_view.saved);
      }
    });
  });

  let (hidden, set_hidden) = create_slice(
    post_state,
    |post_view| Hidden(post_view.hidden),
    |post_view, hidden| post_view.hidden = hidden,
  );
  let hide_post_action = create_hide_post_action();
  Effect::new(move |_| {
    let response = hide_post_action.value();

    with!(|response| {
      if response
        .as_ref()
        .and_then(|r| r.as_ref().ok().map(|r| r.success))
        .unwrap_or(false)
      {
        set_hidden.set(!hidden.get().0);
      }
    });
  });
  provide_context(hidden);
  provide_context(hide_post_action);

  let is_on_post_page = use_route().path().starts_with("/post");

  view! {
    <article class="w-full h-fit pe-1">
      <div class="grid sm:grid-areas-post-listing sm:grid-cols-post-listing sm:grid-rows-post-listing grid-areas-post-listing-mobile grid-cols-post-listing-mobile grid-rows-post-listing-mobile items-center gap-2.5">
        <VoteButtons
          id=PostOrCommentId::Post(id)
          my_vote=my_vote
          score=score
          vote_action=vote_action
          class="grid-in-vote"
        />
        <Thumbnail
          url=post_url
          image_url=image_url
          has_embed_url=move || with!(|embed_video_url| embed_video_url.is_some())
          id=id
        />

      <Show
        when=move || is_on_post_page
        fallback=move || {
          view! {
            <h2 class="text-lg font-medium grid-in-title">
              <A href=format!("/post/{id}")>{post_name}</A>
            </h2>
          }
        }
      >
          <h1 class="text-2xl font-bold grid-in-title">{post_name}</h1>
        </Show>
        <div class="grid-in-to">
          <div class="flex flex-wrap items-center gap-1.5">
            <CreatorListing creator=creator />
            <div class="text-sm">to</div>
            <CommunityListing community=community />
          </div>
          <div class="flex flex-wrap items-center gap-1.5 mt-2">
            <div class="text-xs badge badge-ghost gap-x-0.5">
              <Icon icon=IconType::Clock size=IconSize::Small />
              {time_since_post}
            </div>
            {move || {
                with!(
                    |post_language| post_language.as_ref().map(|lang| view! {
                  <div class="text-xs badge badge-ghost gap-x-0.5">
                    <Icon icon=IconType::Language size=IconSize::Small />
                    {lang}
                  </div>
            })
                )
            }}
          </div>
        </div>

        <div class="flex items-center gap-x-2 grid-in-actions">
          <A
            href=move || { format!("/post/{id}") }
            class="text-sm whitespace-nowrap"
            attr:title=num_comments_label
            attr:aria-label=num_comments_label
          >
            <Icon icon=IconType::Comment class="inline align-baseline" />
            " "
            <span class="align-sub">{comments}</span>
          </A>
          <ContentActions
            post_or_comment_id=PostOrCommentId::Post(id)
            saved=saved
            save_action=save_action
            creator=creator
            ap_id=ap_id
          />
        </div>
      </div>
      {move || {
          with!(
              |post_body| post_body.as_deref().map(|body| is_on_post_page.then(|| view! {
        <div class="bg-base-200 mt-4 p-5 rounded" inner_html=markdown_to_html(body)/>
      }))
          )
      }}
    </article>
  }
}

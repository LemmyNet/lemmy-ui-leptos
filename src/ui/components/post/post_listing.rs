use crate::{
  serverfns::posts::{create_hide_post_action, create_save_post_action, create_vote_post_action},
  ui::components::common::{
    community_listing::CommunityListing,
    content_actions::ContentActions,
    creator_listing::CreatorListing,
    icon::{Icon, IconSize, IconType},
    vote_buttons::{VoteButtons, VotesOrientation},
  },
  utils::types::{Hidden, PostOrCommentId},
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::*;
use leptos::*;
use leptos_router::*;
use tailwind_fuse::tw_join;

#[component]
pub fn PostListing<'a>(post_view: &'a PostView) -> impl IntoView {
  // These post fields cannot change, so no need for signals
  let id = post_view.post.id;
  let ap_id = post_view.post.ap_id.inner().as_str();
  let creator = &post_view.creator;
  let community = &post_view.community;

  let post_state = RwSignal::new(post_view.clone());

  // TODO: These fields will need to be updateable once editing posts is supported
  let (post_name, _set_post_name) = slice!(post_state.post.name);
  let (url, _set_url) = create_slice(
    post_state,
    |state| state.post.url.as_ref().map(|url| url.to_string()),
    |state, url| state.post.url = url,
  );
  let (thumbnail_url, _set_thumbnail_url) = create_slice(
    post_state,
    |state| state.post.thumbnail_url.as_ref().map(ToString::to_string),
    |state, thumbnail_url| state.post.thumbnail_url = thumbnail_url,
  );

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

  let orientation = if is_on_post_page {
    VotesOrientation::Horizontal
  } else {
    VotesOrientation::Vertical
  };

  view! {
    <article class="grid md:grid-areas-post-listing-list md:grid-cols-post-listing-list md:grid-rows-post-listing-list grid-areas-post-listing-list-mobile grid-cols-post-listing-list-mobile grid-rows-post-listing-list-mobile w-fit h-fit items-center gap-y-2">
      <VoteButtons
        id=PostOrCommentId::Post(id)
        my_vote=my_vote
        score=score
        vote_action=vote_action
        class="grid-in-vote"
        orientation=orientation
      />
      {move || {
          with!(
              | thumbnail_url, url | thumbnail_url.as_ref().or(url.as_ref()).map(| thumbnail_url |
              view! { < img class = "w-16 aspect-square rounded grid-in-thumbnail" src = thumbnail_url /> }
              .into_view()).unwrap_or_else(|| view! { < A href = format!("/post/{id}") class =
              "w-16 grid-in-thumbnail" > < Icon icon = IconType::Comments class = "m-auto" size = IconSize::ExtraLarge
              /></ A > } .into_view())
          )
      }}

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

        <h1 class="text-2xl font-bold">{post_name}</h1>
      </Show>
      <div class="flex flex-wrap items-center gap-1.5 grid-in-to">
        <CreatorListing creator=creator />
        <div class="text-sm">to</div>
        <CommunityListing community=community />
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
          <span class="align-sub">{move || comments.get()}</span>
        </A>
        <ContentActions
          post_or_comment_id=PostOrCommentId::Post(id)
          saved=saved
          save_action=save_action
          creator=creator
          ap_id=ap_id
        />
      </div>

    </article>
  }
}

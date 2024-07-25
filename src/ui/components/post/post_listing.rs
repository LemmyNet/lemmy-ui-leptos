use crate::{
  serverfns::posts::{create_hide_post_action, create_save_post_action, create_vote_post_action},
  ui::components::common::{
    community_listing::CommunityListing,
    content_actions::ContentActions,
    creator_listing::CreatorListing,
    icon::{Icon, IconSize, IconType},
    vote_buttons::VoteButtons,
  },
  utils::types::{Comments, ContentActionType, Hidden},
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn PostListing(post_view: PostView) -> impl IntoView {
  // These post fields cannot change, so no need for signals
  let id = post_view.post.id.0;
  let actor_id = post_view.post.ap_id.to_string();

  let creator_id = post_view.creator.id.0;
  let creator_avatar = post_view.creator.avatar.clone().map(|url| url.to_string());
  let creator_name = post_view.creator.name.clone();
  let creator_display_name = post_view.creator.display_name.clone();
  let creator_actor_id = post_view.creator.actor_id.clone().to_string();

  let community_icon = post_view.community.icon.clone().map(|url| url.to_string());
  let community_name = post_view.community.name.clone();
  let community_title = post_view.community.title.clone();
  let community_actor_id = post_view.community.actor_id.clone().to_string();

  let post_state = RwSignal::new(post_view);

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
  let (comments, _set_comments) = create_slice(
    post_state,
    |state| Comments(state.counts.comments),
    |state, comments| state.counts.comments = comments,
  );
  provide_context(comments);

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
    <article class="flex gap-x-3 items-center w-fit">
      <VoteButtons id=id my_vote=my_vote score=score vote_action=vote_action />
      {move || {
          with!(
              | thumbnail_url, url | thumbnail_url.as_ref().or(url.as_ref()).map(| thumbnail_url |
              view! { < img class = "w-24 aspect-square rounded" src = thumbnail_url /> }
              .into_view()).unwrap_or_else(|| view! { < A href = format!("/post/{id}") class =
              "w-24" > < Icon icon = IconType::Comments class = "m-auto" size = IconSize::ExtraLarge
              /></ A > } .into_view())
          )
      }}

      <div class="space-y-1.5">
        <Show
          when=move || is_on_post_page
          fallback=move || {
              view! {
                <h2 class="text-lg font-medium">
                  <A href=format!("/post/{id}")>{post_name}</A>
                </h2>
              }
          }
        >

          <h1 class="text-xl font-bold">
            <A href=format!("/post/{id}")>{post_name}</A>
          </h1>
        </Show>
        <div class="flex items-center gap-1.5">
          <CreatorListing
            avatar=creator_avatar
            name=creator_name
            display_name=creator_display_name
            actor_id=creator_actor_id.clone()
          />
          <div class="text-sm">to</div>
          <CommunityListing
            icon=community_icon
            name=community_name
            title=community_title
            actor_id=community_actor_id
          />
        </div>

        <ContentActions
          content_action_type=ContentActionType::Post
          id=id
          saved=saved
          save_action=save_action
          creator_id=creator_id
          apub_link=actor_id
          creator_actor_id=creator_actor_id
        />
      </div>

    </article>
  }
}

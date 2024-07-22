use crate::ui::components::common::{
  community_listing::CommunityListing,
  content_actions::PostContentActions,
  creator_listing::CreatorListing,
  icon::{Icon, IconSize, IconType},
  vote_buttons::PostVoteButtons,
};
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn PostListing(#[prop(into)] post_view: MaybeSignal<PostView>) -> impl IntoView {
  let post_view = RwSignal::new(post_view.get());
  let (id, _) = slice!(post_view.post.id.0);
  let (post_name, _) = slice!(post_view.post.name);
  let (my_vote, _) = slice!(post_view.my_vote);
  let (score, _) = slice!(post_view.counts.score);
  let url =
    Signal::derive(move || with!(|post_view| post_view.post.url.as_ref().map(ToString::to_string)));
  let thumbnail_url = Signal::derive(move || {
    with!(|post_view| post_view
      .post
      .thumbnail_url
      .as_ref()
      .map(ToString::to_string))
  });

  let (creator_id, _) = slice!(post_view.creator.id.0);
  let (comments, _) = slice!(post_view.counts.comments);
  let (saved, _) = slice!(post_view.saved);
  let apub_link = with!(|post_view| post_view.post.ap_id.to_string());

  let is_on_post_page = use_route().path().starts_with("/post");

  view! {
    <article class="flex gap-x-3 items-center w-fit">
      <PostVoteButtons id=id my_vote=my_vote score=score post_write_signal=post_view.write_only()/>
      {move || {
          with!(
              | thumbnail_url, url | thumbnail_url.as_ref().or(url.as_ref()).map(| thumbnail_url |
              view! { < img class = "w-24 aspect-square rounded" src = thumbnail_url /> }
              .into_view()).unwrap_or_else(|| view! { < A href = move || with!(| id |
              format!("/post/{id}")) class = "w-24" > < Icon icon = IconType::Comments class =
              "m-auto" size = IconSize::ExtraLarge /></ A > } .into_view())
          )
      }}

      <div class="space-y-1.5">
        <Show
          when=move || is_on_post_page
          fallback=move || {
              view! {
                <h2 class="text-lg font-medium">
                  <A href=move || with!(| id | format!("/post/{id}"))>{post_name}</A>
                </h2>
              }
          }
        >

          <h1 class="text-xl font-bold">
            <A href=move || with!(| id | format!("/post/{id}"))>{post_name}</A>
          </h1>
        </Show>
        <div class="flex items-center gap-1.5">
          <CreatorListing creator=with!(| post_view | post_view.creator.clone())/>
          <div class="text-sm">to</div>
          <CommunityListing community=with!(| post_view | post_view.community.clone())/>
        </div>

        <PostContentActions
          id=id
          creator_id=creator_id
          saved=saved
          comments=comments
          post_write_signal=post_view.write_only()
          apub_link=apub_link
        />
      </div>

    </article>
  }
}

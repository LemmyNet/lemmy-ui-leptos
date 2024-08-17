use crate::{
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::{is_image, is_video},
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::newtypes::PostId,
  lemmy_db_views::structs::PostView,
};
use leptos::*;
use leptos_router::A;
use std::rc::Rc;

#[component]
fn Thumbnail(post_state: ReadSignal<PostView>, id: PostId) -> impl IntoView {
  let url =
    Memo::new(move |_| with!(|post_state| post_state.post.url.as_ref().map(|url| url.to_string())));
  let thumbnail = Memo::new(move |_| {
    with!(|post_state, url| post_state
      .post
      .thumbnail_url
      .as_deref()
      .map(AsRef::as_ref)
      .or_else(|| url.as_deref().filter(|url| is_image(url))) // Fall back to post url if no thumbnail, but only if it is an image url
      .map(Rc::<str>::from))
  });
  let embed_video_url_exists =
    Memo::new(move |_| with!(|post_state| post_state.post.embed_video_url.is_some()));

  with!(|url, embed_video_url_exists| {
    match url.as_deref() {
      url if *embed_video_url_exists || url.is_some_and(is_video) => view! {
        <button type="button" class="aspect-square rounded overflow-hidden inline-block relative bg-transparent p-0 border-0 grid-in-thumbnail">
          {thumbnail.get().map(|thumbnail| view! {
            <img class="w-16" src=thumbnail/>
          })}
          <Icon icon=IconType::Video class="block text-white/75 absolute end-0 top-0" size=IconSize::Small/>
        </button>
      }
      .into_view(),
      Some(url) if !is_image(url) => view! {
        <A href=url.to_owned() class="aspect-square rounded overflow-hidden inline-block relative bg-transparent p-0 border-0 grid-in-thumbnail">
          {thumbnail.get().map(|thumbnail| view! {
            <img class="w-16" src=thumbnail/>
          })}
          <Icon icon=IconType::ExternalLink class="block text-white/75 absolute end-0 top-0" size=IconSize::Small/>
        </A>
      }.into_view(),
      Some(_) => view! {
        <button type="button" class="aspect-square rounded overflow-hidden inline-block relative bg-transparent p-0 border-0 grid-in-thumbnail">
          {thumbnail.get().map(|thumbnail| view! {
            <img class="w-16" src=thumbnail/>
          })}
          <Icon icon=IconType::Image class="block text-white/75 absolute end-0 top-0" size=IconSize::Small/>
        </button>
      }.into_view(),
      _ => view! {
          <A href=format!("/post/{id}") class="w-16 grid-in-thumbnail">
            <Icon icon=IconType::Comments class = "m-auto" size = IconSize::ExtraLarge />
          </A>
        }.into_view(),
    }
  })
}

use super::PostUrl;
use crate::{
  ui::components::common::icon::IconType,
  utils::{is_image, is_video},
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::newtypes::PostId,
  lemmy_db_views::structs::PostView,
};
use leptos::*;
use std::rc::Rc;
use thumbnail_inner::ThumbnailInner;

mod thumbnail_inner;

#[component]
pub fn Thumbnail(post_state: ReadSignal<PostView>, id: PostId) -> impl IntoView {
  let url = expect_context::<Memo<Option<PostUrl>>>();
  let embed_video_url_exists =
    Memo::new(move |_| with!(|post_state| post_state.post.embed_video_url.is_some()));

  move || {
    match url.get() {
      url if embed_video_url_exists.get() || url.as_ref().is_some_and(|url| is_video(url.0.as_ref())) => view! { <ThumbnailInner icon=IconType::Video /> }
      .into_view(),
      Some(url) if !is_image(&url.0) => view! { <ThumbnailInner icon=IconType::ExternalLink url=url /> }.into_view(),
      Some(_) => view! { <ThumbnailInner icon=IconType::Image /> }.into_view(),
      _ => view! { <ThumbnailInner icon=IconType::Comments url=PostUrl(Rc::from(format!("/post/{id}"))) /> }.into_view(),
    }
  }
}

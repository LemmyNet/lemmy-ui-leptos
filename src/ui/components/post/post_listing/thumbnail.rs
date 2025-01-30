use crate::{
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::{is_image, is_video},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::newtypes::PostId;
use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use std::sync::Arc;
use tailwind_fuse::{AsTailwindClass, TwVariant};

#[derive(TwVariant)]
enum ThumbnailIconType {
  #[tw(default, class = "m-auto")]
  NoImage,
  #[tw(class = "rounded-bl bg-slate-600/75 block text-white/75 absolute right-0 top-0")]
  Image,
}

#[derive(TwVariant)]
#[tw(class = "border-2 border-neutral size-20 rounded-lg p-0 grid-in-thumbnail")]
enum ThumbnailWrapperType {
  #[tw(default, class = "flex")]
  NoImage,
  #[tw(class = "relative overflow-hidden")]
  Image,
}

#[derive(Clone)]
struct ThumbnailData {
  pub icon: IconType,
  pub icon_size: IconSize,
  pub wrapper_class: Arc<str>,
  pub icon_class: Arc<str>,
}

#[component]
pub fn Thumbnail(
  url: Memo<Option<Arc<str>>>,
  image_url: Memo<Option<Arc<str>>>,
  #[prop(into)] has_embed_url: Signal<bool>,
  id: PostId,
) -> impl IntoView {
  let thumbnail_data = Signal::derive(move || {
    // When there is a thumbnail URL, use the normal size icon and use classes to display the icon in the upper right corner
    let (icon_size, icon_class, wrapper_class) = if image_url.read().is_some() {
      (
        IconSize::Normal,
        ThumbnailIconType::Image.as_class().into(),
        ThumbnailWrapperType::Image.as_class().into(),
      )
    }
    // When there isn't a thumbnail URL, use a larger icon that gets centered in the thumbnail
    else {
      (
        IconSize::ExtraLarge,
        ThumbnailIconType::NoImage.as_class().into(),
        ThumbnailWrapperType::NoImage.as_class().into(),
      )
    };

    let icon = match url.read().as_ref().map(Arc::clone) {
      url if *has_embed_url.read() || url.as_ref().is_some_and(|url| is_video(url.as_ref())) => {
        IconType::Video
      }
      // Video URLs are handled in the previous case, so if the URL isn't an image, it must be an external link
      Some(url) if !is_image(url.as_ref()) => IconType::ExternalLink,
      // Since there are already cases for video and external links URLs, the only other possible type of URL it can be is an image
      Some(_) => IconType::Image,
      None => IconType::Comments,
    };

    ThumbnailData {
      icon,
      icon_size,
      wrapper_class,
      icon_class,
    }
  });

  move || {
    let wrapper_class = Arc::clone(&thumbnail_data.read().wrapper_class);

    if matches!(
      thumbnail_data.read().icon,
      IconType::ExternalLink | IconType::Comments
    ) {
      Either::Left(view! {
        <A
          href=url
            .read()
            .as_ref()
            .map(Arc::clone)
            .map(|url| url.to_string())
            .unwrap_or_else(|| format!("/post/{id}"))
          attr:class=wrapper_class
        >
          <Inner image_url=image_url thumbnail_data=thumbnail_data />
        </A>
      })
    } else {
      Either::Right(view! {
        <button type="button" class=wrapper_class>
          <Inner image_url=image_url thumbnail_data=thumbnail_data />

        </button>
      })
    }
  }
}

#[component]
fn Inner(
  image_url: Memo<Option<Arc<str>>>,
  thumbnail_data: Signal<ThumbnailData>,
) -> impl IntoView {
  view! {
    {move || {
      image_url
        .read()
        .as_ref()
        .map(Arc::clone)
        .map(|thumbnail| {
          view! { <img class="size-full object-cover aspect-square" src=thumbnail /> }
        })
    }}
    <Icon
      icon=thumbnail_data.read().icon
      class=Arc::clone(&thumbnail_data.read().icon_class)
      size=thumbnail_data.read().icon_size
    />
  }
}

use crate::{
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::{is_image, is_video},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::newtypes::PostId;
use leptos::*;
use leptos_router::A;
use std::rc::Rc;
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
  pub wrapper_class: Rc<str>,
  pub icon_class: Rc<str>,
}

#[component]
pub fn Thumbnail(
  url: Memo<Option<Rc<str>>>,
  image_url: Memo<Option<Rc<str>>>,
  #[prop(into)] has_embed_url: Signal<bool>,
  id: PostId,
) -> impl IntoView {
  let thumbnail_data = Signal::derive(move || {
    with!(|url, image_url, has_embed_url| {
      let (icon_size, icon_class, wrapper_class) = if image_url.is_some() {
        (
          IconSize::Normal,
          ThumbnailIconType::Image.as_class().into(),
          ThumbnailWrapperType::Image.as_class().into(),
        )
      } else {
        (
          IconSize::ExtraLarge,
          ThumbnailIconType::NoImage.as_class().into(),
          ThumbnailWrapperType::NoImage.as_class().into(),
        )
      };

      let icon = match url.as_ref().map(Rc::clone) {
        url if *has_embed_url || url.as_ref().is_some_and(|url| is_video(url.as_ref())) => {
          IconType::Video
        }
        Some(url) if !is_image(url.as_ref()) => IconType::ExternalLink,
        Some(_) => IconType::Image,
        None => IconType::Comments,
      };

      ThumbnailData {
        icon,
        icon_size,
        wrapper_class,
        icon_class,
      }
    })
  });

  let inner = Signal::derive(move || {
    with!(|thumbnail_data| view! {
      {move || {
        image_url
            .get()
            .map(|thumbnail| {
                view! { <img class="size-full object-cover aspect-square" src=thumbnail /> }
            })
    }}
    <Icon icon=thumbnail_data.icon class=Rc::clone(&thumbnail_data.icon_class) size=thumbnail_data.icon_size />
    })
  });

  move || {
    with!(|url, thumbnail_data| {
      let wrapper_class = Rc::clone(&thumbnail_data.wrapper_class);
      if matches!(
        thumbnail_data.icon,
        IconType::ExternalLink | IconType::Comments
      ) {
        view! {
          <A href=url.as_ref().map(ToString::to_string).unwrap_or_else(|| format!("/post/{id}")) class=wrapper_class>
            {inner}
          </A>
        }.into_view()
      } else {
        view! {
          <button type="button" class=wrapper_class>
            {inner}
          </button>
        }
        .into_view()
      }
    })
  }
}
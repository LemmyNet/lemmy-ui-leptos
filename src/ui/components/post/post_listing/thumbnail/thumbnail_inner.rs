use crate::ui::components::{
  common::icon::{Icon, IconSize, IconType},
  post::post_listing::{PostImage, PostUrl},
};
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

#[component]
pub fn ThumbnailInner(icon: IconType, #[prop(optional)] url: Option<PostUrl>) -> impl IntoView {
  let post_image = expect_context::<Memo<Option<PostImage>>>();

  let wrapper_class = Signal::derive(move || {
    Rc::<str>::from(
      if post_image.get().is_some() {
        ThumbnailWrapperType::Image
      } else {
        ThumbnailWrapperType::NoImage
      }
      .as_class(),
    )
  });
  let icon_class = Signal::derive(move || {
    Rc::<str>::from(
      if post_image.get().is_some() {
        ThumbnailIconType::Image
      } else {
        ThumbnailIconType::NoImage
      }
      .as_class(),
    )
  });

  let icon_size = Signal::derive(move || {
    if post_image.get().is_some() {
      IconSize::Normal
    } else {
      IconSize::ExtraLarge
    }
  });

  let inner = Signal::derive(move || {
    view! {
      {move || {
          post_image
              .get()
              .map(|thumbnail| {
                  view! { <img class="size-full object-cover aspect-square" src=thumbnail.0 /> }
              })
      }}
      <Icon icon=icon class=move || icon_class.get() size=icon_size />
    }
  });

  if let Some(url) = url {
    view! {
      <A href=url.0.to_string() class=move || wrapper_class.get()>
        {inner}
      </A>
    }
    .into_view()
  } else {
    view! {
      <button type="button" class=move || wrapper_class.get()>
        {inner}
      </button>
    }
    .into_view()
  }
}

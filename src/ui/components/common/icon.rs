use leptos::*;
use strum::{EnumString, IntoStaticStr};
use tailwind_fuse::{tw_merge, AsTailwindClass, TwVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
#[non_exhaustive]
pub enum IconType {
  Eye,
  EyeSlash,
  Notifications,
  Donate,
  Search,
  Upvote,
  Downvote,
  Crosspost,
  VerticalDots,
  Report,
  Comment,
  Comments,
  Block,
  Save,
  SaveFilled,
  Saved,
  CreatePost,
  CreateCommunity,
  Communities,
  Community,
  Documentation,
  Code,
  Info,
  Modlog,
  Instances,
  Legal,
  Theme,
  DropdownCaret,
  Home,
  Profile,
  Hamburger,
  Users,
  Posts,
  Fediverse,
  X,
  Image,
  Video,
  ExternalLink,
  Clock,
  Language,
}

#[derive(Debug, TwVariant)]
pub enum IconSize {
  #[tw(default, class = "size-6")]
  Normal,
  #[tw(class = "size-9")]
  Large,
  #[tw(class = "size-12")]
  ExtraLarge,
  #[tw(class = "size-3")]
  Small,
}

#[component]
pub fn Icon(
  #[prop(into)] icon: MaybeSignal<IconType>,
  #[prop(into, default = TextProp::from(""))] class: TextProp,
  #[prop(into, default = MaybeSignal::Static(IconSize::Normal))] size: MaybeSignal<IconSize>,
) -> impl IntoView {
  let href =
    Signal::derive(move || format!("/icons.svg#{}", Into::<&'static str>::into(icon.get())));
  let class = Signal::derive(move || tw_merge!(class.get().to_string(), size.get()));

  view! {
    <svg class=class aria-hidden="true">
      <use_ href=href xlink:href=href></use_>
    </svg>
  }
}

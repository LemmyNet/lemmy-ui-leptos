use leptos::*;
use strum::{EnumString, IntoStaticStr};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
  Normal,
  Large,
  ExtraLarge,
  Small,
}

impl IntoAttribute for IconSize {
  fn into_attribute(self) -> Attribute {
    match self {
      Self::ExtraLarge => "3rem",
      Self::Large => "2.25rem",
      Self::Small => "0.75rem",
      _ => "1.5rem",
    }
    .into_attribute()
  }

  fn into_attribute_boxed(self: Box<Self>) -> Attribute {
    self.into_attribute()
  }
}

#[component]
pub fn Icon(
  #[prop(into)] icon: MaybeSignal<IconType>,
  #[prop(optional, into)] class: MaybeProp<TextProp>,
  #[prop(into, default = MaybeSignal::Static(IconSize::Normal))] size: MaybeSignal<IconSize>,
) -> impl IntoView {
  let href =
    Signal::derive(move || format!("/icons.svg#{}", Into::<&'static str>::into(icon.get())));

  view! {
    <svg class=class width=size height=size aria-hidden="true">
      <use_ href=href xlink:href=href></use_>
    </svg>
  }
}

use leptos::*;
use strum::{EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
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
  CreatePost,
  CreateCommunity,
  Communities,
  Documentation,
  Code,
  Info,
  Modlog,
  Instances,
  Legal,
}

#[component]
pub fn Icon(
  #[prop(into)] icon: MaybeSignal<IconType>,
  #[prop(optional, into)] class: MaybeProp<TextProp>,
  #[prop(into, default = MaybeSignal::Static(false))] large: MaybeSignal<bool>,
) -> impl IntoView {
  let href =
    Signal::derive(move || format!("/icons.svg#{}", Into::<&'static str>::into(icon.get())));
  let size = Signal::derive(move || if large.get() { "3em" } else { "1.5em" });

  view! {
    <svg class=class width=size height=size>
      <use_ href=href xlink:href=href></use_>
    </svg>
  }
}

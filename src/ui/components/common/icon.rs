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
  Block,
  Save,
}

#[component]
pub fn Icon(
  #[prop(into)] icon: MaybeSignal<IconType>,
  #[prop(optional, into)] class: MaybeProp<TextProp>,
) -> impl IntoView {
  let href =
    Signal::derive(move || format!("/icons.svg#{}", Into::<&'static str>::into(icon.get())));

  view! {
    <svg class=class width="1.5em" height="1.5em">
      <use_ href=href xlink:href=href></use_>
    </svg>
  }
}

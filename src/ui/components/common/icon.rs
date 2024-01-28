use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
  Comments,
  Block,
  Save,
}

impl IconType {
  pub fn as_str(&self) -> &'static str {
    match self {
      IconType::Block => "block",
      IconType::Comments => "comments",
      IconType::Crosspost => "crosspost",
      IconType::Donate => "donate",
      IconType::Downvote => "downvote",
      IconType::Eye => "eye",
      IconType::EyeSlash => "eye-slash",
      IconType::Notifications => "notifications",
      IconType::Report => "report",
      IconType::Save => "save",
      IconType::Search => "search",
      IconType::Upvote => "upvote",
      IconType::VerticalDots => "vertical-dots",
    }
  }
}

#[component]
pub fn Icon(
  #[prop(into)] icon: MaybeSignal<IconType>,
  #[prop(optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
  let href = Signal::derive(move || format!("/icons.svg#{}", icon.get().as_str()));

  view! {
    <svg class=class width="1.5em" height="1.5em">
      <use_ href=href xlink:href=href></use_>
    </svg>
  }
}

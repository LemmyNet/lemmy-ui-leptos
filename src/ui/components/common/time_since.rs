use crate::{
  ui::components::common::icon::{Icon, IconSize, IconType},
  utils::get_time_since,
};
use chrono::{DateTime, Utc};
use leptos::*;

#[component]
pub fn TimeSince(#[prop(into)] datetime: Signal<DateTime<Utc>>) -> impl IntoView {
  let time_str = Signal::derive(move || with!(|datetime| datetime.to_rfc3339()));
  let time_since = Signal::derive(move || with!(|datetime| get_time_since(datetime)));

  view! {
    <time datetime=time_str class="text-xs badge badge-ghost gap-x-0.5">
      <Icon icon=IconType::Clock size=IconSize::Small />
      {time_since}
    </time>
  }
}

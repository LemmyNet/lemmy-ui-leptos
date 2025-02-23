use leptos::prelude::*;
use pretty_num::PrettyNumber;

#[component]
pub fn UserStatRow(count: i64, text: Signal<String>) -> impl IntoView {
  view! {
    <tr class="*:p-2.5 not-first:border-t-2 not-first:border-accent">
      <td class="text-xs font-semibold">{text}</td>
      <td class="text-center font-bold">{count.pretty_format()}</td>
    </tr>
  }
}

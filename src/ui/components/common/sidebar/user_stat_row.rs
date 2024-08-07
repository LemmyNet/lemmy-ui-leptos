use leptos::*;
use si_format::Formattable;

#[component]
pub fn UserStatRow(count: i64, text: Signal<String>) -> impl IntoView {
  view! {
    <tr class="*:p-2.5 [&:not(:first-child)]:border-t-2 [&:not(:first-child)]:border-accent">
      <td class="text-xs font-semibold">{text}</td>
      <td class="text-center font-bold">{count.si_format().to_string()}</td>
    </tr>
  }
}

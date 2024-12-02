use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn CommunitiesPage() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">{move_tr!("communities")}</h2>
    </main>
  }
}

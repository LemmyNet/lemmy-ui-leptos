use leptos::*;
use leptos_fluent::tr;

#[component]
pub fn CommunitiesPage() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">{tr!("communities")}</h2>
    </main>
  }
}

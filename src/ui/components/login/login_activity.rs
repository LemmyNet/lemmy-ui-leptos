use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginActivity(cx: Scope) -> impl IntoView {
  view! { cx,
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Login Activity"</h2>
      <LoginForm/>

    </main>
  }
}

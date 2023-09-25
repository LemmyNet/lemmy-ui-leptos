use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginActivity() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Login Activity"</h2>
      <LoginForm/>

    </main>
  }
}

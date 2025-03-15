use crate::ui::components::login::login_form::LoginForm;
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <main class="max-w-(--breakpoint-sm) mx-auto max-w-(--breakpoint-md) p-3">
      <LoginForm />
    </main>
  }
}

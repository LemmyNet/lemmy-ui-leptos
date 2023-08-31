use crate::{api::login::login, ui::components::login::login_form::LoginForm};
use lemmy_api_common::person::Login;
use leptos::*;

#[component]
pub fn LoginActivity(cx: Scope) -> impl IntoView {
  view! { cx,
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Login Activity"</h2>
      <LoginForm />

    </main>
  }
}

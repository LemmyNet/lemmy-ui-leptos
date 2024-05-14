use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <main class="mx-auto max-w-screen-md p-3">
      <LoginForm/>
    </main>
  }
}

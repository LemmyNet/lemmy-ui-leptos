use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginActivity() -> impl IntoView {
  view! {
      <main class="mx-auto max-w-screen-md p-4">
          <LoginForm/>
      </main>
  }
}

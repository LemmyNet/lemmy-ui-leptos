use crate::ui::components::login::login_form::LoginForm;
use leptos::*;

#[component]
pub fn LoginActivity() -> impl IntoView {
  view! {
    // <div class="w-full flex flex-col sm:flex-row flex-grow">
    //   <div class="sm:container sm:mx-auto">
    //     <div class="w-full flex flex-col sm:flex-row flex-grow">
          // <main role="main" class="w-full h-full flex-grow sm:p-3">
          <main class="mx-auto max-w-screen-md p-3">
            <LoginForm/>
          </main>
    //     </div>
    //   </div>
    // </div>
  }
}

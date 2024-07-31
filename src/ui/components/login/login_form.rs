use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::auth::create_login_action,
  ui::components::common::text_input::{InputType, TextInput},
};
use leptos::*;
use leptos_fluent::tr;
use leptos_router::ActionForm;

#[component]
pub fn LoginForm() -> impl IntoView {
  let login = create_login_action();
  let site_resource = expect_context::<SiteResource>();
  // TODO: make unified, better looking way of handling errors.
  let login_error = Signal::derive(move || {
    login.value().get().and_then(|v| {
      v.map_err(|e| view! { <div class="text-error">{e.to_string()}</div> })
        .err()
    })
  });

  Effect::new(move |_| {
    if login.value().get().is_some_and(|r| r.is_ok()) {
      site_resource.refetch();
    }
  });

  view! {
    <ActionForm class="space-y-3" action=login>
      {login_error}
      <TextInput
        id="username"
        name="username_or_email"
        label={tr!("username")}
        required=true
        min_length=3
      />

      <TextInput
        id="password"
        name="password"
        label={tr!("password")}
        input_type=InputType::Password
        pattern=".{10,60}"
        required=true
      />

      <button class="btn btn-lg" type="submit">
        {tr!("login")}
      </button>
    </ActionForm>
  }
}

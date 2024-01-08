use leptos::*;
use leptos_icons::*;

#[component]
pub fn PasswordInput<F: Fn(String) + 'static>(
  #[prop(optional)] disabled: MaybeProp<bool>,
  #[prop(into)] id: &'static str,
  #[prop(into)] name: &'static str,
  #[prop(optional)] validation_class: MaybeSignal<String>,
  on_input: F,
) -> impl IntoView {
  let (show_password, set_show_password) = create_signal(false);

  view! {
    <div class="form-control w-full">
      <label class="label" for=id>
        <span class="label-text">Password</span>
      </label>
      <div class="join">
        <input
          type=move || show_password.with(|s| if *s { "text " } else { "password" })
          id=id
          class=move || format!("input input-bordered join-item w-full {}", validation_class.get())
          
          name=name
          disabled=move || disabled.get().unwrap_or(false)
          on:input=move |e| {
              on_input(event_target_value(&e));
          }
        />

        <button
          type="button"
          class="btn btn-outline join-item rounded-r-full btn-primary"
          on:click=move |_| set_show_password.update(|s| *s = !*s)
        >
          <Show
            when=show_password
            fallback=|| {
                view! { <Icon icon=Icon::from(ChIcon::ChEye) width="2.5rem" height="2.5rem"/> }
            }
          >

            <Icon icon=Icon::from(ChIcon::ChEyeSlash) width="2.5rem" height="2.5rem"/>
          </Show>
        </button>
      </div>
    </div>
  }
}

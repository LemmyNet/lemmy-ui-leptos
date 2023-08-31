use leptos::*;
use leptos_heroicons::size_24::outline::{Eye, EyeSlash};

#[component]
pub fn PasswordInput(
  cx: Scope,
  #[prop(optional)] disabled: MaybeProp<bool>,
  #[prop(into)] id: &'static str,
  #[prop(into)] name: &'static str,
) -> impl IntoView {
  let (show_password, set_show_password) = create_signal(cx, false);

  view! { cx,
    <div class="form-control w-full">
      <label class="label" for=id>
        <span class="label-text">
          Password
        </span>
      </label>
      <div class="join">
        <input
          type=move || if show_password() { "text " } else { "password" }
          id=id
          class="input input-bordered join-item w-full"
          required
          name=name
          disabled=move || disabled.get().unwrap_or(false)
        />
        <button
          type="button"
          class="btn btn-outline join-item rounded-r-full btn-primary"
          on:click=move |_| set_show_password.update(|s| *s = !*s)
        >
          <Show when=show_password fallback=|cx| view! { cx, <Eye class="w-8"/> }>
            <EyeSlash class="w-8"/>
          </Show>
        </button>
      </div>
    </div>
  }
}

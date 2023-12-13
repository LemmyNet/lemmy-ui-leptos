use leptos::*;
use phosphor_leptos::{Eye, EyeSlash};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
  Text,
  Password,
}

#[component]
pub fn PasswordInput(
  #[prop(optional)] disabled: MaybeProp<bool>,
  #[prop(optional)] required: MaybeProp<bool>,
  #[prop(into)] id: TextProp,
  #[prop(into)] name: TextProp,
  #[prop(into)] label: TextProp,
  #[prop(into)] on_input: Callback<String, ()>,
  #[prop(default = InputType::Text)] input_type: InputType,
) -> impl IntoView {
  let show_password = RwSignal::new(false);
  let for_id = id.get().clone();

  view! {
    <div class="relative w-full !mt-8">
      <input
        type=move || {
            if input_type == InputType::Text || show_password.get() { "text" } else { "password" }
        }

        id=id
        class="peer input w-full pe-10 input-bordered border-x-0 border-t-0 rounded-b-none border-b-2 focus:outline-none bg-base-200/50"
        required
        placeholder=" "
        name=move || name.get()
        disabled=move || disabled.get().unwrap_or(false)
        required=move || required.get().unwrap_or(false)
        on:input=move |e| {
            on_input.call(event_target_value(&e));
        }
      />
      <Show when=move || input_type == InputType::Password>
        <button
          type="button"
          class="btn btn-ghost btn-sm btn-circle absolute end-1 bottom-2 text-accent"
          on:click=move |_| update!(| show_password | * show_password = !* show_password)
        >
        <Show when=move || show_password.get()
              fallback=move || view!{<Eye />} >
          <EyeSlash />
          </Show>
          {move || if show_password.get() { "true" } else { "false" }}
        </button>
      </Show>
      <label
        class="label absolute inset-y-0 start-2 transition-all peer-placeholder-shown:text-neutral/50 peer-[:not(:placeholder-shown)]:-top-20 peer-focus:text-current peer-[:not(:placeholder-shown)]:start-0 peer-[:not(:placeholder-shown)]:text-sm peer-focus:text-sm peer-focus:-top-20 peer-focus:start-0 pointer-events-none select-none"
        for=for_id
      >
        {label}
      </label>
    </div>
  }
}

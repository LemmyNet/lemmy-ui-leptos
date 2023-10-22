use leptos::*;
use leptos_icons::*;

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
  let icon = Signal::derive(move || {
    if show_password() {
      Icon::from(ChIcon::ChEyeSlash)
    } else {
      Icon::from(ChIcon::ChEye)
    }
  });

  view! {
    <div class="relative w-full !mt-8">
      <input
        type=move || {
            if input_type == InputType::Text || show_password() { "text" } else { "password" }
        }

        id=move || id.get()
        class="peer input w-full pe-10 input-bordered border-x-0 border-t-0 rounded-b-none border-b-2 focus:outline-none bg-base-200/50"
        required
        placeholder=" "
        name=move || name.get()
        disabled=move || disabled().unwrap_or(false)
        required=move || required().unwrap_or(false)
        on:input=move |e| {
            on_input(event_target_value(&e));
        }
      />
      <Show when=move || input_type == InputType::Password>
        <button
          type="button"
          class="btn btn-ghost btn-sm btn-circle absolute end-1 bottom-2 text-accent"
          on:click=move |_| update!(| show_password | * show_password = !* show_password)
        >
          <Icon
            icon=icon
            width="1.5rem"
            height="1.5rem"
          />
          {move || if show_password() { "true" } else { "false" }}
        </button>
      </Show>
      <label
        class="label absolute inset-y-0 start-2 transition-all peer-placeholder-shown:text-neutral/50 peer-[:not(:placeholder-shown)]:-top-20 peer-focus:text-current peer-[:not(:placeholder-shown)]:start-0 peer-[:not(:placeholder-shown)]:text-sm peer-focus:text-sm peer-focus:-top-20 peer-focus:start-0 pointer-events-none select-none"
        for=for_id
      >
        {move || label.get().to_string()}
      </label>
    </div>
  }
}

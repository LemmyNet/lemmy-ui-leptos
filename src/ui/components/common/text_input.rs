use crate::ui::components::common::icon::{
  Icon,
  IconType::{Eye, EyeSlash},
};
use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
  Text,
  Password,
}

#[component]
pub fn TextInput(
  #[prop(optional)] disabled: MaybeProp<bool>,
  #[prop(optional)] required: MaybeProp<bool>,
  #[prop(into)] id: TextProp,
  #[prop(into)] name: TextProp,
  #[prop(into)] label: TextProp,
  #[prop(into)] on_input: Callback<String, ()>,
  #[prop(default = InputType::Text)] input_type: InputType,
  #[prop(optional)] validation_class: MaybeSignal<String>,
) -> impl IntoView {
  let show_password = RwSignal::new(false);
  let for_id = id.get().clone();
  let eye_icon =
    Signal::derive(move || with!(|show_password| if *show_password { EyeSlash } else { Eye }));

  view! {
    <div class="relative w-full !mt-8">
      <input
        type=move || {
            if input_type == InputType::Text || show_password.get() { "text" } else { "password" }
        }

        id=id
        class=move || {
            format!(
                "peer input w-full pe-10 input-bordered border-x-0 border-t-0 rounded-b-none border-b-2 focus:outline-none bg-base-200/50 {}",
                validation_class.get(),
            )
        }

        placeholder=" "
        name=move || name.get()
        disabled=move || disabled.get().unwrap_or(false)
        required=move || required.get().unwrap_or(false)
        on:input=move |e| {
            // Need the fully qualified syntax due to possible future collison with an addition to the standard library. See https://doc.rust-lang.org/std/ops/trait.Fn.html#tymethod.call
            leptos::Callable::call(&on_input, event_target_value(&e));
        }
      />

      <Show when=move || input_type == InputType::Password>
        <button
          type="button"
          class="btn btn-ghost btn-sm btn-circle absolute end-1 bottom-2 text-accent"
          on:click=move |_| update!(| show_password | * show_password = !* show_password)
        >
          <Icon icon=eye_icon/>
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

use leptos::*;

#[component]
pub fn Unpack<T: Clone + 'static, F: Fn(T) -> Fragment + 'static>(
  #[prop(into)] item: MaybeSignal<Option<Result<T, ServerFnError>>>,
  children: F,
) -> impl IntoView {
  Signal::derive(move || match item.get() {
    Some(Ok(item)) => Some(Ok(children(item))),
    Some(Err(e)) => Some(Err(e.clone())),
    _ => None,
  })
}

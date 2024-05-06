use leptos::*;

#[component]
pub fn Unpack<T, F, S>(item: S, children: F) -> impl IntoView
where
  T: Clone + 'static,
  F: Fn(T) -> Fragment + 'static,
  S: SignalGet<Value = Option<Result<T, ServerFnError>>> + 'static,
{
  Signal::derive(move || match item.get() {
    Some(Ok(item)) => Some(Ok(children(item))),
    Some(Err(e)) => Some(Err(e.clone())),
    _ => None,
  })
}

use leptos::{with, ServerFnError, Signal, SignalWith};

pub fn derive_query_signal<T, R, S>(
  base_signal: S,
  map_result: fn(&T) -> R,
) -> Signal<Option<Result<R, ServerFnError>>>
where
  T: 'static,
  S: SignalWith<Value = Option<Result<T, ServerFnError>>> + 'static,
{
  Signal::derive(move || {
    with!(|base_signal| base_signal
      .as_ref()
      .map(|base_signal| base_signal.as_ref().map_err(Clone::clone).map(map_result)))
  })
}

use leptos::{with, ServerFnError, Signal};

pub fn derive_query_signal<T, R>(
  base_signal: Signal<Option<Result<T, ServerFnError>>>,
  map_result: fn(&T) -> R,
) -> Signal<Option<Result<R, ServerFnError>>> {
  Signal::derive(move || {
    with!(|base_signal| base_signal
      .as_ref()
      .map(|base_signal| base_signal.as_ref().map_err(Clone::clone).map(map_result)))
  })
}

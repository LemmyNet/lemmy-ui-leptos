use super::types::QuerySignal;
use leptos::prelude::{Read, ServerFnError, Signal};

pub fn derive_query_signal<T, R, S>(base_signal: S, map_result: fn(&T) -> R) -> QuerySignal<R>
where
  T: 'static,
  S: Read<Value = Option<Result<T, ServerFnError>>> + 'static,
  R: 'static + Send + Sync,
{
  Signal::derive(move || {
    base_signal
      .read()
      .as_ref()
      .map(|base_signal| base_signal.as_ref().map_err(Clone::clone).map(map_result))
  })
}

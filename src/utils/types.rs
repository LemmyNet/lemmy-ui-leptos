use leptos::{ServerFnError, Signal};

mod theme;
pub use theme::Theme;

mod server_action;
pub use server_action::*;

mod dialog_types;
pub use dialog_types::*;

mod content_action_types;
pub use content_action_types::*;

mod comment_tree;
pub use comment_tree::*;

pub type QuerySignal<R> = Signal<Option<Result<R, ServerFnError>>>;

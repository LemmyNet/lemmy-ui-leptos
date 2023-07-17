use leptos::{component, view, IntoView, Scope};
use leptos_router::*;

#[component]
pub fn BottomNav(cx: Scope) -> impl IntoView {
  view! { cx,
    <footer class="sticky bottom-0">
      <div class="btm-nav btm-nav-lg">
        <A href="/" class="active">
          // TODO put svg's here
          <span class="btm-nav-label">"Home"</span>
        </A>
        <button>
          <span class="btm-nav-label">"TODO 1"</span>
        </button>
        <button>
          <span class="btm-nav-label">"TODO 2"</span>
        </button>
      </div>
    </footer>
  }
}

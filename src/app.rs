use crate::api;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);

  view! { cx,
    <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Router>
      <Routes>
        <Route path="" view=move |cx| view! { cx, <Home/> }/>
      </Routes>
    </Router>
  }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
  let (count, set_count) = create_signal(cx, 0);

  let posts = create_resource(
    cx,
    move || 2,
    move |_| async move { api::fetch_posts(cx).await },
  );
  let (pending, set_pending) = create_signal(cx, false);

  view! { cx,
    <main class="my-0 mx-auto max-w-3xl text-center">
      <h2 class="p-6 text-4xl">"Not there. Welcome to Leptos with Tailwind"</h2>
      <p class="px-10 pb-10 text-left">
        "Tailwinds will scan your Rust files for Tailwind class names and compile them into a CSS file."
      </p>
      <button class="btn" on:click=move |_| set_count.update(|count| *count += 1)>
        "Something's here | "
        {move || { if count() == 0 { "Click mehhhh!".to_string() } else { count().to_string() } }}
        " | Some more text"
      </button>
      <Suspense fallback=|| {
          view! { cx, "Loading..." }
      }>
        {move || {
            posts
                .read(cx)
                .map(|postRes| match postRes {
                    None => {
                        view! { cx, <div class="item-view">"Error loading this story."</div> }
                    }
                    Some(postRes) => {
                        view! { cx,
                          <div>
                            <For
                              each=move || postRes.posts.clone()
                              key=|pv| pv.post.id
                              view=move |cx, pv| view! { cx, <div>{pv.post.name}</div> }
                            />
                          </div>
                        }
                    }
                })
        }}
      </Suspense>
    </main>
  }
}

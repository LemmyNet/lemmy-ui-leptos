use cfg_if::cfg_if;
use leptos_meta::MetaTags;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lemmy_ui_leptos::{App, cookie_middleware::cookie_middleware, host::get_client};
        use actix_files::Files;
        use actix_web::{*, App as ActixApp};
        use leptos::prelude::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        macro_rules! asset_route {
            ($name:ident, $file:expr) => {
                #[actix_web::get($file)]
                async fn $name(
                    leptos_options: web::Data<leptos::prelude::LeptosOptions>
                ) -> impl actix_web::Responder {
                    let leptos_options = leptos_options.into_inner();
                    let site_root = &leptos_options.site_root;
                    actix_files::NamedFile::open_async(format!("./{site_root}{}", $file)).await
                }
            };
        }

        asset_route!(favicon, "/favicon.svg");
        asset_route!(icons, "/icons.svg");
        asset_route!(default_avatar, "/default-avatar.png");

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).unwrap();
            let addr = conf.leptos_options.site_addr;

            HttpServer::new(move || {
                let routes = generate_route_list(App);
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                ActixApp::new()
                    .route("/serverfn/{tail:.*}", leptos_actix::handle_server_fns())
                    .wrap(cookie_middleware())
                    .service(favicon)
                    .service(icons)
                    .leptos_routes(
                        routes,
                        {
                        let options = leptos_options.clone();
                        move ||   view! {
                            <!DOCTYPE html>
                            <html>
                              <head>
                                <meta charset="utf-8" />
                                <link rel="shortcut icon" href="favicon.svg" type="image/svg+xml" />

                                // debug where there is no visible console (mobile/live/desktop)
                                <script src="//cdn.jsdelivr.net/npm/eruda"/>
                                <script>eruda.init();</script>

                                <meta name="description" content="Lemmy-UI-Leptos." />
                                <meta name="viewport" content="width=device-width, viewport-fit=cover, initial-scale=1" />

                                <AutoReload options=options.clone() />
                                <HydrationScripts options=options.clone()/>
                                <MetaTags/>
                              </head>
                              <body class="h-full max-h-screen flex flex-col overflow-y-hidden">
                                <App />
                              </body>
                            </html>
                          }
                        }
                    )
                    .app_data(web::Data::new(get_client()))
                    .app_data(web::Data::new(leptos_options.clone()))
                    .service(Files::new("/", site_root.as_ref()))
            })
            .bind(&addr)?
            .run()
            .await
        }
    } else {
        fn main() {
            use lemmy_ui_leptos::App;
            console_error_panic_hook::set_once();
            leptos::mount::mount_to_body(App);
        }
    }
}

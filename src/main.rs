use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lemmy_ui_leptos::{App, cookie_middleware::cookie_middleware, host::get_client};
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        macro_rules! asset_route {
            ($name:ident, $file:expr) => {
                #[actix_web::get($file)]
                async fn $name(
                    leptos_options: web::Data<leptos::LeptosOptions>
                ) -> impl actix_web::Responder {
                    let leptos_options = leptos_options.into_inner();
                    let site_root = &leptos_options.site_root;
                    actix_files::NamedFile::open_async(format!("{site_root}/{}", $file)).await
                }
            };
        }

        asset_route!(favicon, "favicon.svg");
        asset_route!(icons, "icons.svg");
        asset_route!(default_avatar, "default-avatar.png");

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr;

            let routes = generate_route_list(App);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .route("/serverfn/{tail:.*}", leptos_actix::handle_server_fns())
                    .wrap(cookie_middleware())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .service(Files::new("/assets", site_root))
                    .service(favicon)
                    .service(icons)
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        App
                    )
                    .app_data(web::Data::new(leptos_options.to_owned()))
                    .app_data(web::Data::new(get_client()))
            })
            .bind(&addr)?
            .run()
            .await
        }
    } else {
        fn main() {
            use lemmy_ui_leptos::App;
            console_error_panic_hook::set_once();
            leptos::mount_to_body(App);
        }
    }
}

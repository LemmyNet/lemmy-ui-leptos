use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lemmy_ui_leptos::{App, server::cookie_middleware::cookie_middleware, host::get_client};
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;

        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[actix_web::get("favicon.svg")]
        async fn favicon(
            leptos_options: web::Data<leptos::LeptosOptions>,
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.svg"))?)
        }

        #[actix_web::get("icons.svg")]
        async fn icons(
            leptos_options: web::Data<leptos::LeptosOptions>
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!("{site_root}/icons.svg"))?)
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr;

            leptos_query::suppress_query_load(true);
            let routes = generate_route_list(App);
            leptos_query::suppress_query_load(false);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .wrap(cookie_middleware())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .service(Files::new("/assets", site_root))
                    .service(favicon)
                    .service(icons)
                    .route("/serverfn/{tail:.*}", leptos_actix::handle_server_fns())
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

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use lemmy_ui_leptos::App;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style/output.css").await
        }

        #[actix_web::get("favicon.ico")]
        async fn favicon(
            leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!(
                "{site_root}/favicon.ico"
            ))?)
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .service(Files::new("/assets", site_root))
            .service(favicon)
                    .service(css)
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        |cx| view! { cx, <App/> },
                    )
                    .app_data(web::Data::new(leptos_options.to_owned()))
                //.wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }

    } else {
        fn main() {
            use lemmy_ui_leptos::App;
            use leptos::*;

            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(|cx| view! { cx, <App/> })
        }
    }
}

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use lemmy_ui_leptos::{App, api_service::route_to_api};
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use awc::Client;

        #[actix_web::get("favicon.svg")]
        async fn favicon(
            leptos_options: web::Data<leptos::LeptosOptions>,
        ) -> actix_web::Result<actix_files::NamedFile> {

            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!(
                "{site_root}/favicon.svg"
            ))?)
        }

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
                    .app_data(web::Data::new(Client::new()))
                    .route("/api/{tail:.*}", web::route()
                           .guard(guard::Any(guard::Get()).or(guard::Header("content-type", "application/json")))
                           .to(route_to_api))
                    .route("/serverfn/{tail:.*}", leptos_actix::handle_server_fns())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .service(Files::new("/assets", site_root))
                    .service(favicon)
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        App
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
            mount_to_body(App)
        }
    }
}

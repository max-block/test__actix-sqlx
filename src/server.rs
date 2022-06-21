use actix_files::Files;
use actix_web::{middleware, web, App as HttpApp, HttpServer};

use crate::{
    app::App,
    router::{data1_router, ui_router},
    template::Template,
};

pub async fn run() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", " actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let app = App::new(&database_url).await;
    let app = web::Data::new(app);

    let tpl = Template::new().unwrap();
    let tpl = web::Data::new(tpl);

    log::info!("starting HTTP server at http://localhost:3000");
    HttpServer::new(move || {
        HttpApp::new()
            .wrap(middleware::Logger::default())
            .app_data(tpl.clone())
            .app_data(app.clone())
            .service(Files::new("/static", "static"))
            .service(data1_router())
            .service(ui_router())
    })
    .bind("localhost:3000")?
    .run()
    .await
}

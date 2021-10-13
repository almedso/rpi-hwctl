use actix_web::{self, get, web, App, HttpServer, Responder};
use log::{debug, info};
use std::time::Duration;
use tokio::{self, time};

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    info!("Hello {}! id:{}", name, id);
    format!("Hello {}! id:{}", name, id)
}

async fn async_main() {
    tokio::spawn(async {
        let mut interval = time::interval(Duration::from_millis(1000));
        let mut tick = 0_u32;

        loop {
            interval.tick().await;
            tick += 1;
            debug!("Another tick passed: {}", tick);
            // do_something().await;
        }
    });

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
    })
    .workers(8)
    .bind("0.0.0.0:8088")
    .expect("Couldn't bind to port 8088")
    .run()
    .await
    .unwrap()
}

fn main() {
    std::env::set_var("RUST_LOG", "rpi_hwctl=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("main-tokio")
            .build()
            .unwrap()
    })
    .block_on(async_main());
}

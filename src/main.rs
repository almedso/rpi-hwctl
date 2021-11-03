use actix_web::{self, get, web, App, HttpServer, Responder};
use log::{debug, info};
use std::time::Duration;
use tokio::{self, time};

use rpi_hwctl::local_hmi::{Explorer700Display, HomePage};
use embedded_multi_page_hmi::{
    page::{ShutdownPage, StartupPage, TextPage},
    Interaction, PageBaseInterface, PageInteractionInterface, PageInterface, PageLifetime,
    PageManager, PageNavigation,
};

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    info!("Hello {}! id:{}", name, id);
    format!("Hello {}! id:{}", name, id)
}


// async fn print_events(m: &mut PageManager<'_, TerminalDisplay>) {
//     let mut reader = EventStream::new();
//     let mut navigation = m.dispatch(PageNavigation::SystemStart).unwrap();

//     loop {
//         let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
//         let mut event = reader.next().fuse();
//         let input: Option<Interaction>;

//         select! {
//             _ = delay => input  = None ,
//             maybe_event = event => {
//                 input = match maybe_event {
//                     Some(Ok(event)) => map_interaction(event),
//                     Some(Err(_e)) => None,
//                     None => None,
//                 };

//             },
//         };
//         let result = match input {
//             None => m.dispatch(navigation),
//             Some(interaction) => m.dispatch_interaction(interaction),
//         };
//         // in this example shutdown page returns PageError after it's lifetime is over
//         // this is used for a clean exit
//         match result {
//             Err(_e) => break,
//             Ok(nav) => navigation = nav,
//         };
//     }
// }

async fn async_main() {
    tokio::spawn(async {

        // hmi setup
        let display = Explorer700Display::new();
        let home = HomePage::new("!!! This is the home page !!!");
        let mut pm = PageManager::new(display, Box::new(home));
        let startup = StartupPage::new("Welcome message", 8);
        pm.register_startup(Box::new(startup));
        let mut navigation = pm.dispatch(PageNavigation::SystemStart).unwrap();
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;
            debug!("Navigation is: {:?}", navigation);
            navigation = pm.dispatch(navigation).unwrap();
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

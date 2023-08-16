use tokio::runtime::Runtime;
use actix_web::{web, App, HttpResponse, HttpServer};

pub struct DisbotterRESTApi {
    rt: Runtime
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

impl DisbotterRESTApi {
    pub fn new() -> Self {
        Self {
            rt: Runtime::new().unwrap()
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        self.rt.block_on(async {
            println!("Starting server...");
            HttpServer::new(|| {
                App::new()
                    .route("/ping", web::get().to(ping))
            })
            .bind(("127.0.0.1", 3000))?
            .run()
            .await
        })
    }
}
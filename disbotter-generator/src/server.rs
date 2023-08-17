use std::sync::Arc;

use tokio::runtime::Runtime;
use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};

use crate::compiler::{DisbotterProjectData, NodesJSCompiler, AvailableNode};

pub struct DisbotterRESTApi {
    rt: Runtime
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[derive(serde::Deserialize)]
struct CompileRequest {
    project: Option<DisbotterProjectData>,
}

async fn compile(mut req: web::Json<CompileRequest>, nodes: web::Data<Arc<Vec<AvailableNode>>>) -> HttpResponse {
    let project = match req.project.take() {
        Some(project) => project,
        None => {
            return HttpResponse::BadRequest().body("Missing project data");
        }
    };
    let mut compiler = NodesJSCompiler::new(project);
    compiler.add_available_nodes_from_vec(nodes.get_ref());
    let project = compiler.compile_project();

    match project {
        Ok(project) => {
            HttpResponse::Ok().body(project.export_to_string())
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to compile project: {:?}", err))
        }
    }
}

impl DisbotterRESTApi {
    pub fn new() -> Self {
        Self {
            rt: Runtime::new().unwrap()
        }
    }

    pub fn start(&self, nodes: Arc<Vec<AvailableNode>>) -> std::io::Result<()> {
        #[cfg(debug_assertions)]
        {
            println!("Starting server in debug mode...");
            std::env::set_var("RUST_LOG", "actix_web=debug");
            env_logger::try_init().ok();
        }

        self.rt.block_on(async {
            println!("Starting server...");
            
            HttpServer::new(move || {
                let cors = actix_cors::Cors::permissive()
                    .allowed_origin_fn(|origin, _| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    .allowed_origin("https://disbotter.olix3001.xyz");
                let nodes = nodes.clone();
                App::new()
                    .wrap(cors)
                    .wrap(Logger::default())
                    .app_data(web::Data::new(nodes))
                    .route("/ping", web::get().to(ping))
                    .route("/compile", web::post().to(compile))
            })
            .bind(("127.0.0.1", 3000))?
            .run()
            .await
        })
    }
}
use std::{sync::{Arc, Mutex}, path::PathBuf, io::Write, process::{Stdio, Child}};

use tokio::runtime::Runtime;
use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};

use crate::compiler::{DisbotterProjectData, NodesJSCompiler, AvailableNode};

pub struct DisbotterRESTApi {
    rt: Runtime,
}

#[derive(serde::Serialize)]
pub struct DisbotterRESTApiConfig {
    #[serde(skip)]
    pub nodes: Arc<Vec<AvailableNode>>,
    #[serde(skip)]
    pub projects: Option<PathBuf>,
    pub can_run: bool,

    pub def_token: String,
    pub def_client_id: String,
    pub def_guild_id: String,
}

struct DisbotterRESTApiState {
    currently_running: Mutex<Option<std::process::Child>>
}

impl Default for DisbotterRESTApiState {
    fn default() -> Self {
        Self {
            currently_running: Mutex::new(None)
        }
    }
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

async fn config(config: web::Data<DisbotterRESTApiConfig>) -> HttpResponse {
    HttpResponse::Ok().json(config.get_ref())
}

#[derive(serde::Deserialize)]
struct CompileRequest {
    project: Option<DisbotterProjectData>,
}

async fn compile(mut req: web::Json<CompileRequest>, config: web::Data<DisbotterRESTApiConfig>) -> HttpResponse {
    let project = match req.project.take() {
        Some(project) => project,
        None => {
            return HttpResponse::BadRequest().body("Missing project data");
        }
    };
    let mut compiler = NodesJSCompiler::new(project);
    compiler.add_available_nodes_from_vec(&config.nodes);
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

#[derive(serde::Deserialize)]
struct RunRequest {
    project: Option<DisbotterProjectData>,
    token: Option<String>,
    client_id: Option<String>,
    guild_id: Option<String>,
}

async fn compile_and_run(mut req: web::Json<RunRequest>, config: web::Data<DisbotterRESTApiConfig>, state: web::Data<DisbotterRESTApiState>) -> HttpResponse {
    if !config.can_run {
        return HttpResponse::Forbidden().body("Running is disabled on the server");
    }

    // First ensure that there is a project
    let project = match req.project.take() {
        Some(project) => project,
        None => {
            return HttpResponse::BadRequest().body("Missing project data");
        }
    };

    // Then if there is no folder related to the project, create one
    let project_folder = config.projects.as_ref().unwrap().join(project.metadata.name.replace(" ", "_").clone());
    if !project_folder.exists() {
        std::fs::create_dir_all(&project_folder).unwrap();
    }

    // Install dependencies
    let cmds = if cfg!(windows) {
        "pnpm.cmd"
    } else {
        "pnpm"
    };

    // If there is already a folder, skip cloning the example project
    if !project_folder.join("src").exists() {

        // Then clone example project to the folder
        let url = "https://github.com/olix3001/disbotter-example-project";
        match git2::Repository::clone(url, &project_folder) {
            Ok(_) => {},
            Err(err) => {
                println!("{:?}", err);
                return HttpResponse::InternalServerError().body("Failed to clone example project");
            }
        };

        let mut cmd = std::process::Command::new(cmds);

        cmd.arg("install").current_dir(&project_folder);
        let output = cmd.output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    return HttpResponse::InternalServerError().body("Failed to install dependencies");
                }
            },
            Err(err) => {
                return HttpResponse::InternalServerError().body(format!("Failed to install dependencies: {:?}", err));
            }
        } 
    }

    // Remove .env file
    std::fs::remove_file(project_folder.join(".env")).unwrap_or(());

    // Write token to .env
    let mut env_file = std::fs::File::create(project_folder.join(".env")).unwrap();
    env_file.write_all(format!("BOT_TOKEN={}\n", req.token.as_ref().unwrap_or(&config.def_token)).as_bytes()).unwrap();
    env_file.write_all(format!("CLIENT_ID={}\n", req.client_id.as_ref().unwrap_or(&config.def_client_id)).as_bytes()).unwrap();

    // Replace guild id in index.ts
    let index_file_content = std::fs::read_to_string(project_folder.join("src/index.ts")).unwrap();
    let re = regex::Regex::new(r"devGuilds: \[.*\]").unwrap();
    let index_file_content = re.replace_all(&index_file_content, &format!("devGuilds: [\"{}\"]", req.guild_id.as_ref().unwrap_or(&config.def_guild_id)));

    std::fs::write(project_folder.join("src/index.ts"), index_file_content.as_bytes()).unwrap();

    // Compile project
    let mut compiler = NodesJSCompiler::new(project);
    compiler.add_available_nodes_from_vec(&config.nodes);
    let project = compiler.compile_project();

    let project = match project {
        Ok(project) => project,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Failed to compile project: {:?}", err));
        }
    };

    // Write project to files
    project.export_to_path(project_folder.join("src"));

    // Start the bot in the project folder
    let mut cmd = std::process::Command::new(cmds);
    cmd
        .arg("run")
        .arg("build")
        .stdout(Stdio::null())
        .current_dir(&project_folder)
        .spawn().unwrap().wait().unwrap();

    let mut cmd = std::process::Command::new("node");
    cmd
        .arg("dist/index.js")
        .stdout(Stdio::null())
        .current_dir(&project_folder);

    let child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Failed to start bot: {:?}", err));
        }
    };

    // Add the child to the state
    let old_running = state.currently_running.lock().unwrap().replace(child);

    if let Some(mut old_running) = old_running {
        match kill_force(&mut old_running) {
            Ok(_) => {
                return HttpResponse::Ok().body("Code successfully compiled and bot started (old bot killed)");
            },
            Err(err) => {
                println!("Failed to kill old bot: {:?}", err);
            }
        }
    }

    HttpResponse::Ok().body("Code successfully compiled and bot started")
}

async fn kill_current(state: web::Data<DisbotterRESTApiState>) -> HttpResponse {
    let mut currently_running = state.currently_running.lock().unwrap();

    if let Some(mut currently_running) = currently_running.take() {
        match kill_force(&mut currently_running) {
            Ok(_) => {
                HttpResponse::Ok().body("Bot successfully killed")
            },
            Err(err) => {
                HttpResponse::InternalServerError().body(format!("Failed to kill bot: {:?}", err))
            }
        }
    } else {
        HttpResponse::Ok().body("No bot was running")
    }
}

// bit of a hack, but it works, so don't touch it
fn kill_force(child: &mut Child) -> Result<(), std::io::Error> {
    if cfg!(windows) {
        let mut cmd = std::process::Command::new("taskkill");
        cmd.arg("/pid").arg(child.id().to_string()).arg("/f");
        cmd.spawn()?.wait()?;
    } else {
        child.kill()?;
    }

    Ok(())
}

// TODO: Add websocket endpoint for logs and heartbeats

impl DisbotterRESTApi {
    pub fn new() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
        }
    }

    pub fn start(&self, nodes: Arc<Vec<AvailableNode>>, projects: Option<PathBuf>) -> std::io::Result<()> {
        #[cfg(debug_assertions)]
        {
            println!("Starting server in debug mode...");
            std::env::set_var("RUST_LOG", "actix_web=debug");
            env_logger::try_init().ok();
        }

        self.rt.block_on(async {
            println!("Starting server...");

            let state = web::Data::new(DisbotterRESTApiState::default());
            
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
                    .app_data(web::Data::new(DisbotterRESTApiConfig {
                        nodes,
                        projects: projects.clone(),
                        can_run: projects.is_some(),
                        def_token: std::env::var("DISCORD_TOKEN").unwrap_or("".to_string()),
                        def_client_id: std::env::var("DISCORD_CLIENT_ID").unwrap_or("".to_string()),
                        def_guild_id: std::env::var("DISCORD_GUILD_ID").unwrap_or("".to_string()),
                    }))
                    .app_data(web::Data::clone(&state))
                    .route("/ping", web::get().to(ping))
                    .route("/config", web::get().to(config))
                    .route("/compile", web::post().to(compile))
                    .route("/cnr", web::post().to(compile_and_run))
                    .route("/kill", web::post().to(kill_current))
            })
            .bind(("127.0.0.1", 3000))?
            .run()
            .await
        })
    }
}
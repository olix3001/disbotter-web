use compiler::{NodesJSCompiler, AvailableNode, upgrade_engine};
use loader::{load_all_nodes, export_node_declarations, Node};
use clap::{Parser, Subcommand, arg};
use colored::*;
use rhai::Engine;

// ===< Module imports >=== //
pub mod loader;
pub mod builder;
pub mod compiler;
mod server;

// ===< Main CLI >=== //
#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    #[command(name="gen-node-declarations", about="Generates node declarations for the web editor")]
    GenNodeDeclarations {
        #[arg(short, long, help="Path to the directory/directories containing the nodes")]
        path: String,
        #[arg(short, long, help="Path to the output file")]
        output: String
    },
    #[command(name="compile", about="Compile a project from a .dbp file")]
    Compile {
        #[arg(short, long, help="Path to the .dbp file")]
        path: String,
        #[arg(short, long, help="Path to the output directory")]
        output: String,
        #[arg(short, long, help="Path to the directory/directories containing the nodes")]
        nodes: String
    },
    #[command(name="init", about="Initialize a new project", aliases=&["new", "create"])]
    Init {
        #[arg(help="Path to the output directory", index = 1)]
        path: String,
        #[arg(long, help="use pnpm instead of npm")]
        pnpm: bool
    },
    #[command(name="server", about="Start the REST API server")]
    Server {
        #[arg(short, long, help="Path to the directory/directories containing the nodes")]
        nodes: String
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::GenNodeDeclarations { path, output }) => {
            // If command is to generate node declarations
            let paths = path.split(",");
            let nodes: Vec<Node> = paths.flat_map(|path| load_all_nodes(path.into()).unwrap_or_else(|err| {
                println!("{} {}", "Failed to load nodes from path:".red(), path.to_string().yellow());
                println!("{:?}", err);
                vec![]
            })).collect();
            export_node_declarations(nodes, output.clone().into());
            println!("{} {}", "Successfully generated node declarations:".green(), output.yellow());
        },
        Some(Commands::Compile { path, output, nodes }) => {
            // If command is to compile a project
            let project = NodesJSCompiler::load_project(path.clone().into());
            let mut compiler = NodesJSCompiler::new(project);
            let paths = nodes.split(",");
            for path in paths.into_iter() {
                compiler.add_available_nodes(path.clone().into());
            }
            let project = compiler.compile_project();

            match project {
                Ok(project) => {
                    project.export_to_path(output.into());
                    println!("{} {}", "Successfully compiled project:".green(), path.yellow());
                },
                Err(err) => {
                    println!("{} {}", "Failed to compile project:".red(), path.yellow());
                    println!("{}", err.to_pretty());
                }
            }
        },
        Some(Commands::Init { path, pnpm }) => {
            // If command is to initialize a new project
            let url = "https://github.com/olix3001/disbotter-example-project";
            println!("{} {}", "Cloning repository:".green(), url.yellow());
            match git2::Repository::clone(url, path.clone()) {
                Ok(_) => {},
                Err(err) => {
                    println!("{} {}", "Failed to clone repository:".red(), url.yellow());
                    println!("{:?}", err);
                    return;
                }
            };

            // Install dependencies
            println!("{}", "Installing dependencies...".green());
            let mut cmd = if pnpm {
                #[cfg(windows)]
                let prog = "pnpm.cmd";
                #[cfg(not(windows))]
                let prog = "pnpm";

                std::process::Command::new(prog)
            } else {
                #[cfg(windows)]
                let prog = "npm.cmd";
                #[cfg(not(windows))]
                let prog = "npm";

                std::process::Command::new(prog)
            };

            cmd.arg("install").current_dir(path.clone());
            let output = cmd.output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("{}", "Successfully installed dependencies:".green());
                    } else {
                        println!("{}", "Failed to install dependencies:".red());
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                        return;
                    }
                },
                Err(err) => {
                    println!("{}", "Failed to install dependencies:".red());
                    println!("{:?}", err);
                    return;
                }
            } 

            // Show success message
            println!("{} {}", "Successfully initialized project in:".green(), path.yellow());

            println!("{} {}", "To compile the project, run:".green(), format!("disbotter compile ... -o {}", std::path::PathBuf::from(path).join("src").to_str().unwrap()).yellow());
            println!("{} {}", "To run the project, go to the project directory and run:".green(), "npm/pnpm start".yellow());
        },
        Some(Commands::Server { nodes }) => {
            // If command is to start the server
            // let paths = nodes.split(",");
            // let nodes: Vec<Node> = paths.flat_map(|path| load_all_nodes(path.into()).unwrap_or_else(|err| {
            //     println!("{} {}", "Failed to load nodes from path:".red(), path.to_string().yellow());
            //     println!("{:?}", err);
            //     vec![]
            // })).collect();
            
            let api = server::DisbotterRESTApi::new();
            let mut engine = Engine::new();
            upgrade_engine(&mut engine);
            let nodes = AvailableNode::load_nodes(nodes.into(), &mut engine);
            api.start(std::sync::Arc::new(nodes)).expect("Failed to start server");
        },
        None => {
            println!("No command specified!");
        }
    }
}
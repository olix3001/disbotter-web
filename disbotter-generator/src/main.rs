use compiler::NodesJSCompiler;
use loader::{load_all_nodes, export_node_declarations, Node};
use clap::{Parser, Subcommand, arg};
use colored::*;

// ===< Module imports >=== //
pub mod loader;
pub mod builder;
pub mod compiler;

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
        None => {
            println!("No command specified!");
        }
    }
}
use compiler::NodesJSCompiler;
use loader::{load_all_nodes, export_node_declarations};

pub mod loader;
pub mod builder;
pub mod compiler;

fn generate_node_declarations() {
    let common_nodes = load_all_nodes("../data/nodes/common".into());
    let command_nodes = load_all_nodes("../data/nodes/command".into());
    let nodes = common_nodes.into_iter().chain(command_nodes.into_iter()).collect();
    export_node_declarations(nodes, "../static/generated/command_node_declarations.json".into());
    println!("Node declarations generated!");
}

fn test_output() {
    let mut compiler = NodesJSCompiler::new(NodesJSCompiler::load_project("C:\\Users\\Oliwier\\Downloads\\yooo.dbp".into()));
    compiler.add_available_nodes("../data/nodes/common".into());
    compiler.add_available_nodes("../data/nodes/command".into());
    let project = compiler.compile_project();
    project.export_to_path("../data/target".into());
}

fn main() {
    generate_node_declarations();
    test_output();
}

use loader::{load_all_nodes, export_node_declarations};

pub mod loader;
pub mod builder;

fn main() {
    let nodes = load_all_nodes("../data/nodes".into());
    export_node_declarations(nodes, "../data/generated/node_declarations.json".into());
}

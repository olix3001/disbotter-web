use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}};

use rhai::{Engine, Scope};

use crate::builder::{CodeBuilder, Program};

#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum PortIdentifier {
    Input {
        node_uid: String,
        port_key: String
    },
    Output {
        node_uid: String,
        port_key: String
    },
    Global {
        key: String
    }
}

impl Clone for NodesJSCompiler {
    fn clone(&self) -> Self {
        self.get_cloned_compiler()
    }
}

pub struct NodesJSCompiler {
    available_nodes: HashMap<String, AvailableNode>,
    project: Arc<DisbotterProjectData>,
    pub var_cache: Arc<Mutex<HashMap<PortIdentifier, String>>>,
    engine: Engine,
    program: Program,
    pub current_flow: Option<DisbotterFlow>,
}

impl NodesJSCompiler {
    pub fn new(project: DisbotterProjectData) -> NodesJSCompiler {
        let mut engine = Engine::new();
        engine.build_type::<CodeBuilder>();

        NodesJSCompiler {
            available_nodes: HashMap::new(),
            project: Arc::new(project),
            var_cache: Arc::new(Mutex::new(HashMap::new())),
            engine,
            program: Program::new(),
            current_flow: None,
        }
    }

    pub fn get_cloned_compiler(&self) -> NodesJSCompiler {
        let mut engine = Engine::new();
        engine.build_type::<CodeBuilder>();

        NodesJSCompiler {
            available_nodes: self.available_nodes.clone(),
            project: Arc::clone(&self.project),
            var_cache: Arc::clone(&self.var_cache),
            engine,
            program: Program::new(),
            current_flow: self.current_flow.clone(),
        }
    }

    pub(crate) fn random_var_name() -> String {
        let id = uuid::Uuid::new_v4();
        format!("__io_{}", id.to_string().replace("-", "_"))
    }

    pub fn load_project(path: PathBuf) -> DisbotterProjectData {
        let file = std::fs::read_to_string(path).unwrap();
        let project: DisbotterProjectData = serde_json::from_str(&file).unwrap();
        project
    }

    pub fn add_available_nodes(&mut self, path: PathBuf) {
        let nodes = AvailableNode::load_nodes(path, &mut self.engine);
        for node in nodes {
            self.available_nodes.insert(node.id.clone(), node);
        }
    }

    pub fn compile_project(mut self) -> Program {
        for command in self.project.clone().content.commands.iter() {
            self.compile_command(&command);
        }

        self.program
    }

    pub fn add_var(&mut self, var_key: String, var_name: String) {
        self.var_cache.lock().unwrap().insert(PortIdentifier::Global { key: var_key }, var_name);
    }

    pub fn clear_var_cache(&mut self) {
        self.var_cache.lock().unwrap().clear();
    }

    pub fn compile_command(&mut self, command: &DisbotterProjectCommand) {
        let builder = self.program.get_file_builder(format!("commands/{}.js", command.name), &self.var_cache, self.get_cloned_compiler());
        self.compile_flow(&command.flow, builder.clone(), "_builtin_on_command");
        builder.finalize();
    }

    pub fn get_flow_target(&self, flow: &DisbotterFlow, port: PortIdentifier) -> Option<PortIdentifier> {
        match port {
            PortIdentifier::Input { node_uid, port_key } => {
                let conn = flow.connections.iter()
                    .find(|c| c.to == node_uid && c.to_key == port_key);
                if let Some(conn) = conn {
                    Some(PortIdentifier::Output { node_uid: conn.from.clone(), port_key: conn.from_key.clone() })
                } else {
                    None
                }
            },
            PortIdentifier::Output { node_uid, port_key } => {
                let conn = flow.connections.iter()
                    .find(|c| c.from == node_uid && c.from_key == port_key);
                if let Some(conn) = conn {
                    Some(PortIdentifier::Input { node_uid: conn.to.clone(), port_key: conn.to_key.clone() })
                } else {
                    None
                }
            },
            _ => None
        }
    }

    /// Map the outputs of other nodes to the inputs of the current node
    pub fn map_node_inputs(&mut self, flow: &DisbotterFlow, node: &DisbotterFlowNode, builder: CodeBuilder) {
        // Iterate over all inputs
        let node_connections = flow.connections.iter()
            .filter(|c| c.to == node.uid)
            .collect::<Vec<&DisbotterFlowConnection>>();

        let var_cache_c = self.var_cache.clone();
        let mut var_cache = var_cache_c.lock().unwrap();
        for conn in node_connections.iter() {
            // Check for existing data
            let existing_data = var_cache
                .get(&PortIdentifier::Output { node_uid: conn.from.clone(), port_key: conn.from_key.clone() });

            if let Some(existing_data) = existing_data {
                let existing_data = existing_data.clone();
                var_cache.insert(PortIdentifier::Input { node_uid: conn.to.clone(), port_key: conn.to_key.clone() }, existing_data);
            } else {
                // Check if the node here is connected to a pure function
                let from_node = flow.nodes.iter()
                    .find(|n| n.uid == conn.from)
                    .unwrap();

                let from_node_template = self.available_nodes.get(&from_node.node_type).unwrap();

                if from_node_template.is_pure {
                    // Compile the node
                    drop(var_cache); // Drop the lock to prevent deadlocks
                    self.compile_node(from_node, flow, builder.clone());
                    var_cache = var_cache_c.lock().unwrap();

                    // Bind the output to the input
                    let new_data = var_cache
                        .get(&PortIdentifier::Output { node_uid: conn.from.clone(), port_key: conn.from_key.clone() })
                        .unwrap()
                        .clone();
                    var_cache.insert(PortIdentifier::Input { node_uid: conn.to.clone(), port_key: conn.to_key.clone() }, new_data);
                }
            }
        }

        // Map hardcoded inputs
        for (key, value) in node.input_hardcoded.iter() {
            // If input is not mapped already
            if !var_cache.contains_key(&PortIdentifier::Input { node_uid: node.uid.clone(), port_key: key.clone() }) {
                // Map
                var_cache.insert(PortIdentifier::Input { node_uid: node.uid.clone(), port_key: key.clone() }, value.to_string());
            }
        }
    }

    pub fn compile_flow_from_port(&mut self, flow: &DisbotterFlow, builder: CodeBuilder, port: PortIdentifier, node: &DisbotterFlowNode) {
        let oport_key = match port {
            PortIdentifier::Output { node_uid: _, port_key } => port_key,
            _ => panic!("Invalid port connection")
        };

        let mut current_flow_out: Option<PortIdentifier> = 
            Some(node.get_port_out(&oport_key));

        // Compile node that is connected to the start node
        while let Some(flow_out) = current_flow_out {
            let target = self.get_flow_target(flow, flow_out.clone());
            if target.is_none() {
                break;
            }
            if let PortIdentifier::Input { node_uid, port_key } = target.unwrap() {
                if port_key != "__flow_in__" {
                    panic!("Invalid flow connection");
                }
                let node = flow.nodes.iter().find(|n| n.uid == node_uid).unwrap();


                self.compile_node(node, flow, builder.clone());
                current_flow_out = Some(node.get_port_out("__flow_out__"));
            } else {
                break;
            }
        }
    }

    pub fn compile_flow(&mut self, flow: &DisbotterFlow, mut builder: CodeBuilder, start_node_id: &str) {
        builder.compiler.current_flow = Some(flow.clone());
        // Find the start node
        let start_node = flow.nodes.iter().find(|n| n.node_type == start_node_id).unwrap();
        // Clear the var cache
        self.clear_var_cache();
        // Add interaction key
        self.add_var("interaction".to_string(), "___INTERACTION___".to_string());

        // Compile the start node
        self.compile_node(start_node, flow, builder.clone());

        // Compile the rest of the flow
        self.compile_flow_from_port(flow, builder.clone(), start_node.get_port_out("__flow_out__"), start_node);
    }

    pub fn compile_node(&mut self, node: &DisbotterFlowNode, flow: &DisbotterFlow, mut builder: CodeBuilder) {
        self.map_node_inputs(flow, node, builder.clone());
        let node_type = &node.node_type;
        let node_id = &node.uid;
        let node = self.available_nodes.get(node_type).unwrap();
        builder.current_node_id = node_id.clone();
        node.call_action(&mut self.engine, builder.clone());
        println!("Compiled node {} ({})", node_type, node_id);
    }
}

#[derive(Clone)]
pub struct AvailableNode {
    pub id: String,
    ast: Arc<rhai::AST>,
    pub is_pure: bool
}

impl AvailableNode {
    pub fn call_action(&self, engine: &mut Engine, builder: CodeBuilder) {
        let mut scope = Scope::new();
        engine.call_fn::<()>(&mut scope, &self.ast, "action", (builder,)).unwrap();
    }

    pub fn load_nodes(path: PathBuf, engine: &mut Engine) -> Vec<AvailableNode> {
        let mut nodes = Vec::new();
        let files = std::fs::read_dir(path).unwrap();
        for file in files {
            let file = file.unwrap();
            let file_path = file.path();
            if file_path.is_file() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".rhai") {
                    let file = std::fs::read_to_string(file_path).unwrap();
                    let ast = engine.compile(&file).unwrap();
                    let mut constants = ast.iter_literal_variables(true, false);
                    let id = constants.find(|c| c.0 == "id").expect("All nodes should have an id").2.clone_cast::<String>();
                    let is_pure = constants.find(|c| c.0 == "pure").is_some();
                    drop(constants); // Drop the iterator so we can use the AST
                    nodes.push(AvailableNode {
                        id,
                        ast: Arc::new(ast),
                        is_pure
                    });
                }
            }
        }
        nodes
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisbotterProjectData {
    metadata: DisbotterProjectMetadata,
    content: DisbotterProjectContent
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisbotterProjectMetadata {
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisbotterProjectContent {
    commands: Vec<DisbotterProjectCommand>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisbotterProjectCommand {
    uid: String,
    name: String,
    description: String,
    flow: DisbotterFlow
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DisbotterFlow {
    nodes: Vec<DisbotterFlowNode>,
    connections: Vec<DisbotterFlowConnection>
}

impl DisbotterFlow {
    pub fn get_node(&self, uid: &str) -> &DisbotterFlowNode {
        self.nodes.iter().find(|n| n.uid == uid).unwrap()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DisbotterFlowNode {
    uid: String,
    #[serde(rename = "type")]
    node_type: String,
    #[serde(rename = "inputHardcoded")]
    input_hardcoded: HashMap<String, serde_json::Value>
}

impl DisbotterFlowNode {
    pub fn get_port_in(&self, key: &str) -> PortIdentifier {
        PortIdentifier::Input {
            node_uid: self.uid.clone(),
            port_key: key.to_string()
        }
    }

    pub fn get_port_out(&self, key: &str) -> PortIdentifier {
        PortIdentifier::Output {
            node_uid: self.uid.clone(),
            port_key: key.to_string()
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DisbotterFlowConnection {
    #[serde(rename = "type")]
    connection_type: i32,
    from: String,
    to: String,
    #[serde(rename = "fromKey")]
    from_key: String,
    #[serde(rename = "toKey")]
    to_key: String
}
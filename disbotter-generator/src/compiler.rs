use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}};

use rhai::{Engine, Scope, EvalContext, Expression, EvalAltResult, Dynamic};

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
        upgrade_engine(&mut engine);

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
        upgrade_engine(&mut engine);

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
        let id = id.to_string();
        let id = id.split("-");
        // First two parts of the uuid combined should be enough to be unique
        let id = format!("{}{}", id.clone().next().unwrap(), id.clone().next().unwrap());
        format!("{}", id)
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
        let mut builder = self.program.get_file_builder(format!("commands/{}.ts", command.name), &self.var_cache, self.get_cloned_compiler());
        builder.add_lines(vec![
            "export default class extends Command {".to_string(),
            "   public readonly builder = new SlashCommandBuilder()".to_string(),
            format!("       .setName(\"{}\")", command.name),
            format!("       .setDescription(\"{}\");", command.description),
            "".to_string(),
            "   public async handle(__TRANSLATIONS__: LocalizedTranslations, __INTERACTION__: CommandInteraction): Promise<void> {".to_string(),
        ]);
        builder.increase_ident_by(2);

        self.compile_flow(&command.flow, builder.clone(), "__start__");

        builder.decrease_ident_by(2);
        builder.add_lines(vec![
            "   }".to_string(),
            "}".to_string(),
        ]);
        builder.add_import("CommandInteraction, SlashCommandBuilder".to_string(), "discord.js".to_string());
        builder.add_import("Command, LocalizedTranslations".to_string(), "disbotter".to_string());
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
                var_cache.insert(PortIdentifier::Input { node_uid: node.uid.clone(), port_key: key.clone() }, get_raw_value(value));
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
        let start_node = flow.nodes.iter().find(|n| n.node_type == start_node_id).expect(
            "Every flow needs a start node"
        );
        // Clear the var cache
        self.clear_var_cache();
        // Add interaction key
        self.add_var("___interaction".to_string(), "__INTERACTION__".to_string());
        self.add_var("___translations".to_string(), "__TRANSLATIONS__".to_string());

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

fn expr_shortcut_add_line(context: &mut EvalContext, inputs: &[Expression]) -> Result<Dynamic, Box<EvalAltResult>> {
    // Replace -> $expr$ with builder.add_line($expr$)
    let expr = &inputs[0];
    let expr = context.eval_expression_tree(expr)?;
    let mut builder = context.scope().get("builder").unwrap().clone().try_cast::<CodeBuilder>().unwrap();

    builder.add_line(expr.into_string().unwrap());

    Ok(Dynamic::from(()))
}

fn expr_shortcut_set_output(context: &mut EvalContext, inputs: &[Expression]) -> Result<Dynamic, Box<EvalAltResult>> {
    // Replace $ident$ <- $expr$ with builder.set_output($ident$, $expr$)
    let ident = inputs[0].get_string_value().unwrap();
    let expr = &inputs[1];
    let expr = context.eval_expression_tree(expr)?;
    let mut builder = context.scope().get("builder").unwrap().clone().try_cast::<CodeBuilder>().unwrap();

    builder.set_output(ident.to_string(), expr.into_string().unwrap());

    Ok(Dynamic::from(()))
}

fn get_raw_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{}\"", s),
        serde_json::Value::Number(n) => n.as_f64().unwrap().to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => "".to_string()
    }
}

pub fn upgrade_engine(engine: &mut Engine) {
    // CodeBuilder Type
    engine.build_type::<CodeBuilder>();

    // -> $expr$ syntax as a shorthand for builder.add_line($expr$)
    engine.register_custom_syntax(
        ["->", "$expr$"],
        false,
        expr_shortcut_add_line
    ).ok();

    // out name = $expr$ syntax as a shorthand for builder.set_output($expr$)
    engine.register_custom_syntax(
        ["out", "$ident$", "=", "$expr$"],
        false,
        expr_shortcut_set_output
    ).ok();

    // inv name as a shorthand for builder.get_input(name)
    engine.register_custom_syntax(
        ["inv", "$ident$"],
        false,
        |context: &mut EvalContext, inputs: &[Expression]| -> Result<Dynamic, Box<EvalAltResult>> {
            let ident = inputs[0].get_string_value().unwrap();
            let mut builder = context.scope().get("builder").unwrap().clone().try_cast::<CodeBuilder>().unwrap();

            Ok(Dynamic::from(builder.get_in_var(ident.to_string())))
        }
    ).ok();
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
            } else if file_path.is_dir() {
                nodes.append(&mut Self::load_nodes(file_path, engine));
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
    pub(crate) node_type: String,
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
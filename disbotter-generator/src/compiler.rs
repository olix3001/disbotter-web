use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}, fmt::{Display, Debug}};
use rhai::{Engine, Scope, EvalContext, Expression, EvalAltResult, Dynamic};
use colored::*;

use crate::builder::{CodeBuilder, Program};

/// Represents a port on a node, it consists of a node uid and a port key
/// There are also global ports, which are just a key
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
    CompTime {
        node_uid: String,
        port_key: String,
        data_key: String
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

/// Compiler for the nodes
pub struct NodesJSCompiler {
    available_nodes: HashMap<String, AvailableNode>,
    project: Arc<DisbotterProjectData>,
    pub var_cache: Arc<Mutex<HashMap<PortIdentifier, String>>>,
    engine: Engine,
    program: Program,
    pub current_flow: Option<DisbotterFlow>,
    current_command: Option<DisbotterProjectCommand>,
}

/// all possible errors that can occur during compilation
pub enum CompilerError {
    RhaiError(Box<EvalAltResult>),
    InvalidPortIdentifier(PortIdentifier),
    NodeNotFound(String),
    BadContext(String),
    NoStartNode,
}

impl CompilerError {
    pub fn to_pretty(&self) -> String {
        match self {
            CompilerError::RhaiError(err) => {
                format!("{}: {}", "Rhai error".red(), err)
            },
            CompilerError::InvalidPortIdentifier(port) => {
                format!("{}: {}", "Invalid port identifier".red(), port)
            },
            CompilerError::NodeNotFound(node_id) => {
                format!("{}: {}", "Node not found".red(), node_id)
            }
            CompilerError::NoStartNode => {
                format!("{}: {}", "No start node".red(), "No start node found")
            }
            CompilerError::BadContext(context) => {
                format!("{}: {}", "Bad context".red(), context)
            }
        }
    }
}

impl Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

impl Display for PortIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortIdentifier::Input { node_uid, port_key } => {
                write!(f, "Input: {} -> {}", node_uid, port_key)
            },
            PortIdentifier::Output { node_uid, port_key } => {
                write!(f, "Output: {} -> {}", node_uid, port_key)
            },
            PortIdentifier::Global { key } => {
                write!(f, "Global: {}", key)
            }
            PortIdentifier::CompTime { node_uid, port_key, data_key } => {
                write!(f, "CompTime: {} -> {} ({})", node_uid, port_key, data_key)
            }
        }
    }
}

impl NodesJSCompiler {
    /// Create a new compiler
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
            current_command: None,
        }
    }

    /// Clones the compiler, used for nodes like the if node
    /// that need to compile their own code
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
            current_command: self.current_command.clone(),
        }
    }

    /// Generates a random variable name.
    /// Variable names are just two first parts of a v4 uuid combined.
    /// It is useful to prefix variable with for example underscore to ensure
    /// they don't start with a number
    pub(crate) fn random_var_name() -> String {
        let id = uuid::Uuid::new_v4();
        let id = id.to_string();
        let id = id.split("-");
        // First two parts of the uuid combined should be enough to be unique
        let id = format!("{}{}", id.clone().next().unwrap(), id.clone().next().unwrap());
        format!("{}", id)
    }

    /// Loads a project from a path
    pub fn load_project(path: PathBuf) -> DisbotterProjectData {
        let file = std::fs::read_to_string(path).unwrap();
        let project: DisbotterProjectData = serde_json::from_str(&file).unwrap();
        project
    }

    /// Adds available nodes from a path
    pub fn add_available_nodes(&mut self, path: PathBuf) {
        let nodes = AvailableNode::load_nodes(path, &mut self.engine);
        for node in nodes {
            self.available_nodes.insert(node.id.clone(), node);
        }
    }

    /// Compiles the project.
    /// This means it will compile all commands, events, etc.
    pub fn compile_project(mut self) -> Result<Program, CompilerError> {
        for command in self.project.clone().content.commands.iter() {
            self.compile_command(&command)?;
        }

        Ok(self.program)
    }

    /// Adds a global variable to the var cache
    pub fn add_var(&mut self, var_key: String, var_name: String) {
        self.var_cache.lock().unwrap().insert(PortIdentifier::Global { key: var_key }, var_name);
    }

    /// Clears all variables from the var cache
    pub fn clear_var_cache(&mut self) {
        self.var_cache.lock().unwrap().clear();
    }

    /// Compiles specified command
    pub fn compile_command(&mut self, command: &DisbotterProjectCommand) -> Result<(), CompilerError> {
        // Get builder
        let mut builder = self.program.get_file_builder(format!("commands/{}.ts", command.name), &self.var_cache, self.get_cloned_compiler());

        self.current_command = Some(command.clone());
        
        // Boilerplate
        builder.add_lines(vec![
            "export default class extends Command {".to_string(),
            "   public readonly builder = new SlashCommandBuilder()".to_string(),
            format!("       .setName(\"{}\")", command.name),
            format!("       .setDescription(\"{}\")", command.description),
        ]);

        // Add options
        for option in command.options.iter() {
            let cmd_option = match option.option_type {
                0 => {
                    "addStringOption"
                },
                1 => {
                    "addUserOption"
                },
                2 => {
                    "addChannelOption"
                },
                _ => {
                    return Err(CompilerError::BadContext(format!("Unknown option type: {}", option.option_type)));
                }
            };

            builder.add_line(
                format!("\t\t.{}(option => option\n\t\t\t.setName(\"{}\")\n\t\t\t.setDescription(\"{}\")\n\t\t\t.setRequired({})\n\t\t)", cmd_option, option.name, option.description, option.required),
            );
        }

        // More boilerplate
        builder.add_lines(vec![
            "".to_string(),
            "   public async handle(__TRANSLATIONS__: LocalizedTranslations, __INTERACTION__: CommandInteraction): Promise<void> {".to_string(),
        ]);
        builder.increase_ident_by(2);

        // Compile flow
        self.compile_flow(&command.flow, builder.clone(), "__start__")?;

        // More boilerplate
        builder.decrease_ident_by(2);
        builder.add_lines(vec![
            "   }".to_string(),
            "}".to_string(),
        ]);
        builder.add_import("CommandInteraction, SlashCommandBuilder".to_string(), "discord.js".to_string());
        builder.add_import("Command, LocalizedTranslations".to_string(), "disbotter".to_string());
        
        builder.add_on_top("// @ts-nocheck".to_string());
        builder.add_on_top("// This file is automatically generated by Disbotter".to_string());
        builder.finalize();

        Ok(())
    }

    /// Get whatever port is connected to
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

    /// Gets global variable
    pub fn get_global_var(&self, key: String) -> Option<String> {
        self.var_cache.lock().unwrap().get(&PortIdentifier::Global { key })
            .map(|v| v.clone())
    }

    /// Some nodes need to be compiled differently, for example option nodes, these nodes are compiled here
    /// and return true if they were compiled
    /// If this function returns false, the node is not special and should be compiled normally
    pub fn compile_special_node(&mut self, mut builder: CodeBuilder, node: &DisbotterFlowNode) -> Result<bool, CompilerError> {
        // Check if node is special
        if node.node_type.starts_with("___special_") {
            // Option node
            if node.node_type.starts_with("___special_get_option_") {
                let option_name = node.node_type.replace("___special_get_option_", "");
                let option_name = option_name.replace("___", "");

                // Get interaction
                let interaction_name = self.get_global_var("___interaction".to_string());

                if interaction_name.is_none() {
                    return Err(CompilerError::BadContext("Cannot get option outside of interaction".to_string()));
                }

                // Get option type
                let opt_type = match self.current_command.as_ref().unwrap().options.iter().find(|o| o.name == option_name) {
                    Some(opt) => {
                        match opt.option_type {
                            0 => "String",
                            1 => "User",
                            2 => "Channel",
                            _ => {
                                return Err(CompilerError::BadContext(format!("Unknown option type: {}", opt.option_type)));
                            }
                        }
                    },
                    None => {
                        return Err(CompilerError::BadContext(format!("Cannot get option \"{}\" because it does not exist", option_name)));
                    }
                };

                // Create variable for option
                let option_var_name = format!("__get_option_{}", option_name);
                builder.add_line(format!("let {} = {}.options.get{}(\"{}\");", option_var_name, interaction_name.unwrap(), opt_type, option_name));

                builder.var_cache.lock().unwrap().insert(
                    PortIdentifier::Output { node_uid: node.uid.clone(), port_key: "value".to_string() },
                    option_var_name.clone()
                );

                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Map the outputs of other nodes to the inputs of the current node
    pub fn map_node_inputs(&mut self, flow: &DisbotterFlow, node: &DisbotterFlowNode, builder: CodeBuilder) -> Result<(), CompilerError> {
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

                let from_node_template = self.available_nodes.get(&from_node.node_type);
                if from_node_template.is_none() {
                    drop(var_cache);
                    if self.compile_special_node(builder.clone(), from_node)? {
                        var_cache = var_cache_c.lock().unwrap();
                        // Bind the output to the input
                        let new_data = var_cache
                            .get(&PortIdentifier::Output { node_uid: conn.from.clone(), port_key: conn.from_key.clone() })
                            .unwrap()
                            .clone();
                        var_cache.insert(PortIdentifier::Input { node_uid: conn.to.clone(), port_key: conn.to_key.clone() }, new_data);
                        continue;
                    }
                    return Err(CompilerError::NodeNotFound(from_node.node_type.clone()));
                }
                let from_node_template = from_node_template.unwrap();

                if from_node_template.is_pure {
                    // Compile the node
                    drop(var_cache); // Drop the lock to prevent deadlocks
                    self.compile_node(from_node, flow, builder.clone())?;
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

        Ok(())
    }

    /// Compiles flow starting from specified node
    pub fn compile_flow_from_port(&mut self, flow: &DisbotterFlow, builder: CodeBuilder, port: PortIdentifier, node: &DisbotterFlowNode)
        -> Result<(), CompilerError> {
        let oport_key = match &port {
            PortIdentifier::Output { node_uid: _, port_key } => Ok(port_key),
            _ => Err(CompilerError::InvalidPortIdentifier(port.clone()))
        }?;

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
                    return Err(CompilerError::InvalidPortIdentifier(port.clone()));
                }
                let node = flow.nodes.iter().find(|n| n.uid == node_uid).unwrap();


                self.compile_node(node, flow, builder.clone())?;
                current_flow_out = Some(node.get_port_out("__flow_out__"));
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Find node that has id equal to specified id
    pub fn compile_flow(&mut self, flow: &DisbotterFlow, mut builder: CodeBuilder, start_node_id: &str) -> Result<(), CompilerError> {
        builder.compiler.current_flow = Some(flow.clone());
        // Find the start node
        let start_node = flow.nodes.iter().find(|n| n.node_type == start_node_id);
        if start_node.is_none() {
            return Err(CompilerError::NoStartNode);
        }
        let start_node = start_node.unwrap();
        // Clear the var cache
        self.clear_var_cache();
        // Add interaction key
        self.add_var("___interaction".to_string(), "__INTERACTION__".to_string());
        self.add_var("___guild".to_string() , "__INTERACTION__.guild".to_string());
        self.add_var("___translations".to_string(), "__TRANSLATIONS__".to_string());

        // Compile the start node
        self.compile_node(start_node, flow, builder.clone())?;

        // Compile the rest of the flow
        self.compile_flow_from_port(flow, builder.clone(), start_node.get_port_out("__flow_out__"), start_node)?;

        Ok(())
    }

    pub fn compile_node(&mut self, node: &DisbotterFlowNode, flow: &DisbotterFlow, mut builder: CodeBuilder) -> Result<(), CompilerError> {
        self.map_node_inputs(flow, node, builder.clone())?;
        let node_type = &node.node_type;
        let node_id = &node.uid;
        let node = self.available_nodes.get(node_type);
        if node.is_none() {
            return Err(CompilerError::NodeNotFound(node_type.clone()));
        }
        let node = node.unwrap();
        builder.current_node_id = node_id.clone();
        node.call_action(&mut self.engine, builder.clone())?;
        println!("{} {} ({})", "Compiled node".green(), node.id, node_id);

        Ok(())
    }
}

/// ===< Engine upgrades >=== //
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
    pub fn call_action(&self, engine: &mut Engine, builder: CodeBuilder) -> Result<(), CompilerError> {
        let mut scope = Scope::new();
        engine.call_fn::<()>(&mut scope, &self.ast, "action", (builder,)).map_err(|e| CompilerError::RhaiError(e))?;
        Ok(())
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

// ===< Data structures >=== //
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DisbotterProjectCommand {
    uid: String,
    name: String,
    description: String,
    flow: DisbotterFlow,
    options: Vec<DisbotterProjectCommandOption>
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DisbotterProjectCommandOption {
    name: String,
    description: String,
    #[serde(rename = "type")]
    option_type: i32,
    required: bool,
    choices: Vec<String>
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
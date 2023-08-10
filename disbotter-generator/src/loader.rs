use std::{path::PathBuf, collections::{HashMap, BTreeMap}, fs::{self, File}};

use rhai::{Engine, Dynamic};

use crate::compiler::upgrade_engine;

pub struct NodeScriptLoader {
    pub engine: Engine,
}

#[derive(Debug)]
pub enum NodeScriptLoadingError {
    InvalidScript,
    MissingValue(String),
    InvalidIODeclaration
}

impl NodeScriptLoader {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        upgrade_engine(&mut engine);
        Self {
            engine
        }
    }

    fn get_variable<'a, T: Clone + 'static>(variables: &'a HashMap<String, rhai::Dynamic>, name: &str) -> Result<T, NodeScriptLoadingError> {
        if let Some(value) = variables.get(name) {
            return Ok(value.clone_cast())
        }

        Err(NodeScriptLoadingError::MissingValue(name.to_string()))
    }

    pub fn load(&mut self, script: PathBuf) -> Result<Node, NodeScriptLoadingError> {
        let ast = self.engine.compile_file(script);

        if let Err(_) = ast {
            return Err(NodeScriptLoadingError::InvalidScript);
        }

        let ast = ast.unwrap();

        let variables = ast.iter_literal_variables(true, false)
            .map(|(name, _, value)| (name.to_string(), value.clone()))
            .collect::<HashMap<String, rhai::Dynamic>>();

        let mut node = Node {
            id: Self::get_variable(&variables, "id")?,
            title: Self::get_variable(&variables, "title")?,
            description: Self::get_variable(&variables, "description")?,
            category: Self::get_variable(&variables, "category")?,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            default_hardcoded: HashMap::new(),
        };

        // Add flow io
        if !variables.contains_key("noFlowIn") && !variables.contains_key("pure") {
            node.inputs.insert("__flow_in__".to_string(), NodeIO {
                ty: NodeIOTy {
                    ty: DataType::Flow,
                    ..Default::default()
                },
                name: "flow_in".to_string(),
            });
        }
        if !variables.contains_key("noFlowOut") && !variables.contains_key("pure") {
            node.outputs.insert("__flow_out__".to_string(), NodeIO {
                ty: NodeIOTy {
                    ty: DataType::Flow,
                    ..Default::default()
                },
                name: "flow_out".to_string(),
            });
        }

        // Add inputs
        let inputs = Self::get_variable::<rhai::Map>(&variables, "inputs")?;
        for (name, input) in inputs {
            let input = input.clone().cast::<rhai::Map>();
            let input = input.iter()
                .map(|(name, value)| (name.to_string(), value.clone_cast()))
                .collect::<BTreeMap<String, Dynamic>>();
            let ty = input.get("type").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let display_name = input.get("name").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let struct_tags = input.get("struct_tags");

            let default = input.get("start_value");

            node.inputs.insert(name.to_string(), NodeIO {
                ty: NodeIOTy {
                    ty: match ty.as_str() {
                        "flow" => DataType::Flow,
                        "number" => DataType::Number,
                        "text" => DataType::Text,
                        "boolean" => DataType::Boolean,
                        "struct" => DataType::Structure,
                        _ => DataType::Any,
                    },
                    struct_tags: if struct_tags.is_some() {
                        let tags = struct_tags.unwrap().clone_cast::<rhai::Array>();
                        tags.iter().map(|tag| tag.clone_cast::<String>()).collect::<Vec<String>>()
                    } else {
                        vec![]
                    }
                },
                name: display_name.to_string(),
            });

            if default.is_some() {
                node.default_hardcoded.insert(name.to_string(), default.unwrap().clone());
            }
        }

        // Add outputs
        let outputs = Self::get_variable::<rhai::Map>(&variables, "outputs")?;
        for (name, output) in outputs {
            let output = output.clone().cast::<rhai::Map>();
            let output = output.iter().map(|(k, v)| (k.to_string(), v)).collect::<HashMap<String, &Dynamic>>();
            let ty = output.get("type").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let display_name = output.get("name").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let struct_type: Option<&&Dynamic> = output.get("struct_tags");

            node.outputs.insert(name.to_string(), NodeIO {
                ty: NodeIOTy {
                    ty: match ty.as_str() {
                        "flow" => DataType::Flow,
                        "number" => DataType::Number,
                        "text" => DataType::Text,
                        "boolean" => DataType::Boolean,
                        "struct" => DataType::Structure,
                        _ => DataType::Any,
                    },
                    struct_tags: if struct_type.is_some() {
                        let tags = struct_type.unwrap().clone_cast::<rhai::Array>();
                        tags.iter().map(|tag| tag.clone_cast::<String>()).collect::<Vec<String>>()
                    } else {
                        vec![]
                    }
                },
                name: display_name.to_string(),
            });
        }

        Ok(node)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub inputs: HashMap<String, NodeIO>,
    pub outputs: HashMap<String, NodeIO>,
    #[serde(rename = "defaultHardcoded")]
    pub default_hardcoded: HashMap<String, Dynamic>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DataType {
    Flow = 0,
    Number = 1,
    Text = 2,
    Boolean = 3,
    Structure = 4,
    #[default]
    Any = 5,
}

impl serde::Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'a> serde::Deserialize<'a> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let value = u8::deserialize(deserializer)?;
        Ok(match value {
            0 => DataType::Flow,
            1 => DataType::Number,
            2 => DataType::Text,
            3 => DataType::Boolean,
            4 => DataType::Structure,
            _ => DataType::Any,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct NodeIOTy {
    #[serde(rename = "type")]
    ty: DataType,
    #[serde(rename = "structTags")]
    struct_tags: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NodeIO {
    #[serde(rename = "type")]
    ty: NodeIOTy,
    name: String,
}

pub fn load_all_nodes(path: PathBuf) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut loader = NodeScriptLoader::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_s = path.clone();
        let path_s = path_s.to_str().unwrap_or_default().clone();

        if path.is_dir() {
            nodes.extend(load_all_nodes(path));
        } else {
            let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();

            if extension == "rhai" {
                let node = loader.load(path).expect(format!("Failed to load node script: {:?}", path_s).as_str());
                nodes.push(node);
            }
        }
    }

    nodes
}

pub fn export_node_declarations(nodes: Vec<Node>, target_path: PathBuf) {
    fs::create_dir_all(target_path.parent().unwrap()).unwrap();
    let file = File::create(target_path).unwrap();

    serde_json::ser::to_writer_pretty(file, &nodes).unwrap();
}
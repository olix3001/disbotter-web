use std::{path::PathBuf, collections::{HashMap, BTreeMap}, fs::{self, File}, hash::Hash, fmt::{Debug, Formatter}};

use rhai::{Engine, Dynamic};
use serde::ser::SerializeMap;

use crate::compiler::upgrade_engine;

/// This struct is responsible for loading node metadata from a script file
/// and converting it to a .json file that can be used by the web editor
pub struct NodeScriptLoader {
    /// RHAI engine used to load the script
    pub engine: Engine,
}

/// All possible errors that can occur while loading a node script
#[derive(Debug)]
pub enum NodeScriptLoadingError {
    InvalidScript,
    MissingValue(String),
    InvalidIODeclaration
}

impl NodeScriptLoader {
    /// Creates a new NodeScriptLoader
    pub fn new() -> Self {
        let mut engine = Engine::new();
        upgrade_engine(&mut engine);
        Self {
            engine
        }
    }

    /// Gets a variable from a HashMap, returning an error if it doesn't exist
    fn get_variable<'a, T: Clone + 'static>(variables: &'a HashMap<String, rhai::Dynamic>, name: &str) -> Result<T, NodeScriptLoadingError> {
        if let Some(value) = variables.get(name) {
            return Ok(value.clone_cast())
        }

        Err(NodeScriptLoadingError::MissingValue(name.to_string()))
    }

    /// Loads a node from a script file
    pub fn load(&mut self, script: PathBuf) -> Result<Node, NodeScriptLoadingError> {
        // Compile script into AST
        let ast = self.engine.compile_file(script);

        if let Err(_) = ast {
            return Err(NodeScriptLoadingError::InvalidScript);
        }

        let ast = ast.unwrap();

        // Get all constant variables
        let variables = ast.iter_literal_variables(true, false)
            .map(|(name, _, value)| (name.to_string(), value.clone()))
            .collect::<HashMap<String, rhai::Dynamic>>();

        // Create node based on variables
        let mut node = Node {
            id: Self::get_variable(&variables, "id")?,
            title: Self::get_variable(&variables, "title")?,
            description: Self::get_variable(&variables, "description")?,
            category: Self::get_variable(&variables, "category")?,
            inputs: KeyMap::new(),
            outputs: KeyMap::new(),
            default_hardcoded: HashMap::new(),
        };

        // Add flow I/O
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
        let mut index_map = HashMap::<String, usize>::new();
        let mut imap = HashMap::new();
        for (name, input) in inputs.iter() {
            let input = input.clone().cast::<rhai::Map>();
            let input = input.iter()
                .map(|(name, value)| (name.to_string(), value.clone_cast()))
                .collect::<BTreeMap<String, Dynamic>>();
            let ty = input.get("type").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let display_name = input.get("name").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let struct_tags = input.get("struct_tags");

            let default = input.get("start_value");

            let index = input.get("index");
            if let Some(index) = index {
                let index = index.clone_cast::<i64>();
                index_map.insert(name.to_string(), index as usize);
            } else {
                index_map.insert(name.to_string(), 100);
            }
            
            let io = NodeIO {
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
            };

            imap.insert(name.to_string(), io);

            if default.is_some() {
                node.default_hardcoded.insert(name.to_string(), default.unwrap().clone());
            }
        }

        // Add outputs
        let outputs = Self::get_variable::<rhai::Map>(&variables, "outputs")?;
        let mut output_index_map = HashMap::<String, usize>::new();
        let mut omap = HashMap::new();
        for (name, output) in outputs {
            let output = output.clone().cast::<rhai::Map>();
            let output = output.iter().map(|(k, v)| (k.to_string(), v)).collect::<HashMap<String, &Dynamic>>();
            let ty = output.get("type").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let display_name = output.get("name").ok_or(NodeScriptLoadingError::InvalidIODeclaration)?.clone_cast::<String>();
            let struct_type: Option<&&Dynamic> = output.get("struct_tags");

            let index = output.get("index");
            if let Some(index) = index {
                let index = index.clone_cast::<i64>();
                output_index_map.insert(name.to_string(), index as usize);
            } else {
                output_index_map.insert(name.to_string(), 100);
            }

            omap.insert(name.to_string(), NodeIO {
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

        node.inputs.extend_from_map_and_keymap(imap, index_map);
        node.outputs.extend_from_map_and_keymap(omap, output_index_map);

        Ok(node)
    }
}

/// Custom map type that preserves insertion order
pub struct KeyMap<K, V> {
    pub keys: Vec<K>,
    pub values: HashMap<K, V>,
}

impl<K, V> serde::Serialize for KeyMap<K, V> where K: serde::Serialize + Eq + Hash, V: serde::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut smap = serializer.serialize_map(Some(self.keys.len()))?;
        for key in self.keys.iter() {
            smap.serialize_entry(key, self.values.get(key).unwrap())?;
        }
        smap.end()
    }
}

impl<'de, K, V> serde::Deserialize<'de> for KeyMap<K, V> where K: serde::Deserialize<'de> + Eq + Hash + Clone, V: serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let map = HashMap::<K, V>::deserialize(deserializer)?;
        let keys = map.keys().cloned().collect::<Vec<K>>();
        Ok(Self {
            keys,
            values: map,
        })
    }
}

impl<K, V> Debug for KeyMap<K, V> where K: Debug, V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.values.iter()).finish()
    }
}

impl<K, V> KeyMap<K, V> where K: Clone + Hash + Eq {
    /// Returns a new KeyMap based on the given map, and a map of keys to their insertion order
    pub fn from_map_and_keymap(map: HashMap<K, V>, keymap: HashMap<K, usize>) -> Self {
        let mut keys = keymap.into_iter().collect::<Vec<(K, usize)>>();
        keys.sort_by(|(_, a), (_, b)| a.cmp(b));
        let keys = keys.into_iter().map(|(k, _)| k).collect::<Vec<K>>();
        Self {
            keys,
            values: map,
        }
    }

    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.keys.push(key.clone());
        self.values.insert(key, value);
    }

    /// Extends the KeyMap with the given map and keymap
    pub fn extend_from_map_and_keymap(&mut self, map: HashMap<K, V>, keymap: HashMap<K, usize>) {
        let mut keys = keymap.into_iter().collect::<Vec<(K, usize)>>();
        keys.sort_by(|(_, a), (_, b)| a.cmp(b));
        let keys = keys.into_iter().map(|(k, _)| k).collect::<Vec<K>>();
        self.keys.extend(keys);
        self.values.extend(map);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub inputs: KeyMap<String, NodeIO>,
    pub outputs: KeyMap<String, NodeIO>,
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

/// Loads all nodes from the given path and its subdirectories
pub fn load_all_nodes(path: PathBuf) -> Result<Vec<Node>, NodeScriptLoadingError> {
    let mut nodes: Vec<Node> = Vec::new();

    let mut loader = NodeScriptLoader::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_dir() {
            nodes.extend(load_all_nodes(path)?);
        } else {
            let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();

            if extension == "rhai" {
                let node = loader.load(path)?;
                nodes.push(node);
            }
        }
    }

    Ok(nodes)
}

/// Exports the given nodes to the given path
pub fn export_node_declarations(nodes: Vec<Node>, target_path: PathBuf) {
    fs::create_dir_all(target_path.parent().unwrap()).unwrap();
    let file = File::create(target_path).unwrap();

    serde_json::ser::to_writer_pretty(file, &nodes).unwrap();
}
use std::{sync::{Mutex, Arc}, collections::HashMap, path::PathBuf};

use rhai::CustomType;

use crate::compiler::{PortIdentifier, NodesJSCompiler};

/// Represents final program with many files
pub struct Program {
    pub files: Vec<Arc<Mutex<ProgramFile>>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    pub fn debug_print(&self) {
        for file in self.files.iter() {
            println!("====< File: {} >=====", file.lock().unwrap().path);
            println!("{}", file.lock().unwrap().code);
        }
    }

    pub fn get_file_builder(&mut self, path: String, var_cache: &Arc<Mutex<HashMap<PortIdentifier, String>>>, compiler: NodesJSCompiler) -> CodeBuilder {
        let file = Arc::new(Mutex::new(ProgramFile {
            code: String::new(),
            path: path.clone(),
        }));

        self.files.push(Arc::clone(&file));

        CodeBuilder {
            lines: Arc::new(Mutex::new(Vec::new())),
            file,
            current_ident: Arc::new(Mutex::new(0)),
            var_cache: Arc::clone(&var_cache),
            current_node_id: String::new(),
            compiler,
            var_cache_stack: Vec::new(),
        }
    }

    /// Export program files to given path
    pub fn export_to_path(&self, path: PathBuf) {
        for file in self.files.iter() {
            let mut file_path = path.clone();
            file_path.push(&file.lock().unwrap().path);
            // Ensure that directory exists
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            std::fs::write(file_path, &file.lock().unwrap().code).unwrap();
        }
    }
}

pub struct ProgramFile {
    pub code: String,
    pub path: String,
}

#[derive(Clone)]
pub struct CodeBuilder {
    pub lines: Arc<Mutex<Vec<String>>>,
    pub file: Arc<Mutex<ProgramFile>>,
    pub current_ident: Arc<Mutex<usize>>,
    pub var_cache: Arc<Mutex<HashMap<PortIdentifier, String>>>,
    pub current_node_id: String,
    pub compiler: NodesJSCompiler,
    var_cache_stack: Vec<HashMap<PortIdentifier, String>>,
}

impl CodeBuilder {
    pub fn finalize(self) -> () {
        let mut code = String::new();

        for line in self.lines.lock().unwrap().iter() {
            code.push_str(&line);
            code.push('\n');
        }

        self.file.lock().unwrap().code = code;
    }

    pub fn finalize_vec(self) -> Vec<String> {
        self.lines.lock().unwrap().drain(..).collect()
    }

    pub(crate) fn increase_ident_by(&mut self, amount: usize) {
        *self.current_ident.lock().unwrap() += amount;
    }

    pub(crate) fn decrease_ident_by(&mut self, amount: usize) {
        *self.current_ident.lock().unwrap() -= amount;
    }

    pub fn clone_empty(&self) -> Self {
        Self {
            lines: Arc::new(Mutex::new(Vec::new())),
            file: Arc::clone(&self.file),
            current_ident: Arc::clone(&self.current_ident),
            var_cache: Arc::clone(&self.var_cache),
            current_node_id: self.current_node_id.clone(),
            compiler: self.compiler.clone(),
            var_cache_stack: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: String) {
        let mut ident = String::new();

        for _ in 0..self.current_ident.lock().unwrap().clone() {
            ident.push_str("    ");
        }

        self.lines.lock().unwrap().push(format!("{}{}", ident, line));
    }

    pub fn add_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            self.add_line(line);
        }
    }

    pub fn begin_block(&mut self) {
        *self.current_ident.lock().unwrap() += 1;
        self.push_stack();
    }

    pub fn end_block(&mut self) {
        *self.current_ident.lock().unwrap() -= 1;
        self.pop_stack();
    }

    pub fn add_import(&mut self, imports: String, path: String) {
        self.lines.lock().unwrap().insert(0, 
            format!("import {{{}}} from \"{}\"", imports, path)
        );
    }

    pub fn get_in_var(&mut self, port_name: String) -> String {
        let var_cache = self.var_cache.lock().unwrap();
        let port = PortIdentifier::Input { node_uid: self.current_node_id.clone(), port_key: port_name.clone() };
        // Look for globals first
        if let Some(var) = var_cache.get(&PortIdentifier::Global { key: port_name }) {
            return var.clone();
        }

        if let Some(var) = var_cache.get(&port) {
            return var.clone();
        } else {
            return "undefined".to_string();
        }
    }

    pub fn get_out_var(&mut self, port_key: String) -> String {
        let mut var_cache = self.var_cache.lock().unwrap();
        let port = PortIdentifier::Output { node_uid: self.current_node_id.clone(), port_key: port_key.clone() };
        let nt = self.compiler.current_flow.as_ref().unwrap().get_node(&self.current_node_id).node_type.clone();

        if let Some(var) = var_cache.get(&port) {
            return var.clone();
        } else {
            let vn = format!("__io_N{}_O{}_{}", nt, port_key, NodesJSCompiler::random_var_name());
            var_cache.insert(port.clone(), vn.clone());
            return vn;
        }
    }

    pub fn set_output(&mut self, port: String, value: String) {
        let ovar = self.get_out_var(port.clone());
        self.add_line(format!("const {} = {}", ovar, value));
    }

    pub fn bind_io(&mut self, ip: String, op: String) {
        let ivar = self.get_in_var(ip.clone());
        let ovar = PortIdentifier::Output { node_uid: self.current_node_id.clone(), port_key: op.clone() };
        self.var_cache.lock().unwrap()
            .insert(ovar, ivar);
    }

    pub fn map_io(&mut self, op: String, value: String) {
        let ovar = PortIdentifier::Output { node_uid: self.current_node_id.clone(), port_key: op.clone() };
        self.var_cache.lock().unwrap()
            .insert(ovar, value);
    }

    pub fn push_stack(&mut self) {
        self.var_cache_stack.push(self.var_cache.lock().unwrap().clone());
    }

    pub fn pop_stack(&mut self) {
        let mut var_cache = self.var_cache.lock().unwrap();
        let new_cache = self.var_cache_stack.pop().unwrap();

        var_cache.clear();
        var_cache.extend(new_cache);
    }

    pub fn compile_flow_output_here(&mut self, flow_port: String) {
        // Compile the flow output here
        let new_builder = self.clone_empty();
        let cf = self.compiler.current_flow.as_ref().unwrap().clone();
        let port = PortIdentifier::Output { node_uid: self.current_node_id.clone(), port_key: flow_port };

        self.compiler.compile_flow_from_port(&cf, new_builder.clone(), port, cf.get_node(&self.current_node_id));

        // Add the code to the current builder
        self.add_lines(new_builder.finalize_vec());
    }

    pub fn get_random_var_name(&mut self) -> String {
        NodesJSCompiler::random_var_name()
    }
}

impl CustomType for CodeBuilder {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("CodeBuilder")
            .with_fn("add_line", Self::add_line)
            .with_fn("add_lines", Self::add_lines)
            .with_fn("begin_block", Self::begin_block)
            .with_fn("end_block", Self::end_block)
            .with_fn("add_import", Self::add_import)
            .with_fn("get_input", Self::get_in_var)
            .with_fn("get_out_var", Self::get_out_var)
            .with_fn("set_output", Self::set_output)
            .with_fn("bind_io", Self::bind_io)
            .with_fn("map_io", Self::map_io)
            .with_fn("push_stack", Self::push_stack)
            .with_fn("pop_stack", Self::pop_stack)
            .with_fn("get_random_var_name", Self::get_random_var_name)
            .with_fn("compile_flow_output_here", Self::compile_flow_output_here);
    }
}
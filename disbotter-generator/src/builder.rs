use std::sync::{Mutex, Arc};

use rhai::CustomType;

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

    pub fn get_file_builder(&mut self, path: String) -> CodeBuilder {
        let file = Arc::new(Mutex::new(ProgramFile {
            code: String::new(),
            path: path.clone(),
        }));

        self.files.push(Arc::clone(&file));

        CodeBuilder {
            lines: Arc::new(Mutex::new(Vec::new())),
            file,
            current_ident: Arc::new(Mutex::new(0)),
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

    pub fn add_line(&self, line: String) -> &Self {
        let mut ident = String::new();

        for _ in 0..self.current_ident.lock().unwrap().clone() {
            ident.push_str("    ");
        }

        self.lines.lock().unwrap().push(format!("{}{}", ident, line));
        self
    }

    pub fn add_lines(&self, lines: Vec<String>) -> &Self {
        for line in lines {
            self.add_line(line);
        }
        self
    }

    pub fn begin_block(&self) -> &Self {
        *self.current_ident.lock().unwrap() += 1;
        self
    }

    pub fn end_block(&self) -> &Self {
        *self.current_ident.lock().unwrap() -= 1;
        self
    }

    pub fn add_on_top(&self, line: String) -> &Self {
        self.lines.lock().unwrap().insert(0, line);
        self
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
            .with_fn("add_on_top", Self::add_on_top);
    }
}
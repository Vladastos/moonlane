use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::{ImportTree, PathRoot, Program};
use crate::error::{MoonlaneError, ParseErrorCode};
use crate::parser;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LoadedModule {
    pub module_path: Vec<String>,
    pub file_path: PathBuf,
    pub program: Program,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ModuleGraph {
    pub root: PathBuf,
    pub modules: Vec<LoadedModule>,
}

pub fn load_root(path: impl AsRef<Path>) -> Result<ModuleGraph, MoonlaneError> {
    let root = canonicalize_existing(path.as_ref())?;
    let mut loader = Loader::default();
    loader.load_module(root.clone(), Vec::new())?;
    Ok(ModuleGraph { root, modules: loader.modules })
}

pub fn load_program(path: impl AsRef<Path>) -> Result<Program, MoonlaneError> {
    let graph = load_root(path)?;
    let mut imports = Vec::new();
    let mut exports = Vec::new();
    let mut decls = Vec::new();

    for loaded in graph.modules {
        imports.extend(loaded.program.imports);
        exports.extend(loaded.program.exports);
        decls.extend(loaded.program.decls);
    }

    Ok(Program { imports, exports, decls })
}

#[derive(Default)]
struct Loader {
    modules: Vec<LoadedModule>,
    visited: HashSet<PathBuf>,
    stack: Vec<PathBuf>,
}

impl Loader {
    fn load_module(&mut self, file_path: PathBuf, module_path: Vec<String>) -> Result<(), MoonlaneError> {
        if let Some(cycle_start) = self.stack.iter().position(|p| p == &file_path) {
            let mut chain: Vec<String> = self.stack[cycle_start..]
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            chain.push(file_path.display().to_string());
            return Err(module_error(
                format!("circular module dependency: {}", chain.join(" -> ")),
                &file_path,
            ));
        }

        if self.visited.contains(&file_path) {
            return Ok(());
        }

        let source = fs::read_to_string(&file_path).map_err(|e| {
            module_error(
                format!("failed to read module '{}': {e}", file_path.display()),
                &file_path,
            )
        })?;
        let filename = file_path.display().to_string();
        let program = parser::parse(&source, &filename)?;

        validate_super_root(&program, &module_path, &file_path)?;

        self.stack.push(file_path.clone());
        for import in &program.imports {
            if let Some((module_name, child_file)) = resolve_import_module(&file_path, &import.path.root, &import.path.tree) {
                let child = canonicalize_existing(&child_file)?;
                let mut child_path = module_path.clone();
                child_path.push(module_name);
                self.load_module(child, child_path)?;
            }
        }
        self.stack.pop();

        self.visited.insert(file_path.clone());
        self.modules.push(LoadedModule { module_path, file_path, program });
        Ok(())
    }
}

fn canonicalize_existing(path: &Path) -> Result<PathBuf, MoonlaneError> {
    path.canonicalize().map_err(|e| {
        module_error(
            format!("failed to resolve module '{}': {e}", path.display()),
            path,
        )
    })
}

/// Extract the first path segment from an import declaration that corresponds to a
/// module file alongside the importing file. Returns `(module_name, candidate_path)`
/// if the import resolves to a local file, or `None` for `std::`, `root::`, and
/// other roots that don't map to a sibling file.
fn resolve_import_module(parent_file: &Path, root: &PathRoot, tree: &ImportTree) -> Option<(String, PathBuf)> {
    let parent_dir = parent_file.parent().unwrap_or_else(|| Path::new("."));

    // Only resolve imports rooted at `self::`, `super::`, or a bare name (child module).
    // `root::`, `std::` are resolved globally at a later stage.
    let first_name = match root {
        PathRoot::Self_ => {
            // self::name::... — the first segment of the tree is a child
            tree_first_segment(tree)?
        }
        PathRoot::Super => {
            // super::name::... — resolved relative to parent dir; skip for graph loading
            return None;
        }
        PathRoot::Root | PathRoot::Std => return None,
        PathRoot::Name(name) => name.clone(),
    };

    let candidate = parent_dir.join(format!("{first_name}.mln"));
    if candidate.exists() {
        Some((first_name, candidate))
    } else {
        None
    }
}

fn tree_first_segment(tree: &ImportTree) -> Option<String> {
    match tree {
        ImportTree::Name { name, .. } => Some(name.clone()),
        ImportTree::Path { name, .. } => Some(name.clone()),
        ImportTree::Group(_) | ImportTree::Glob => None,
    }
}

fn validate_super_root(program: &Program, module_path: &[String], file_path: &Path) -> Result<(), MoonlaneError> {
    if !module_path.is_empty() {
        return Ok(());
    }

    for import in &program.imports {
        if import.path.root == PathRoot::Super || import_tree_contains_super(&import.path.tree) {
            return Err(module_error("`super::` is invalid from the root module", file_path));
        }
    }

    Ok(())
}

fn import_tree_contains_super(tree: &ImportTree) -> bool {
    match tree {
        ImportTree::Name { .. } | ImportTree::Glob => false,
        ImportTree::Group(trees) => trees.iter().any(import_tree_contains_super),
        ImportTree::Path { tree, .. } => import_tree_contains_super(tree),
    }
}

fn module_error(message: impl Into<String>, path: &Path) -> MoonlaneError {
    MoonlaneError::ParseError {
        code: ParseErrorCode::P0001,
        message: message.into(),
        start: 0,
        end: 0,
        filename: path.display().to_string(),
        line: 1,
        col: 1,
        source_line: None,
    }
}

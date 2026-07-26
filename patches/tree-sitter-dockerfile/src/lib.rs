//! Dockerfile language support for tree-sitter (patched: uses tree-sitter-language 0.1 API).
//! This patch is compatible with tree-sitter 0.25+.

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_dockerfile() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for Dockerfile.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_dockerfile) };

/// The content of the `node-types.json` file for this grammar.
pub const NODE_TYPES: &str = include_str!("../src/node-types.json");

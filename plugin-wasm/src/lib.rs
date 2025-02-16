extern crate swc_node_base;

use common::{deserialize_ast, serialize_ast};
use swc_ecma_ast::Program;
use swc_ecmascript::visit::FoldWith;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process(ast: &[u8], conifg_json: &str) -> Vec<u8> {
    let ast: Program = deserialize_ast(&ast).unwrap();

    let config: plugin::Config = serde_json::from_str(conifg_json).unwrap();

    let mut v = plugin::transform(config);

    let ast = ast.fold_with(&mut v);

    serialize_ast(&ast).unwrap()
}

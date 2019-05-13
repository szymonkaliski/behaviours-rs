#[macro_use]
extern crate serde_derive;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate wasm_bindgen;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {}

#[derive(Deserialize)]
struct TreeNode(String, String);

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}\n", self.0, self.1);
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn create(tree: &JsValue) -> Simulation {
        let tree: Vec<TreeNode> = tree.into_serde().unwrap();

        log!("HERE!");

        for t in tree {
            log!("{}?", t);
        }

        Simulation {}
    }
}

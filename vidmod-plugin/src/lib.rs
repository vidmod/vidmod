#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

use vidmod_node::Node;

pub struct Plugin {
    pub make_node: fn(params: BTreeMap<String, String>) -> Node,
}

include!(concat!(env!("OUT_DIR"), "/plugins.rs"));

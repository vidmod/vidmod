use std::collections::BTreeMap;

use vidmod_node::Node;

pub struct Plugin {
    pub make_node: fn(params: BTreeMap<String, String>) -> Node,
}

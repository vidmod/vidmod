#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

use glob::glob;
use vidmod_node::Node;

pub type PluginRegSymbol<'a> = libloading::Symbol<
    'a,
    extern "C" fn() -> Vec<(String, fn(params: BTreeMap<String, String>) -> Node)>,
>;

pub struct Plugin {
    pub make_node: fn(params: BTreeMap<String, String>) -> Node,
}

lazy_static! {
    pub static ref PLUGIN_LIBRARIES: BTreeMap<String, libloading::Library> = {
        let mut res = BTreeMap::new();
        println!("Searching for plugins in {}/debug/", OUT_DIR);
        for i in glob(&format!("{}/release/libvidmod_plugins_*.so", OUT_DIR)).unwrap() {
            let lib = unsafe { libloading::Library::new(i.unwrap()).unwrap() };
            let plugin_name: libloading::Symbol<extern "C" fn() -> String> =
                unsafe { lib.get(b"plugin_name").unwrap() };
            res.insert(plugin_name(), lib);
        }
        res
    };
}

lazy_static! {
    pub static ref PLUGINS: BTreeMap<String, Plugin> = {
        let mut res = BTreeMap::new();
        for (plugin_name, lib) in PLUGIN_LIBRARIES.iter() {
            let register_plugin: PluginRegSymbol = unsafe { lib.get(b"register_plugin").unwrap() };
            for (node_name, make_node) in register_plugin() {
                res.insert(
                    format!("{}::{}", plugin_name, node_name),
                    Plugin { make_node },
                );
            }
        }
        res
    };
}

include!(concat!(env!("OUT_DIR"), "/libdir.rs"));

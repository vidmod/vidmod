use std::{
    env,
    fs::{read_dir, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use regex::Regex;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("plugins.rs");
    let mut test_file = File::create(&dest_path).unwrap();

    test_file
        .write_all(
            b"lazy_static! {
        pub static ref PLUGINS: BTreeMap<String, Plugin> = {
            let mut res = BTreeMap::new();\n",
        )
        .unwrap();

    let plugin_directories = read_dir("../vidmod-plugins/").unwrap();

    for dir in plugin_directories {
        let dir = dir.unwrap();
        if dir.file_name().to_str().unwrap().bytes().next().unwrap() == b'.' {
            continue;
        }

        let crate_name = dir.file_name().as_os_str().to_str().unwrap().to_owned();
        let crate_name_clean = crate_name.replace('-', "_");

        let lib = File::open(dir.path().join("src").join("lib.rs")).unwrap();
        for line in BufReader::new(lib).lines() {
            let r = Regex::new(r"^pub mod ([^ ]+);$").unwrap();
            if let Some(cap) = r.captures(&line.unwrap()) {
                let module_name = cap.get(1).unwrap().as_str();
                if module_name == "plugin" {
                    continue;
                }
                let module_file = dir.path().join("src").join(format!("{}.rs", module_name));
                let module = File::open(module_file).unwrap();
                let mut lines = BufReader::new(module).lines();
                while let Some(Ok(line)) = lines.next() {
                    if line == "#[node_decl]" {
                        let line = lines.next().unwrap().unwrap();
                        let r = Regex::new(r"^pub struct ([^ ]+) \{$").unwrap();
                        let plugin_struct = r.captures(&line).unwrap().get(1).unwrap().as_str();

                        test_file
                            .write_all(
                                format!(
                                    "\t\t\tlet plugin = Plugin {{ make_node: |params| Node::N2(Box::new({}::{}::{}::new(params)))}}; \n",
                                    crate_name_clean, module_name, plugin_struct
                                )
                                .as_bytes(),
                            )
                            .unwrap();

                        test_file
                            .write_all(
                                format!(
                                    "\t\t\tres.insert(\"{}::{}\".to_owned(),plugin);\n",
                                    crate_name, plugin_struct
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    }
                }
            }
        }
    }

    test_file
        .write_all(
            b"\t\t\tres
\t\t};
}",
        )
        .unwrap();

    println!("cargo:rerun-if-changed=../vidmod-plugins");
}

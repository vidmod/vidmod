use std::{env::args, fs::File, path::PathBuf, process::exit, str::FromStr};

use vidmod_core::spec::Project;

fn main() {
    if args().len() == 2 {
        let proj_path = PathBuf::from_str(&args().next_back().unwrap()).unwrap();
        if let Ok(proj_manifest) = File::open(proj_path.join("manifest.yml")) {
            let mut project = Project::load(proj_manifest, proj_path);
            while project.tick() {}
        } else {
            println!("Cannot find manifest {:?}", proj_path.join("manifest.yml"));
            exit(1);
        }
    } else {
        println!("{} [path]", args().next().unwrap());
        exit(1);
    }
}

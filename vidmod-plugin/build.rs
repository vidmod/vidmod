use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("libdir.rs");
    let mut test_file = File::create(&dest_path).unwrap();
    test_file
        .write_all(
            format!(
                "static OUT_DIR : &'static str = \"{}/../target\";",
                env!("CARGO_MANIFEST_DIR")
            )
            .as_bytes(),
        )
        .unwrap();

    println!("cargo:rerun-if-changed=../vidmod-plugins");
}

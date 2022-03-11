use std::{
    env,
    fs::{read_dir, DirEntry, File},
    io::Write,
    path::Path,
};

// build script's entry point
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    let test_data_directories = read_dir("../examples/").unwrap();

    for directory in test_data_directories {
        let directory = directory.unwrap();
        let out_dir = directory.path().canonicalize().unwrap().join("out");
        if !out_dir.exists() {
            std::fs::create_dir(out_dir).unwrap();
        }
        write_test(&mut test_file, &directory);
    }
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
    let directory = directory.path().canonicalize().unwrap();
    let path = directory.display();
    let test_name = directory.file_name().unwrap().to_string_lossy();

    write!(
        test_file,
        include_str!("./tests/test_template"),
        name = test_name,
        path = path
    )
    .unwrap();
}

fn write_header(test_file: &mut File) {
    write!(
        test_file,
        r#"
        use std::{{fs::File, path::PathBuf, str::FromStr}};

        use vidmod_core::spec::Project;
"#
    )
    .unwrap();
}

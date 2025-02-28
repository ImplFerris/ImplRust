use mdbook_gen::generate_router_build_script;
use std::{env::current_dir, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=../../md-src/");

    make_docs("blog");
    make_docs("learn");
    make_docs("resources");
}

fn make_docs(folder_path: &str) {
    let mdbook_dir = PathBuf::from("../../md-src").join(folder_path);
    let out_dir = current_dir().unwrap().join("src/docs");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push_str("\nuse super::*;\n");

    let mod_suffix = folder_path.replace(".", "");
    let filename = format!("router_{mod_suffix}.rs");

    std::fs::write(out_dir.join(filename), out).unwrap();
}

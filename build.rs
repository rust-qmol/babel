use cmake::Config;
use std::process::Command;

fn main() {
    // Command::new("git").args(&["clone", "https://github.com/openbabel/openbabel.git"]).status().unwrap();

    let dst = Config::new("openbabel").build();

    println!("{}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=openbabel");

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_args(&[format!("-I{}/include/openbabel3", dst.display())])
        .header(format!("{}/include/openbabel3/openbabel/mol.h", dst.display()))
        .wrap_unsafe_ops(true)
        .generate_comments(true)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::env::current_dir().unwrap();

    bindings
        .write_to_file(out_path.join("src/libopenbabel.rs"))
        .expect("Couldn't write bindings!");

}

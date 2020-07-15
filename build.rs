use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dune_dir = "./_build/default/callable-ocaml";
    Command::new("dune")
        .args(&["build", "./callable-ocaml/callable.exe.o"])
        .status()
        .expect("Dune failed");
    Command::new("cp")
        .args(&[
            //&format!("{}/.callable.eobjs/native/dune__exe__Callable.o", dune_dir),
            &format!("{}/callable.exe.o", dune_dir),
            &format!("{}/libcallable.o", out_dir),
        ])
        .status()
        .expect("File copy failed.");
    Command::new("rm")
        .args(&["-f", &format!("{}/libcallable.a", out_dir)])
        .status()
        .expect("rm failed");
    Command::new("ar")
        .args(&[
            "qs",
            &format!("{}/libcallable.a", out_dir),
            &format!("{}/libcallable.o", out_dir),
        ])
        .status()
        .expect("ar failed");

    println!("cargo:rerun-if-changed={}/callable.ml", dune_dir);
    println!("cargo:rerun-if-changed={}/dune", dune_dir);
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=callable");
}

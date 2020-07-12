static RUNTIME: std::sync::Once = std::sync::Once::new();

pub fn init_ocaml_runtime() {
    RUNTIME.call_once(|| {
        let arg0 = "ocaml".as_ptr() as *const i8;
        let c_args = vec![arg0, std::ptr::null()];
        unsafe { ocaml::sys::caml_main(c_args.as_ptr()) }
    });
}

extern "C" {
    fn caml_gc_compaction();
    fn caml_gc_minor();
}

pub fn collect_and_compact() {
    println!("+++ Performing full OCaml GC...");
    let now = std::time::SystemTime::now();
    unsafe { caml_gc_compaction(); }
    println!("=== Done. ({} secs)", now.elapsed().unwrap().as_secs_f32());
}

pub fn collect_minor() {
    unsafe { caml_gc_minor(); }
}

pub mod ocaml_bytes;
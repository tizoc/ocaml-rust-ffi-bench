use dmz::call::OCamlFun;
use dmz::{alloc_string, call_ocaml, with_gc, GCtoken, Val};
use lazy_static::lazy_static;

mod utils;

lazy_static! {
    static ref OCAML_TWICE: OCamlFun = OCamlFun::named("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: OCamlFun =
        OCamlFun::named("increment_bytes").expect("Missing 'increment_bytes' function");
}


pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let result = with_gc(|gc| {
        let bytes = dmz::call! { alloc_string(gc, bytes) };
        let first_n: Val<dmz::int> =
            unsafe { Val::new(gc, utils::ocaml_int_of_i32(first_n as i32)) };
        let result = call_ocaml! {OCAML_INCREMENT_BYTES(gc, bytes, first_n)};
        let result: Val<String> = result.expect("Error in 'increment_bytes' call result");
        result.eval()
    });
    return utils::string_of_ocaml(result);
}

pub fn twice(num: i32) -> i32 {
    let result = with_gc(|gc| {
        let num: Val<dmz::int> = unsafe { Val::new(gc, utils::ocaml_int_of_i32(num)) };
        let result = call_ocaml! {OCAML_TWICE(gc, num)};
        let result: Val<dmz::int> = result.expect("Error in 'twice' call result");
        result.eval()
    });
    return utils::i32_of_ocaml_int(result);
}

pub fn twice_dynamic_lookup(num: i32) -> i32 {
    let ocaml_twice: OCamlFun = OCamlFun::named("twice").expect("Missing 'twice' function");
    let result = with_gc(|gc| {
        let num: Val<dmz::int> = unsafe { Val::new(gc, utils::ocaml_int_of_i32(num)) };
        let result = call_ocaml! {ocaml_twice(gc, num)};
        let result: Val<dmz::int> = result.expect("Error in 'twice' call result");
        result.eval()
    });
    return utils::i32_of_ocaml_int(result);
}

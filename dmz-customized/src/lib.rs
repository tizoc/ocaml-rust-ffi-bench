use dmz::call::OCamlFun;
use dmz::RawValue;
use dmz::{alloc_string, with_gc, GCtoken, Val};
use lazy_static::lazy_static;

lazy_static! {
    static ref OCAML_TWICE: OCamlFun = OCamlFun::named("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: OCamlFun =
        OCamlFun::named("increment_bytes").expect("Missing 'increment_bytes' function");
}

fn string_of_ocaml(bytes: RawValue) -> String {
    let len = unsafe { dmz::caml_string_length(bytes) };
    let ptr = bytes as *const u8;
    unsafe {
        let slice = ::std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice).expect("Invalid UTF-8").into()
    }
}

pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let result = with_gc(|gc| {
        let bytes = dmz::call! { alloc_string(gc, bytes) };
        let first_n: Val<dmz::int> = unsafe { Val::new(gc, ocaml_int_of_i32(first_n as i32)) };
        let result: Val<String> = OCAML_INCREMENT_BYTES
            .call2(bytes, first_n)
            .expect("OCaml 'increment_bytes' call result")
            .mark(gc)
            .eval(gc);
        result.eval()
    });
    return string_of_ocaml(result);
}

fn ocaml_int_of_i32(num: i32) -> RawValue {
    return ((num as i64) << 1) | 1;
}

fn i32_of_ocaml_int(num: RawValue) -> i32 {
    return (num as i32) >> 1;
}

pub fn twice(num: i32) -> i32 {
    let result = with_gc(|gc| {
        let num: Val<dmz::int> = unsafe { Val::new(gc, ocaml_int_of_i32(num)) };
        let result: Val<dmz::int> = OCAML_TWICE
            .call(num)
            .expect("OCaml 'twice' call result")
            .mark(gc)
            .eval(gc);
        result.eval()
    });
    return i32_of_ocaml_int(result);
}

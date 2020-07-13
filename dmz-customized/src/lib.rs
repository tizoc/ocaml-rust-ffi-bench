use dmz::call::{ocaml_named_function, OCamlFun};
use dmz::RawValue;
use dmz::{alloc_string, with_gc, GCtoken, Val};
use lazy_static::lazy_static;

lazy_static! {
    static ref OCAML_TWICE: Val<'static, OCamlFun> = ocaml_named_function("twice");
    static ref OCAML_INCREMENT_BYTES: Val<'static, OCamlFun> =
        ocaml_named_function("increment_bytes");
}

fn ocaml_of_str(bytes: &str) -> RawValue {
    with_gc(|gc| {
        let result = dmz::call! { alloc_string(gc, bytes) };
        result.eval()
    })
}

fn string_of_ocaml(bytes: RawValue) -> String {
    let len = unsafe { dmz::caml_string_length(bytes) };
    let ptr = bytes as *const u8;
    unsafe {
        let slice = ::std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice).expect("Invalid UTF-8").into()
    }
}

pub fn ocaml_increment_bytes_internal(bytes: RawValue, first_n: RawValue) -> RawValue {
    with_gc(|gc| {
        let arg1: Val<String> = unsafe { Val::new(gc, bytes) };
        let arg2: Val<dmz::int> = unsafe { Val::new(gc, first_n) };
        let result: Val<String> = OCAML_INCREMENT_BYTES
            .call2(arg1, arg2)
            .expect("OCaml 'increment_bytes' call result")
            .mark(gc)
            .eval(gc);
        result.eval()
    })
}

pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let bytes = ocaml_of_str(bytes);
    let first_n = ocaml_int_of_i32(first_n as i32);
    let result = ocaml_increment_bytes_internal(bytes, first_n);
    return string_of_ocaml(result);
}

fn ocaml_int_of_i32(num: i32) -> RawValue {
    return ((num as i64) << 1) | 1;
}

fn i32_of_ocaml_int(num: RawValue) -> i32 {
    return (num as i32) >> 1;
}

fn ocaml_twice_internal(num: RawValue) -> RawValue {
    with_gc(|gc| {
        let arg: Val<dmz::int> = unsafe { Val::new(gc, num) };
        let result: Val<dmz::int> = OCAML_TWICE
            .call(arg)
            .expect("OCaml 'twice' call result")
            .mark(gc)
            .eval(gc);
        result.eval()
    })
}

pub fn twice(n: i32) -> i32 {
    let n = ocaml_int_of_i32(n);
    let result = ocaml_twice_internal(n);
    return i32_of_ocaml_int(result);
}

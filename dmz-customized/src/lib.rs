use dmz::call::OCamlFun;
use dmz::{alloc_string, with_gc, GCtoken, Val, call_ocaml};
use lazy_static::lazy_static;

lazy_static! {
    static ref OCAML_TWICE: OCamlFun = OCamlFun::named("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: OCamlFun =
        OCamlFun::named("increment_bytes").expect("Missing 'increment_bytes' function");
}

mod utils {
    use dmz::RawValue;

    pub fn ocaml_int_of_i32(num: i32) -> RawValue {
        return ((num as i64) << 1) | 1;
    }

    pub fn i32_of_ocaml_int(num: RawValue) -> i32 {
        return (num as i32) >> 1;
    }

    pub fn string_of_ocaml(bytes: RawValue) -> String {
        let len = unsafe { dmz::caml_string_length(bytes) };
        let ptr = bytes as *const u8;
        unsafe {
            let slice = ::std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8(slice).expect("Invalid UTF-8").into()
        }
    }
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

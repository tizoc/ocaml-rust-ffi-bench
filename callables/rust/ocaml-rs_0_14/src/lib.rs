use ocaml::ToValue;

// More efficient version than what ocaml-rs provides.
// Since ocaml-rs maps OCaml bytes/string into String,
// it needs to do extra work to validate that the contents
// of the memory chunk is valid utf-8.
// This implementation copies memory directly and avoids
// such checks.
fn u8vec_from_ocaml_bytes(bytes: ocaml::Value) -> Vec<u8> {
    unsafe {
        let len = ocaml::sys::caml_string_length(bytes.0);
        let src_ptr = ocaml::sys::string_val(bytes.0);
        let mut dst = Vec::with_capacity(len);
        let dst_ptr = dst.as_mut_ptr();
        core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        dst.set_len(len);
        dst
    }
}

#[ocaml::native_func]
pub fn rust_increment_bytes(bytes: ocaml::Value, first_n: ocaml::Value) -> ocaml::Value {
    let mut bytes = u8vec_from_ocaml_bytes(bytes);
    let first_n = first_n.int_val() as usize;

    for i in 0..first_n {
        bytes[i] += 1;
    }

    return bytes.to_value();
}

#[ocaml::native_func]
pub fn rust_decrement_bytes(bytes: ocaml::Value, first_n: ocaml::Value) -> ocaml::Value {
    let mut bytes = u8vec_from_ocaml_bytes(bytes);
    let first_n = first_n.int_val() as usize;

    for i in 0..first_n {
        bytes[i] -= 1;
    }

    return bytes.to_value();
}

#[ocaml::native_func]
pub fn rust_twice(num: ocaml::Value) -> ocaml::Value {
    let n = num.int_val();
    return ocaml::Value::int(n * 2);
}
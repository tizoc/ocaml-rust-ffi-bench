use lazy_static::lazy_static;
use ocaml::Value;
use ocaml::{FromValue, ToValue};

lazy_static! {
    static ref OCAML_TWICE: Value = Value::named("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: Value =
        Value::named("increment_bytes").expect("Missing 'increment_bytes' function");
}

#[ocaml::native_func]
pub fn ocaml_increment_bytes_internal(bytes: ocaml::Value, first_n: ocaml::Value) -> ocaml::Value {
    let result = OCAML_INCREMENT_BYTES
        .call2(bytes, first_n)
        .expect("OCaml 'increment_bytes' call result");
    return result;
}

pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let result = ocaml_increment_bytes_internal(bytes.to_value(), first_n.to_value());
    return String::from_value(result);
}

#[ocaml::native_func]
pub fn ocaml_twice_internal(num: ocaml::Value) -> ocaml::Value {
    let result = OCAML_TWICE.call(num).expect("OCaml 'twice' call result");
    return result;
}

pub fn twice(n: i32) -> i32 {
    let ocaml_n = (n as ocaml::Int).to_value();
    let result = ocaml_twice_internal(ocaml_n);
    return result.int_val() as i32;
}

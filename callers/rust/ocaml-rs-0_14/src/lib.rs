use lazy_static::lazy_static;
use ocaml::Value;
use ocaml::{FromValue, ToValue};

mod ocamlfun;

use ocamlfun::OCamlFun;

// FIXME: not fully safe, ocaml-rs returns the actual value and
// not a pointer to the location with the value. There is no guarantee
// in OCaml's spec that says that the location will not change.
lazy_static! {
    static ref OCAML_TWICE_SAFE: OCamlFun = OCamlFun::named("twice").expect("Missing 'twice' function");
    static ref OCAML_TWICE: Value = Value::named("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: Value =
        Value::named("increment_bytes").expect("Missing 'increment_bytes' function");
    static ref OCAML_DECREMENT_BYTES: Value =
        Value::named("decrement_bytes").expect("Missing 'decrement_bytes' function");
}

#[ocaml::native_func]
pub fn ocaml_increment_bytes(bytes: ocaml::Value, first_n: ocaml::Value) -> ocaml::Value {
    let result = OCAML_INCREMENT_BYTES
        .call2(bytes, first_n)
        .expect("OCaml 'increment_bytes' call result");
    return result;
}

pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let result = ocaml_increment_bytes(bytes.to_value(), first_n.to_value());
    return String::from_value(result);
}

#[ocaml::native_func]
pub fn ocaml_decrement_bytes(bytes: ocaml::Value, first_n: ocaml::Value) -> ocaml::Value {
    let result = OCAML_DECREMENT_BYTES
        .call2(bytes, first_n)
        .expect("OCaml 'decrement_bytes' call result");
    return result;
}

pub fn decrement_bytes(bytes: &str, first_n: usize) -> String {
    let result = ocaml_decrement_bytes(bytes.to_value(), first_n.to_value());
    return String::from_value(result);
}

#[ocaml::native_func]
pub fn ocaml_twice(num: ocaml::Value) -> ocaml::Value {
    let result = OCAML_TWICE.call(num).expect("OCaml 'twice' call result");
    return result;
}

#[ocaml::native_func]
pub fn ocaml_twice_safe(num: ocaml::Value) -> ocaml::Value {
    let result = OCAML_TWICE_SAFE.call(num).expect("OCaml 'twice' call result");
    return result;
}

pub fn twice(n: i32) -> i32 {
    let ocaml_n = (n as ocaml::Int).to_value();
    let result = ocaml_twice(ocaml_n);
    return result.int_val() as i32;
}

pub fn twice_safe(n: i32) -> i32 {
    let ocaml_n = (n as ocaml::Int).to_value();
    let result = ocaml_twice_safe(ocaml_n);
    return result.int_val() as i32;
}

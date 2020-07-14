use lazy_static::lazy_static;
use ocaml::{FromValue, ToValue};

// FIXME: not fully safe, ocaml-rs returns the actual value and
// not a pointer to the location with the value. There is no guarantee
// in OCaml's spec that says that the location will not change.
lazy_static! {
    static ref OCAML_TWICE: ocaml::value::Value =
        ocaml::named_value("twice").expect("Missing 'twice' function");
    static ref OCAML_INCREMENT_BYTES: ocaml::value::Value =
        ocaml::named_value("increment_bytes").expect("Missing 'increment_bytes' function");
}

pub fn increment_bytes(bytes: &str, first_n: usize) -> String {
    let ret: String;

    ocaml::caml_body!(||, <result, ocaml_bytes, ocaml_first_n>, {
        ocaml_bytes = bytes.to_value();
        ocaml_first_n = first_n.to_value();
        result = OCAML_INCREMENT_BYTES.call2_exn(ocaml_bytes, ocaml_first_n)
            .expect("OCaml 'increment_bytes' call result");
        ret = String::from_value(result);
    });

    return ret;
}

pub fn twice(n: i32) -> i32 {
    let ret: i32;

    ocaml::caml_body!(||, <result, ocaml_num>, {
        ocaml_num = n.to_value();
        result = OCAML_TWICE.call_exn(ocaml_num)
            .expect("OCaml 'twice' call result");
        ret = result.i32_val();
    });

    return ret;
}

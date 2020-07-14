use ocaml_util::ocaml_bytes::OCamlBytes;
use ocaml::ToValue;

fn main() {
    //ocaml::runtime::init();
    ocaml_util::init_ocaml_runtime();

    let bytes = "0000000000000000";
    let mut ocaml_bytes = OCamlBytes::alloc(16);

    ocaml_bytes.keep();

    let increment_bytes_result_14 = ocaml_rs_0_14::increment_bytes(bytes, 10);

    let twice_result_14 = ocaml_rs_0_14::twice(10);

    println!(
        "Result for increment_bytes 0.14: {}",
        increment_bytes_result_14
    );

    println!("Result for twice 0.14: {}", twice_result_14);

    ocaml_rs_0_14::ocaml_increment_bytes(ocaml_bytes.to_value(), 10.to_value());
    println!(
        "First byte for ocaml_increment_bytes 0.14: {}",
        ocaml_bytes.get_byte(0).unwrap()
    );
    ocaml_rs_0_14::ocaml_decrement_bytes(ocaml_bytes.to_value(), 10.to_value());
    println!(
        "First byte for ocaml_decrement_bytes 0.14: {}",
        ocaml_bytes.get_byte(0).unwrap()
    );

    ocaml_bytes.dispose();

    let increment_bytes_result_9 = ocaml_rs_0_09::increment_bytes(bytes, 10);
    let twice_result_09 = ocaml_rs_0_09::twice(10);

    println!(
        "Result for increment_bytes 0.9.3: {}",
        increment_bytes_result_9
    );
    println!("Result for twice 0.9.3: {}", twice_result_09);

    let increment_bytes_result_dmz = dmz_customized::increment_bytes(bytes, 10);
    let twice_result_dmz = dmz_customized::twice(10);
    println!(
        "Result for increment_bytes dmz: {}",
        increment_bytes_result_dmz
    );
    println!("Result for twice tmz: {}", twice_result_dmz);


    ocaml::runtime::shutdown();
}

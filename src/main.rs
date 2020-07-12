fn main() {
    //ocaml::runtime::init();
    ocaml_util::init_ocaml_runtime();

    let bytes = "0000000000000000";

    let increment_bytes_result_14 = ocaml_rs_0_14::increment_bytes(bytes, 10);
    let twice_result_14 = ocaml_rs_0_14::twice(10);

    println!(
        "Result for increment_bytes 0.14: {}",
        increment_bytes_result_14
    );
    println!("Result for twice 0.14: {}", twice_result_14);

    let increment_bytes_result_9 = ocaml_rs_0_09::increment_bytes(bytes, 10);
    let twice_result_09 = ocaml_rs_0_09::twice(10);

    println!(
        "Result for increment_bytes 0.9.3: {}",
        increment_bytes_result_9
    );
    println!("Result for twice 0.9.3: {}", twice_result_09);

    ocaml::runtime::shutdown();
}

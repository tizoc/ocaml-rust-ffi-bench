use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dmz_customized;
use ocaml::ToValue;
use ocaml_rs_0_09;
use ocaml_rs_0_14;
use ocaml_util::ocaml_bytes::OCamlBytes;

fn criterion_benchmark(c: &mut Criterion) {
    ocaml_util::init_ocaml_runtime();
    ocaml_util::collect_and_compact();

    let first10 = 10;
    let bytes16 = String::from("00000000000000000000000000000000");
    let bytes32 = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let bytes64 = String::from("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let bytes256 = String::from("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let mut ocaml_bytes256 = OCamlBytes::alloc(256);

    ocaml_bytes256.keep();

    // Basic call with integer
    c.bench_function("ocaml_rs_0_09::twice(20)", |b| {
        b.iter(|| ocaml_rs_0_09::twice(black_box(20)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::twice(20)", |b| {
        b.iter(|| ocaml_rs_0_14::twice(black_box(20)))
    });
    ocaml_util::collect_and_compact();
    // Pointer-safe version with custom ocaml funcall implementation
    c.bench_function("ocaml_rs_0_14::twice_safe(20)", |b| {
        b.iter(|| ocaml_rs_0_14::twice_safe(black_box(20)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::twice(20)", |b| {
        b.iter(|| dmz_customized::twice(black_box(20)))
    });
    ocaml_util::collect_and_compact();

    // Test with dynamic lookup of ocaml function
    c.bench_function("dmz_customized::twice_dynamic_lookup(20)", |b| {
        b.iter(|| dmz_customized::twice_dynamic_lookup(black_box(20)))
    });
    ocaml_util::collect_and_compact();

    // Calls with copied byte arrays
    c.bench_function("ocaml_rs_0_09::increment_bytes(16bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_09::increment_bytes(black_box(&bytes16), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::increment_bytes(16bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_14::increment_bytes(black_box(&bytes16), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::increment_bytes(16bytes, first10)", |b| {
        b.iter(|| dmz_customized::increment_bytes(black_box(&bytes16), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_09::increment_bytes(32bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_09::increment_bytes(black_box(&bytes32), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::increment_bytes(32bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_14::increment_bytes(black_box(&bytes32), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::increment_bytes(32bytes, first10)", |b| {
        b.iter(|| dmz_customized::increment_bytes(black_box(&bytes32), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_09::increment_bytes(64bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_09::increment_bytes(black_box(&bytes64), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::increment_bytes(64bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_14::increment_bytes(black_box(&bytes64), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::increment_bytes(64bytes, first10)", |b| {
        b.iter(|| dmz_customized::increment_bytes(black_box(&bytes64), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_09::increment_bytes(256bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_09::increment_bytes(black_box(&bytes256), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::increment_bytes(256bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_14::increment_bytes(black_box(&bytes256), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::increment_bytes(256bytes, first10)", |b| {
        b.iter(|| dmz_customized::increment_bytes(black_box(&bytes256), black_box(first10)))
    });
    ocaml_util::collect_and_compact();

    c.bench_function(
        "ocaml_rs_0_14::ocaml_increment_bytes(256bytes_shared, first10)",
        |b| {
            b.iter(|| {
                ocaml_rs_0_14::ocaml_increment_bytes(
                    black_box(ocaml_bytes256.to_value()),
                    black_box(first10.to_value()),
                );
                // Restore bytes
                //ocaml_rs_0_14::ocaml_decrement_bytes(
                //    black_box(ocaml_bytes256.to_value()),
                //    black_box(first10.to_value()),
                //);
            })
        },
    );
    ocaml_util::collect_and_compact();

    ocaml_bytes256.dispose();

    ocaml::shutdown();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

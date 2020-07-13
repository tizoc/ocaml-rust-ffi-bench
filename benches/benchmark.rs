use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ocaml_rs_0_09;
use ocaml_rs_0_14;
use dmz_customized;
use ocaml_util;

fn criterion_benchmark(c: &mut Criterion) {
    ocaml_util::init_ocaml_runtime();
    ocaml_util::collect_and_compact();

    let first10 = 10;
    let bytes16 = String::from("00000000000000000000000000000000");
    let bytes32 = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let bytes64 = String::from("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let bytes256 = String::from("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // Basic call with integer
    c.bench_function("ocaml_rs_0_09::twice(20)", |b| {
        b.iter(|| ocaml_rs_0_09::twice(black_box(20)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("ocaml_rs_0_14::twice(20)", |b| {
        b.iter(|| ocaml_rs_0_14::twice(black_box(20)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::twice(20)", |b| {
        b.iter(|| dmz_customized::twice(black_box(20)))
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
    ocaml_util::collect_and_compact();*/
    c.bench_function("ocaml_rs_0_14::increment_bytes(256bytes, first10)", |b| {
        b.iter(|| ocaml_rs_0_14::increment_bytes(black_box(&bytes256), black_box(first10)))
    });
    ocaml_util::collect_and_compact();
    c.bench_function("dmz_customized::increment_bytes(256bytes, first10)", |b| {
        b.iter(|| dmz_customized::increment_bytes(black_box(&bytes256), black_box(first10)))
    });
    ocaml_util::collect_and_compact();

    ocaml::shutdown();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

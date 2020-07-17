module OCamlRS014 = struct
  external increment_bytes: bytes -> int -> bytes = "rust_increment_bytes"
  external increment_bytes_inplace: bytes -> int -> bytes = "rust_increment_bytes_inplace"
  external decrement_bytes: bytes -> int -> bytes = "rust_decrement_bytes"
  external decrement_bytes_inplace: bytes -> int -> bytes = "rust_decrement_bytes_inplace"
  external twice: int -> int = "rust_twice"
end

let assert_correct () =
  let bytes16 = Bytes.make 16 '0' in
  assert (Bytes.equal (OCamlRS014.increment_bytes bytes16 10)
                      (Bytes.of_string "1111111111000000"));
  let bytes16 = OCamlRS014.increment_bytes_inplace bytes16 10 in
  assert (Bytes.equal bytes16 (Bytes.of_string "1111111111000000"));
  let bytes16 = OCamlRS014.decrement_bytes_inplace bytes16 10 in
  assert (Bytes.equal bytes16 (Bytes.of_string "0000000000000000"));
  print_endline "Assertions passed"

open! Core
open Core_bench
let () =
  assert_correct ();
  let n = 10 in
  let bytes16 = Bytes.make 16 '0' in
  let bytes32 = Bytes.make 32 '0' in
  let bytes64 = Bytes.make 64 '0' in
  let bytes256 = Bytes.make 256 '0' in
  let bytes256shared = Bytes.make 256 '0' in
  Command.run (Bench.make_command [
    Bench.Test.create ~name:"OCamlRS014.twice(10)" (fun () ->
      ignore (OCamlRS014.twice n));
    Bench.Test.create ~name:"OCamlRS014.increment_bytes(bytes16, 10)" (fun () ->
      ignore (OCamlRS014.increment_bytes bytes16 10));
    Bench.Test.create ~name:"OCamlRS014.increment_bytes(bytes32, 10)" (fun () ->
      ignore (OCamlRS014.increment_bytes bytes32 10));
    Bench.Test.create ~name:"OCamlRS014.increment_bytes(bytes64, 10)" (fun () ->
      ignore (OCamlRS014.increment_bytes bytes64 10));
    Bench.Test.create ~name:"OCamlRS014.increment_bytes(bytes256, 10)" (fun () ->
      ignore (OCamlRS014.increment_bytes bytes256 10));
    Bench.Test.create ~name:"OCamlRS014.increment_bytes_inplace(bytes256shared, 10)" (fun () ->
      ignore (OCamlRS014.increment_bytes_inplace bytes256shared 10));
  ])
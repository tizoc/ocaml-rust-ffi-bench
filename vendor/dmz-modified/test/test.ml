let bigstring = Alcotest.testable Bigstring.print Bigstring.equal

let to_string () =
  Alcotest.(check string)
    "string from tuple"
    "str: hello, int: 42"
    (Externals.tuple_to_string ("hello", 42))

let string_tail () =
  Alcotest.(check string)
    "tail of string"
    "ello"
    (match Externals.strtail "hello" with Some s -> s | None -> "?")

let bytes_tail () =
  Alcotest.(check string)
    "tail of bytes"
    "ello"
    (match Externals.bytestail (Bytes.of_string "hello") with
     | Some s -> (Bytes.unsafe_to_string s)
     | None -> "?")

let bigstring_tail () =
  Alcotest.(check bigstring)
    "tail of bigstring"
    (Bigstring.of_string "ELLO")
    (match Externals.bigstrtail (Bigstring.of_string "HELLO") with
     | Some bs -> bs
     | None -> (Bigstring.of_string "?"))

let construct_three_tuple () =
  Alcotest.(check int)
    "build three-tuple by duplicating int"
    42
    (let (_a, _b, c) = (Externals.tuple3 42) in c)

let recordfst () =
  let r : Externals.foobar = { foo = 1 ; bar = 2 } in
  Alcotest.(check int)
    "access field in record"
    1
    (Externals.recordfst r)

let recordsnd () =
  let r : Externals.foobar = { foo = 1 ; bar = 2 } in
  Alcotest.(check int)
    "access field in record"
    2
    (Externals.recordsnd r)

let div () =
  Alcotest.(check int)
    "division"
    1
    (match Externals.div 2 2 with Ok x -> x | Error _ -> 0)

let divbyzero () =
  Alcotest.(check string)
    "division by zero"
    "Divide by zero"
    (match Externals.div 2 0 with Ok x -> string_of_int x | Error s -> s)

let nth () =
  Alcotest.(check int)
    "nth list element"
    (42 * 42)
    (match Externals.nth (List.init 43 (fun x -> x * x)) 42 with Some x -> x | None -> -1)

let is_even () =
  Alcotest.(check bool)
    "is even?"
    true
    (Externals.is_even 42)

let bool_to_int () =
  Alcotest.(check int)
    "is even?"
    0
    (Externals.bool_to_int false)

let inc () = Alcotest.(check int) "increment int" 43 (Externals.inc 42)
let inc64 () = Alcotest.(check int64) "increment int64" 43L (Externals.inc64 42L)
let atoi () = Alcotest.(check int) "convert char to int" 42 (Externals.atoi '*')
let itoa () = Alcotest.(check char) "convert int to char" '*' (Externals.itoa 42)

let tests = [
    "tuple to string",                     `Quick, to_string ;
    "tail of string",                      `Quick, string_tail ;
    "tail of bytes",                       `Quick, bytes_tail ;
    "increment int",                       `Quick, inc ;
    "increment int64",                     `Quick, inc64 ;
    "convert char to int",                 `Quick, atoi ;
    "convert int to char",                 `Quick, itoa ;
    "tail of bigstring",                   `Quick, bigstring_tail ;
    "construct three-tuple",               `Quick, construct_three_tuple ;
    "access first field in record",        `Quick, recordfst ;
    "access second field in record",       `Quick, recordsnd ;
    "division",                            `Quick, div ;
    "division by zero",                    `Quick, divbyzero ;
    "nth list element",                    `Quick, nth ;
    "is_even",                             `Quick, is_even ;
    "bool_to_int",                         `Quick, bool_to_int ;
  ]

let () =
  Alcotest.run "dmz" [
      "tests", tests;
    ]


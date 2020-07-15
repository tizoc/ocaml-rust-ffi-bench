let () =
  for _i = 1 to 1000 do
    let a = ref 10 and b = ref 20 in
    let a', b' = Externals.mkpair a b in
    if a <> a || b <> b' then begin
      Printf.printf "%d=%d, %d=%d\n%!" !a !a' !b !b';
      assert false;
    end
  done;

  Printf.printf "%s\n" (Externals.tuple_to_string ("hello", 42));
  Printf.printf "%s\n" (match Externals.strtail "hello" with Some s -> s | None -> "?");

  let _pair =
    match Externals.somestr 42 with
    | Some s -> Printf.printf "Some: %s\n" s
    | None -> Printf.printf "None" in

  let _ = Externals.printint 42 in
  let _ = Externals.printint64 42L in
  let _ = Externals.printchar (Char.chr 72) in

  let _ = Externals.printbigstring (Bigstring.of_string "HELLO") in
  Printf.printf "%s\n" (match Externals.bigstrtail (Bigstring.of_string "HELLO") with
                        | Some bs -> Bigstring.to_string bs
                        | None -> "?");
  ()

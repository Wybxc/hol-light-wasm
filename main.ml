let () =
    let a = Z.of_int 1234 in
    let s = Z.sign a in
    assert (s = 1);
    print_endline "OK: sign"
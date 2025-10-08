let () =
    let a = Z.of_int 1234 in
    let s = Z.sign a in
    print_int s;
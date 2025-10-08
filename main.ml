let () =
    (* Test Z.sign *)
    let a = Z.of_int 1234 in
    let s = Z.sign a in
    assert (s = 1);
    print_endline "OK: sign positive";

    let b = Z.of_int (-567) in
    let s = Z.sign b in
    assert (s = -1);
    print_endline "OK: sign negative";

    let c = Z.of_int 0 in
    let s = Z.sign c in
    assert (s = 0);
    print_endline "OK: sign zero";

    (* Test Z.shift_left *)
    let d = Z.of_int 5 in
    let shifted = Z.shift_left d 2 in
    let expected = Z.of_int 20 in (* 5 << 2 = 20 *)
    assert (Z.equal shifted expected);
    print_endline "OK: shift_left";

    let e = Z.of_int 1 in
    let shifted2 = Z.shift_left e 10 in
    let expected2 = Z.of_int 1024 in (* 1 << 10 = 1024 *)
    assert (Z.equal shifted2 expected2);
    print_endline "OK: shift_left large";

    (* Test Z.pred *)
    let f = Z.of_int 100 in
    let pred_f = Z.pred f in
    let expected_pred = Z.of_int 99 in
    assert (Z.equal pred_f expected_pred);
    print_endline "OK: pred positive";

    let g = Z.of_int 0 in
    let pred_g = Z.pred g in
    let expected_pred_g = Z.of_int (-1) in
    assert (Z.equal pred_g expected_pred_g);
    print_endline "OK: pred zero";

    let h = Z.of_int (-5) in
    let pred_h = Z.pred h in
    let expected_pred_h = Z.of_int (-6) in
    assert (Z.equal pred_h expected_pred_h);
    print_endline "OK: pred negative";

    (* Test with larger numbers *)
    (* let large = Z.of_string "123456789012345" in
    let large_sign = Z.sign large in
    assert (large_sign = 1);
    print_endline "OK: sign large number"; *)

    (* let large_pred = Z.pred large in
    let expected_large_pred = Z.of_string "123456789012344" in
    assert (Z.equal large_pred expected_large_pred);
    print_endline "OK: pred large number"; *)

    (* Test Z.add *)
    let x = Z.of_int 15 in
    let y = Z.of_int 25 in
    let sum = Z.add x y in
    assert (Z.equal sum (Z.of_int 40));
    print_endline "OK: add";

    (* Test Z.sub *)
    let x = Z.of_int 50 in
    let y = Z.of_int 30 in
    let diff = Z.sub x y in
    assert (Z.equal diff (Z.of_int 20));
    print_endline "OK: sub";

    (* Test Z.mul *)
    let x = Z.of_int 6 in
    let y = Z.of_int 7 in
    let product = Z.mul x y in
    assert (Z.equal product (Z.of_int 42));
    print_endline "OK: mul";

    (* Test Z.div *)
    let x = Z.of_int 42 in
    let y = Z.of_int 6 in
    let quotient = Z.div x y in
    assert (Z.equal quotient (Z.of_int 7));
    print_endline "OK: div";

    (* Test Z.rem *)
    let x = Z.of_int 23 in
    let y = Z.of_int 5 in
    let remainder = Z.rem x y in
    assert (Z.equal remainder (Z.of_int 3));
    print_endline "OK: rem";

    (* Test Z.compare *)
    let x = Z.of_int 10 in
    let y = Z.of_int 20 in
    let z = Z.of_int 10 in
    assert (Z.compare x y = -1);
    assert (Z.compare y x = 1);
    assert (Z.compare x z = 0);
    print_endline "OK: compare";

    (* Test Z.abs *)
    let x = Z.of_int (-42) in
    let y = Z.of_int 42 in
    let abs_x = Z.abs x in
    let abs_y = Z.abs y in
    assert (Z.equal abs_x (Z.of_int 42));
    assert (Z.equal abs_y (Z.of_int 42));
    print_endline "OK: abs";

    (* Test Z.neg *)
    let x = Z.of_int 42 in
    let neg_x = Z.neg x in
    assert (Z.equal neg_x (Z.of_int (-42)));
    print_endline "OK: neg";

    (* Test Z.succ *)
    let x = Z.of_int 99 in
    let succ_x = Z.succ x in
    assert (Z.equal succ_x (Z.of_int 100));
    print_endline "OK: succ";

    (* Test Z.to_int *)
    let x = Z.of_int 42 in
    let i = Z.to_int x in
    assert (i = 42);
    print_endline "OK: to_int";

    (* Test Z.gcd *)
    let x = Z.of_int 48 in
    let y = Z.of_int 18 in
    let gcd_xy = Z.gcd x y in
    assert (Z.equal gcd_xy (Z.of_int 6));
    print_endline "OK: gcd";

    (* Test Z.fdiv and Z.cdiv *)
    let x = Z.of_int 23 in
    let y = Z.of_int 5 in
    let fdiv_result = Z.fdiv x y in
    let cdiv_result = Z.cdiv x y in
    assert (Z.equal fdiv_result (Z.of_int 4));
    assert (Z.equal cdiv_result (Z.of_int 5));
    print_endline "OK: fdiv and cdiv";

    (* Test Z.divexact *)
    let x = Z.of_int 42 in
    let y = Z.of_int 6 in
    let exact_div = Z.divexact x y in
    assert (Z.equal exact_div (Z.of_int 7));
    print_endline "OK: divexact";

      print_endline "All tests passed!";
let rec fib n =
  if n < 3 then n
  else fib (n - 1) + fib (n - 2)

let () =
  let n = 43 in
  let res = fib n in
  print_int res
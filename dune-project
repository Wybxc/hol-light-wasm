(lang dune 3.14)

(name hol_light_wasm)

(version 0.1)

(generate_opam_files false)

(package
 (name hol_light_wasm)
 (synopsis "HOL Light WASM")
 (description "HOL Light compiled to WebAssembly")
 (depends
  (ocaml
   (>= 5.2.0))
  (camlp5 :build)))

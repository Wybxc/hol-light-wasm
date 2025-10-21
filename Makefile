main.wat: main.wasm
	wasm-tools print $< > $@

main.preinit.wasm: main.wasm
	wizer $< --allow-wasi -o $@

SKIPOPT=--traps-never-happen
O3=--dae --gufa --global-refining --heap-store-optimization --heap2local --inlining-optimizing -O3 $(SKIPOPT) --metrics
Oz=-Oz $(SKIPOPT) --metrics
WASMOPTFLAGS=-g --closed-world --metrics --precompute --type-unfinalizing --generate-global-effects --ssa --type-ssa --metrics $(O3) $(O3) $(O3) --monomorphize $(O3) --gto --type-merging --minimize-rec-groups --converge $(Oz) --type-finalizing --metrics

main.wasm: main.js
	wasm-opt $(WASMOPTFLAGS) main.assets/code.wasm -o $@

main.js: main.byte
	wasm_of_ocaml --enable wasi --enable exnref $< -o $@

main.byte: main.ml
	dune build
	rm -f $@
	cp _build/default/main.bc $@

switch:
	@echo "Creating and switching to OCaml 5.2.0 environment with necessary packages..."
	opam switch create . 5.2.0
	opam pin add js_of_ocaml git+https://github.com/ocsigen/js_of_ocaml.git#6af5f5ce9b9bee9a58d07d7c97c8621eb3eb5845 --confirm-level=yes
	opam pin add js_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#6af5f5ce9b9bee9a58d07d7c97c8621eb3eb5845 --confirm-level=yes
	opam pin add wasm_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#6af5f5ce9b9bee9a58d07d7c97c8621eb3eb5845 --confirm-level=yes

hol-light:
	@echo "Building and pinning hol_light package..."
	cd hol-light && make HOLLIGHT_USE_MODULE=1
	opam pin add hol_light ./hol-light --kind=local --confirm-level=yes
	opam reinstall hol_light --verbose --confirm-level=yes

clean:
	rm -rf *.cm* *.o *.wasm *.byte *.wat *.js
	rm -rf *.assets/

.PHONY: clean switch hol-light

example.wat: example.wasm
	wasm-tools print $< > $@

example.wasm: example.js
	cp example.assets/code.wasm example.wasm

example.js: example.byte
	wasm_of_ocaml --enable wasi --enable exnref $< -o $@

example.byte: example.ml
	ocamlc -o $@ $<

clean:
	rm -rf *.cm* *.o *.wasm *.byte *.wat *.js
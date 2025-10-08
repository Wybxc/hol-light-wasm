example.wat: example.wasm
	wasm-tool print $< > $@

example.wasm: example.byte
	wasm_of_ocaml --enable wasi $< -o $@

example.byte: example.ml
	ocamlc -o $@ -nostdlib -I ./runtime/stdlib $<

clean:
	rm -rf *.cm* *.o *.wasm *.byte *.wat
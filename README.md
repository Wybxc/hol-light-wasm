reference: https://github.com/sebmarkbage/ocamlrun-wasm

```shell
./configure --disable-native-compiler --disable-debugger --disable-ocamldoc --target=wasm32-wasi
./runtime/ocamlrun ./ocamlc -o ../build/example.byte ../example.ml -nostdlib -I ./stdlib

emconfigure ./configure CC="emcc" CFLAGS="-sPURE_WASI" --disable-native-compiler --disable-debugger --disable-ocamldoc
```

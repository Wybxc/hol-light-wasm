use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> Result<()> {
    let mut config = Config::default();
    config.wasm_gc(true);
    config.wasm_function_references(true);
    config.wasm_exceptions(true);

    let engine = Engine::new(&config)?;

    eprintln!("Loading WebAssembly module...");
    let wat = include_bytes!("../../main.wasm");
    let module = Module::new(&engine, wat)?;

    eprintln!("Loading WASI...");
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |t| t)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();
    let mut store = Store::new(&engine, wasi_ctx);
    let instance = linker.instantiate(&mut store, &module)?;

    let exports = instance
        .exports(&mut store)
        .map(|e| e.name().to_string())
        .collect::<Vec<_>>();
    for name in &exports {
        let export = instance.get_export(&mut store, name).unwrap();
        let ty = export.ty(&store);
        match ty {
            ExternType::Func(f) => println!("[Func] {name}: {f}"),
            ExternType::Global(g) => println!("[Global] {name}: {g:?}"),
            ExternType::Memory(m) => println!("[Memory] {name}: {m:?}"),
            ExternType::Table(t) => println!("[Table] {name}: {t:?}"),
            ExternType::Tag(t) => println!("[Tag] {name}: {t:?}"),
        }
    }

    eprintln!("Start WASM run!");
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start.call(&mut store, ())?;

    Ok(())
}

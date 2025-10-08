use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let mut config = Config::default();
    config.wasm_gc(true);
    config.wasm_function_references(true);
    config.wasm_exceptions(true);

    let engine = Engine::new(&config)?;

    let wat = include_bytes!("../../example.wat");
    let module = Module::new(&engine, wat)?;

    let mut linker = Linker::new(&engine);
    /*  (import "wasi_snapshot_preview1" "fd_write" (func (;0;) (type 30)))
    (import "wasi_snapshot_preview1" "random_get" (func (;1;) (type 27)))
    (import "wasi_snapshot_preview1" "proc_exit" (func (;2;) (type 25)))
    (import "wasi_snapshot_preview1" "fd_seek" (func (;3;) (type 31)))
    (import "wasi_snapshot_preview1" "fd_close" (func (;4;) (type 11))) */
    linker.func_wrap("wasi_snapshot_preview1", "fd_write", || {})?;

    let mut store = Store::new(&engine, 4);
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

    Ok(())
}

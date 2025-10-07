use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let mut config = Config::default();
    config.wasm_gc(true);
    config.wasm_function_references(true);
    config.wasm_exceptions(true);

    let engine = Engine::new(&config)?;

    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    let mut linker = Linker::new(&engine);
    linker.func_wrap(
        "host",
        "host_func",
        |caller: Caller<'_, u32>, param: i32| {
            println!("Got {} from WebAssembly", param);
            println!("my host state is: {}", caller.data());
        },
    )?;

    let mut store = Store::new(&engine, 4);
    let instance = linker.instantiate(&mut store, &module)?;

    let exports = instance
        .exports(&mut store)
        .map(|e| e.name().to_string())
        .collect::<Vec<_>>();
    for name in &exports {
        let export = instance.get_export(&mut store, name).unwrap();
        let ty = export.ty(&store);
        let ty = match ty {
            ExternType::Func(f) => format!("Func: {}", f),
            ExternType::Global(g) => format!("Global: {:?}", g),
            ExternType::Memory(m) => format!("Memory: {:?}", m),
            ExternType::Table(t) => format!("Table: {:?}", t),
            ExternType::Tag(t) => format!("Tag: {:?}", t),
        };
        println!("Export: {} - {}", name, ty);
    }

    Ok(())
}

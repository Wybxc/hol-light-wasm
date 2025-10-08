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
    linker.func_wrap("js_runtime", "print_endline", || println!())?;
    linker.func_wrap("js_runtime", "print_i32", |x: i32| print!("{}", x))?;
    linker.func_wrap("js_runtime", "print_f64", |x: f64| print!("{}", x))?;
    linker.func_wrap("js_runtime", "putchar", |x: i32| {
        print!("{}", x as u8 as char)
    })?;
    linker.func_wrap("js_runtime", "flush", || { /* flush output */ })?;
    linker.func_wrap("js_runtime", "atan2", |y: f64, x: f64| y.atan2(x))?;
    linker.func_wrap("js_runtime", "sin", |x: f64| x.sin())?;
    linker.func_wrap("js_runtime", "asin", |x: f64| x.asin())?;
    linker.func_wrap("js_runtime", "cos", |x: f64| x.cos())?;
    linker.func_wrap("js_runtime", "fmod", |x: f64, y: f64| x % y)?;

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

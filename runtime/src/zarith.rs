use std::ops::Shl;

use rug::{Complete, Integer, integer::Order};
use wasmtime::*;

// TODO: union(i32, Integer)?

#[derive(Debug, Clone)]
struct Z {
    data: Integer,
}

impl Z {
    const ORDER: Order = Order::Lsf;

    pub fn new(data: Integer) -> Self {
        Self { data }
    }

    pub fn from_wasm(mut store: impl AsContextMut, value: &EqRef) -> Result<Self> {
        if let Some(i31) = value.as_i31(&store)? {
            Ok(Self::new(Integer::from(i31.get_i32())))
        } else if let Some(arr) = value.as_array(&store)? {
            let digits = arr
                .elems(store.as_context_mut())?
                .map(|elem| {
                    elem.i64()
                        .map(|i| i as u64)
                        .ok_or_else(|| anyhow::anyhow!("expected i64"))
                })
                .collect::<Result<Vec<_>>>()?;

            let data = Integer::from_digits(&digits, Self::ORDER);
            Ok(Self::new(data))
        } else {
            anyhow::bail!("expected i31 or array, got {:?}", value.ty(store)?);
        }
    }

    pub fn into_wasm(self, mut store: impl AsContextMut) -> Result<Rooted<EqRef>> {
        let digits = self.data.to_digits::<u64>(Self::ORDER);
        let digits = digits
            .into_iter()
            .map(|d| Val::I64(d as i64))
            .collect::<Vec<_>>();
        let array_ty = ArrayType::new(
            store.as_context().engine(),
            FieldType::new(Mutability::Const, ValType::I64.into()),
        );
        let allocator = ArrayRefPre::new(&mut store, array_ty);
        let array = ArrayRef::new_fixed(store, &allocator, &digits)?;
        Ok(array.to_eqref())
    }

    pub fn inner(&self) -> &Integer {
        &self.data
    }
}

fn int(mut store: impl AsContextMut, x: i32) -> Result<Rooted<EqRef>> {
    let i31 = I31::new_i32(x).expect("i31");
    AnyRef::from_i31(&mut store, i31).unwrap_eqref(&mut store)
}

/// ```ocaml
/// val sign : Z -> int
/// ```
fn z_sign<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let s = x.inner().signum_ref().complete().to_i32().unwrap();
    int(caller, s)
}

/// ```ocaml
/// val format : string -> Z -> string
/// ```
fn z_format<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("format: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val equal : Z -> Z -> bool
/// ```
fn z_equal<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("equal: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val sub : Z -> Z -> Z
/// ```
fn z_sub<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("sub: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val shift_left : Z -> int -> Z
/// ```
fn z_shift_left<T>(
    mut caller: Caller<T>,
    x: Rooted<EqRef>,
    y: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = y.as_i31(&caller)?.unwrap().get_u32();
    let result = x.inner().shl(y).complete();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val rem : Z -> Z -> Z
/// ```
fn z_rem<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("rem: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val gcd : Z -> Z -> Z
/// ```
fn z_gcd<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("gcd: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val fdiv : Z -> Z -> Z
/// ```
fn z_fdiv<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("fdiv: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val divexact : Z -> Z -> Z
/// ```
fn z_divexact<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("divexact: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val div : Z -> Z -> Z
/// ```
fn z_div<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("div: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val compare : Z -> Z -> int
/// ```
fn z_compare<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("compare: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val add : Z -> Z -> Z
/// ```
fn z_add<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("add: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val abs : Z -> Z
/// ```
fn z_abs<T>(caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    println!("abs: {ty_x}");
    todo!()
}

/// ```ocaml
/// val to_int : Z -> int
/// ```
fn z_to_int<T>(caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    println!("to_int: {ty_x}");
    todo!()
}

/// ```ocaml
/// val succ : Z -> Z
/// ```
fn z_succ<T>(caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    println!("succ: {ty_x}");
    todo!()
}

/// ```ocaml
/// val pred : Z -> Z
/// ```
fn z_pred<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let result = x.inner() - 1i32;
    let result = result.complete();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val neg : Z -> Z
/// ```
fn z_neg<T>(caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    println!("neg: {ty_x}");
    todo!()
}

/// ```ocaml
/// val mul_overflows : int -> int -> bool
/// ```
fn z_mul_overflows<T>(
    caller: Caller<T>,
    x: Rooted<EqRef>,
    y: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("mul_overflows: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val mul : Z -> Z -> Z
/// ```
fn z_mul<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("mul: {ty_x}, {ty_y}");
    todo!()
}

/// ```ocaml
/// val init : unit -> unit
/// ```
fn z_init(unit: Rooted<EqRef>) -> Rooted<EqRef> {
    unit
}

/// ```ocaml
/// val cdiv : Z -> Z -> Z
/// ```
fn z_cdiv<T>(caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let ty_x = x.ty(&caller)?;
    let ty_y = y.ty(&caller)?;
    println!("cdiv: {ty_x}, {ty_y}");
    todo!()
}

pub fn add_to_linker<T>(linker: &mut Linker<T>) -> anyhow::Result<()>
where
    T: 'static,
{
    linker.func_wrap("env", "ml_z_sign", z_sign)?;
    linker.func_wrap("env", "ml_z_format", z_format)?;
    linker.func_wrap("env", "ml_z_equal", z_equal)?;
    linker.func_wrap("env", "ml_z_sub", z_sub)?;
    linker.func_wrap("env", "ml_z_shift_left", z_shift_left)?;
    linker.func_wrap("env", "ml_z_rem", z_rem)?;
    linker.func_wrap("env", "ml_z_gcd", z_gcd)?;
    linker.func_wrap("env", "ml_z_fdiv", z_fdiv)?;
    linker.func_wrap("env", "ml_z_divexact", z_divexact)?;
    linker.func_wrap("env", "ml_z_div", z_div)?;
    linker.func_wrap("env", "ml_z_compare", z_compare)?;
    linker.func_wrap("env", "ml_z_add", z_add)?;
    linker.func_wrap("env", "ml_z_abs", z_abs)?;
    linker.func_wrap("env", "ml_z_to_int", z_to_int)?;
    linker.func_wrap("env", "ml_z_succ", z_succ)?;
    linker.func_wrap("env", "ml_z_pred", z_pred)?;
    linker.func_wrap("env", "ml_z_neg", z_neg)?;
    linker.func_wrap("env", "ml_z_mul_overflows", z_mul_overflows)?;
    linker.func_wrap("env", "ml_z_mul", z_mul)?;
    linker.func_wrap("env", "ml_z_init", z_init)?;
    linker.func_wrap("env", "ml_z_cdiv", z_cdiv)?;
    Ok(())
}

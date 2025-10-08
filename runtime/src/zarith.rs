use rug::{Complete, Integer, integer::Order, ops::DivRounding};
use wasmtime::*;

/// Zarith integers, known as `Z.t` in OCaml.
///
/// It's a semi-opaque type: they are opaque to user-level OCaml code, but the
/// runtime system internally represents small integers as `i31`.
/// Large integers remain opaque both to user code and the runtime.
/// In our implementation, these large integers are represented as arrays of u64
/// digits, stored in little-endian order.
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
        // If the integer fits in an i31, use that representation.
        if let Some(i31) = self.data.to_i32().and_then(I31::new_i32) {
            return AnyRef::from_i31(&mut store, i31).unwrap_eqref(&mut store);
        }

        // Otherwise use an array of u64 digits.
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

    pub fn into_inner(self) -> Integer {
        self.data
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
    let s = x.into_inner().signum().to_i32().unwrap();
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
fn z_equal<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.inner().eq(y.inner());
    int(&mut caller, result as i32)
}

/// ```ocaml
/// val sub : Z -> Z -> Z
/// ```
fn z_sub<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner() - y.into_inner();
    Z::new(result).into_wasm(caller)
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
    let result = x.into_inner() << y;
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val rem : Z -> Z -> Z
/// ```
fn z_rem<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner() % y.into_inner();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val gcd : Z -> Z -> Z
/// ```
fn z_gcd<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner().gcd(y.inner());
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val fdiv : Z -> Z -> Z
/// ```
fn z_fdiv<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner().div_floor(y.inner());
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val divexact : Z -> Z -> Z
/// ```
fn z_divexact<T>(
    mut caller: Caller<T>,
    x: Rooted<EqRef>,
    y: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner().div_exact(y.inner());
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val div : Z -> Z -> Z
/// ```
fn z_div<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner() / y.into_inner();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val compare : Z -> Z -> int
/// ```
fn z_compare<T>(
    mut caller: Caller<T>,
    x: Rooted<EqRef>,
    y: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.inner().cmp(y.inner());
    int(&mut caller, result as i32)
}

/// ```ocaml
/// val add : Z -> Z -> Z
/// ```
fn z_add<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner() + y.into_inner();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val abs : Z -> Z
/// ```
fn z_abs<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let result = x.into_inner().abs();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val to_int : Z -> int
/// ```
fn z_to_int<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let result = x
        .inner()
        .to_i32()
        .ok_or_else(|| anyhow::anyhow!("overflow"))?;
    int(&mut caller, result)
}

/// ```ocaml
/// val succ : Z -> Z
/// ```
fn z_succ<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let result = x.into_inner() + 1;
    Z::new(result).into_wasm(caller)
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
fn z_neg<T>(mut caller: Caller<T>, x: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let result = -x.into_inner();
    Z::new(result).into_wasm(caller)
}

/// ```ocaml
/// val mul_overflows : int -> int -> bool
/// ```
fn z_mul_overflows<T>(
    mut caller: Caller<T>,
    x: Rooted<EqRef>,
    y: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    let x = x.as_i31(&caller)?.unwrap().get_i32();
    let y = y.as_i31(&caller)?.unwrap().get_i32();
    let (_result, overflowed) = x.overflowing_mul(y);
    int(&mut caller, overflowed as i32)
}

/// ```ocaml
/// val mul : Z -> Z -> Z
/// ```
fn z_mul<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner() * y.into_inner();
    Z::new(result).into_wasm(caller)
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
fn z_cdiv<T>(mut caller: Caller<T>, x: Rooted<EqRef>, y: Rooted<EqRef>) -> Result<Rooted<EqRef>> {
    let x = Z::from_wasm(&mut caller, &x)?;
    let y = Z::from_wasm(&mut caller, &y)?;
    let result = x.into_inner().div_ceil(y.inner());
    Z::new(result).into_wasm(caller)
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

use wasmtime::*;

/// ```ocaml
/// val sign : Z -> int
/// ```
fn z_sign(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val format : string -> Z -> string
/// ```
fn z_format(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val equal : Z -> Z -> bool
/// ```
fn z_equal(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val sub : Z -> Z -> Z
/// ```
fn z_sub(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val shift_left : Z -> int -> Z
/// ```
fn z_shift_left(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val rem : Z -> Z -> Z
/// ```
fn z_rem(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val gcd : Z -> Z -> Z
/// ```
fn z_gcd(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val fdiv : Z -> Z -> Z
/// ```
fn z_fdiv(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val divexact : Z -> Z -> Z
/// ```
fn z_divexact(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val div : Z -> Z -> Z
/// ```
fn z_div(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val compare : Z -> Z -> int
/// ```
fn z_compare(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val add : Z -> Z -> Z
/// ```
fn z_add(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val abs : Z -> Z
/// ```
fn z_abs(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val to_int : Z -> int
/// ```
fn z_to_int(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val succ : Z -> Z
/// ```
fn z_succ(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val pred : Z -> Z
/// ```
fn z_pred(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val neg : Z -> Z
/// ```
fn z_neg(_x: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val mul_overflows : int -> int -> bool
/// ```
fn z_mul_overflows(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
    todo!()
}

/// ```ocaml
/// val mul : Z -> Z -> Z
/// ```
fn z_mul(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
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
fn z_cdiv(_x: Rooted<EqRef>, _y: Rooted<EqRef>) -> Rooted<EqRef> {
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

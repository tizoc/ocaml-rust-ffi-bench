use crate::{RawValue, Val, GCResult1, Tag_val, Closure_tag};

extern "C" {
    fn caml_named_value(name: *const cty::c_char) -> *const RawValue;

    fn caml_callback(closure: RawValue, arg1: RawValue) -> RawValue;
    fn caml_callback2(closure: RawValue, arg1: RawValue, arg2: RawValue) -> RawValue;
    fn caml_callback3(
        closure: RawValue,
        arg1: RawValue,
        arg2: RawValue,
        arg3: RawValue,
    ) -> RawValue;
    fn caml_callbackN(closure: RawValue, narg: usize, args: *mut RawValue) -> RawValue;

    fn caml_callback_exn(closure: RawValue, arg1: RawValue) -> RawValue;
    fn caml_callback2_exn(closure: RawValue, arg1: RawValue, arg2: RawValue) -> RawValue;
    fn caml_callback3_exn(
        closure: RawValue,
        arg1: RawValue,
        arg2: RawValue,
        arg3: RawValue,
    ) -> RawValue;
    fn caml_callbackN_exn(closure: RawValue, narg: usize, args: *mut RawValue) -> RawValue;
}

#[derive(Debug)]
pub enum CamlError {
    Exception(RawValue),
}

#[derive(Debug)]
pub enum Error {
    Caml(CamlError),
}

// #define Is_exception_result(v) (((v) & 3) == 2)
pub const fn is_exception_result(val: RawValue) -> bool {
    val & 3 == 2
}

// #define Extract_exception(v) ((v) & ~3)
pub const fn extract_exception(val: RawValue) -> RawValue {
    val & !3
}

pub struct OCamlFun(RawValue);

fn get_named(name: &str) -> Option<RawValue> {
    unsafe {
        let s = match std::ffi::CString::new(name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let named = caml_named_value(s.as_ptr());
        if named.is_null() {
            return None;
        }

        if Tag_val(*named) != Closure_tag {
            return None;
        }

        Some(named as RawValue)
    }
}

pub fn ocaml_named_function(name: &str) -> Val<'static, OCamlFun> {
    let func_ref = get_named(name).expect("Failed to obtain named function");
    Val {
        _marker: Default::default(),
        raw: func_ref,
    }
}

impl<'a> Val<'a, OCamlFun> {
    pub fn call<'gc, T, R>(self, arg: Val<'gc, T>) -> Result<GCResult1<R>, Error> {
        let fptr = self.raw as *const RawValue;
        let v = unsafe { caml_callback_exn(*fptr, arg.raw) };

        if is_exception_result(v) {
            let ex = extract_exception(v);
            Err(Error::Caml(CamlError::Exception(ex)))
        } else {
            let gv = GCResult1::of(v);
            Ok(gv)
        }
    }

    pub fn call2<'gc, T, U, R>(self, arg1: Val<'gc, T>, arg2: Val<'gc, U>) -> Result<GCResult1<R>, Error> {
        let fptr = self.raw as *const RawValue;
        let v = unsafe { caml_callback2_exn(*fptr, arg1.raw, arg2.raw) };

        if is_exception_result(v) {
            let ex = extract_exception(v);
            Err(Error::Caml(CamlError::Exception(ex)))
        } else {
            let gv = GCResult1::of(v);
            Ok(gv)
        }
    }

}

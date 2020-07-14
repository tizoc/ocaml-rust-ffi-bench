use crate::{Closure_tag, GCResult1, RawValue, Tag_val, Val};

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

#[derive(Copy, Clone)]
pub struct OCamlFun(*const RawValue);

unsafe impl Sync for OCamlFun {}

fn get_named(name: &str) -> Option<*const RawValue> {
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

        Some(named)
    }
}

impl OCamlFun {
    pub fn named(name: &str) -> Option<OCamlFun> {
        get_named(name).map(OCamlFun)
    }

    pub fn call<T, R>(self, arg: Val<T>) -> Result<GCResult1<R>, Error> {
        let v = unsafe { caml_callback_exn(*self.0, arg.raw) };

        if is_exception_result(v) {
            let ex = extract_exception(v);
            Err(Error::Caml(CamlError::Exception(ex)))
        } else {
            let gv = GCResult1::of(v);
            Ok(gv)
        }
    }

    pub fn call2<T, U, R>(self, arg1: Val<T>, arg2: Val<U>) -> Result<GCResult1<R>, Error> {
        let v = unsafe { caml_callback2_exn(*self.0, arg1.raw, arg2.raw) };

        if is_exception_result(v) {
            let ex = extract_exception(v);
            Err(Error::Caml(CamlError::Exception(ex)))
        } else {
            let gv = GCResult1::of(v);
            Ok(gv)
        }
    }
}

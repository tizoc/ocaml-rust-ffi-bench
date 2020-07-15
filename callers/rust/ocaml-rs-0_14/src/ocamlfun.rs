use ocaml::ToValue;
use ocaml::Value;

#[derive(Copy, Clone)]
pub struct OCamlFun(*const Value);

unsafe impl Sync for OCamlFun {}

// This is a safer version of references to callable functions.
// Unlike the default, this one keeps a reference to the location with
// the pointer to the closure, which gets dereferenced upon invocation.
// This guarantees that a valid value is being used even if the OCaml
// GC decides to move the closure.
// Another advantage is that the closure tag check is done at lookup time
// and not every time the closure is going to be applied.
impl OCamlFun {
    pub fn named(name: &str) -> Option<OCamlFun> {
        unsafe {
            let s = match std::ffi::CString::new(name) {
                Ok(s) => s,
                Err(_) => return None,
            };
            let named = ocaml::sys::caml_named_value(s.as_ptr());
            if named.is_null() {
                return None;
            }

            let named = named as *const Value;
            if (*named).tag() != ocaml::Tag::CLOSURE {
                return None;
            }

            Some(OCamlFun(named))
        }
    }

    pub fn call<A: ToValue>(self, arg: A) -> Result<Value, ocaml::Error> {
        let f = unsafe { *(self.0) };
        let mut v = ocaml::frame!((res) {
            res = unsafe { Value(ocaml::sys::caml_callback_exn(f.0, arg.to_value().0)) };
            res
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(ocaml::CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }
}

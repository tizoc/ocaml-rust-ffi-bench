use ocaml::{CamlError, Error, ToValue, Value};

/// `OCamlBytes` wraps an OCaml `bytes` without converting it to Rust
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct OCamlBytes(Value);

unsafe impl ToValue for OCamlBytes {
    fn to_value(self) -> Value {
        self.0
    }
}

impl OCamlBytes {
    pub fn alloc(n: usize) -> OCamlBytes {
        let x = ocaml::frame!((x) {
            x = unsafe { Value(ocaml::sys::caml_alloc_string(n)) };
            x
        });
        OCamlBytes(x)
    }

    pub fn keep(&mut self) {
        unsafe {
            let ptr = (&mut self.0) as *mut Value as *mut isize;
            ocaml::sys::caml_register_global_root(ptr);
        }
    }

    pub fn dispose(&mut self) {
        unsafe {
            let ptr = (&mut self.0) as *mut Value as *mut isize;
            ocaml::sys::caml_remove_global_root(ptr);
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ocaml::sys::caml_string_length((self.0).0) }
    }

    pub fn set_byte(&mut self, i: usize, n: u8) -> Result<(), Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        unsafe {
            self.set_byte_unchecked(i, n);
        };

        Ok(())
    }

    #[inline]
    pub unsafe fn set_byte_unchecked(&mut self, i: usize, n: u8) {
        let ptr = ((self.0).0 as *mut u8).add(i);
        *ptr = n;
    }

    pub fn get_byte(self, i: usize) -> Result<u8, Error> {
        if i >= self.len() {
            return Err(CamlError::ArrayBoundError.into());
        }

        Ok(unsafe { self.get_byte_unchecked(i) })
    }

    #[inline]
    pub unsafe fn get_byte_unchecked(self, i: usize) -> u8 {
        *((self.0).0 as *mut u8).add(i)
    }
}

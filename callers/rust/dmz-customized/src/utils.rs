use dmz::RawValue;

pub fn ocaml_int_of_i32(num: i32) -> RawValue {
    return ((num as i64) << 1) | 1;
}

pub fn i32_of_ocaml_int(num: RawValue) -> i32 {
    return (num as i32) >> 1;
}

pub fn string_of_ocaml(bytes: RawValue) -> String {
    let len = unsafe { dmz::caml_string_length(bytes) };
    let ptr = bytes as *const u8;
    unsafe {
        let slice = ::std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice).expect("Invalid UTF-8").into()
    }
}

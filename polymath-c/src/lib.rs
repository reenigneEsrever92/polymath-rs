use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn to_math_ml(input: *const u8) -> *const u8 {
    if !input.is_null() {
        let c_string = CStr::from_ptr(input as *const i8).to_str();

        if let Ok(c_string) = c_string {
            let mathml = polymath_rs::to_math_ml(c_string);

            if let Ok(c_string) = CString::new(mathml.as_str()) {
                c_string.into_raw() as *const u8
            } else {
                [0].as_ptr() as *const u8
            }
        } else {
            [0].as_ptr() as *const u8
        }
    } else {
        [0].as_ptr() as *const u8
    }
}

#[cfg(test)]
mod test {
    use std::ffi::CString;

    use crate::to_math_ml;

    #[test]
    fn test_expression() {
        unsafe {
            let mathml = CString::from_raw(to_math_ml(
                CString::new("i  1 sum_{i 1}^{n} i^{3}")
                    .unwrap()
                    .as_bytes()
                    .as_ptr(),
            ) as *mut i8);

            println!("{:?}", mathml.to_str().unwrap());
            println!("{:?}", String::from_utf8(vec![44, 91, 0, 4]).unwrap());
        }
    }
}

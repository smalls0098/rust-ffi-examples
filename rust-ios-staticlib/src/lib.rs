use std::ffi::{CStr, CString};
use std::os::raw::{c_char};

// 调用核心框架
use rust_core::get_core_info;

#[no_mangle]
pub extern fn get_version() -> *mut c_char {
    CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw()
}

#[repr(C)]
pub struct TokenInfo {
    pub token: *mut c_char,
    pub core: *mut c_char,
}

#[no_mangle]
pub extern fn get_token(token: *mut c_char) -> TokenInfo {
    let info = TokenInfo {
        token: CString::new(char2str(token)).unwrap().into_raw(),
        core: CString::new(get_core_info()).unwrap().into_raw(),
    };
    return info
}

#[no_mangle]
pub extern fn rust_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        CString::from_raw(s)
    };
}

fn char2str(data: *mut c_char) -> String {
    let data_c_str = unsafe { CStr::from_ptr(data) };
    let data_str = match data_c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    return data_str.to_string();
}

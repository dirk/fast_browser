extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::mem;

pub struct Browser {
    // Flags
    is_chrome: bool,
    is_edge:   bool,
    is_opera:  bool,
}

impl Browser {
    fn parse(ua: &str) -> Browser {
        let is_opera  = Browser::is_opera(ua);
        let is_edge   = Browser::is_edge(ua);
        let is_chrome = (ua.contains("Chrome") || ua.contains("CriOS")) && !is_opera && !is_edge;

        Browser {
            is_chrome: is_chrome,
            is_edge:   is_edge,
            is_opera:  is_opera,
        }
    }

    fn borrow_c<'a>(browser: *mut Browser) -> &'a Browser {
        unsafe { mem::transmute(browser) }
    }

    fn is_opera(ua: &str) -> bool {
        ua.contains("Opera") || ua.contains("OPR")
    }

    fn is_edge(ua: &str) -> bool {
        ua.contains("Edge/") || ua.contains("Trident/8")
    }
}

#[no_mangle]
pub extern "C" fn parse_browser(cstring: *const c_char) -> *const Browser {
    let string  = unsafe { CStr::from_ptr(cstring) }.to_str().unwrap();
    let browser = Browser::parse(string);

    Box::into_raw(Box::new(browser))
}

#[no_mangle]
pub extern "C" fn free_browser(browser: *mut Browser) {
    unsafe {
        drop(Box::from_raw(browser));
    }
}

#[no_mangle]
pub extern "C" fn is_opera(browser: *mut Browser) -> bool {
    Browser::borrow_c(browser).is_opera
}

#[no_mangle]
pub extern "C" fn is_chrome(browser: *mut Browser) -> bool {
    Browser::borrow_c(browser).is_chrome
}

#[no_mangle]
pub extern "C" fn is_edge(browser: *mut Browser) -> bool {
    Browser::borrow_c(browser).is_edge
}

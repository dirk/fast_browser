#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate regex;

mod browser;
mod user_agent;

use libc::c_char;
use std::ffi::{CStr, CString};
use self::browser::BrowserFamily;
use self::user_agent::UserAgent;

#[no_mangle]
pub extern fn parse_user_agent(cstring: *const c_char) -> *const UserAgent {
    let string  = unsafe { CStr::from_ptr(cstring) }.to_str().unwrap();
    let browser = UserAgent::parse(string);

    Box::into_raw(Box::new(browser))
}

/// Take back ownership of an externally-owned `Browser` and destructively deallocate it.
#[no_mangle]
pub extern fn free_user_agent(ua: *mut UserAgent) {
    drop(unsafe { Box::from_raw(ua) })
}

macro_rules! is_family {
    ($function:ident, $family:path) => {
        #[no_mangle]
        pub extern fn $function(ua: *const UserAgent) -> bool {
            UserAgent::borrow_from_c(ua).browser.family == $family
        }
    };
}

is_family!(is_chrome,  BrowserFamily::Chrome);
is_family!(is_edge,    BrowserFamily::Edge);
is_family!(is_firefox, BrowserFamily::Firefox);
is_family!(is_opera,   BrowserFamily::Opera);
is_family!(is_safari,  BrowserFamily::Safari);

#[no_mangle]
pub extern fn get_browser_major_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.major_version
}

#[no_mangle]
pub extern fn get_browser_minor_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.minor_version
}

/// Returns the user agent's browser family name as a heap-allocated `CString`
#[no_mangle]
pub extern fn get_browser_family(ua: *const UserAgent) -> *mut c_char {
    let ua = UserAgent::borrow_from_c(ua);

    let family = match ua.browser.family {
        BrowserFamily::Chrome       => "Chrome",
        BrowserFamily::Edge         => "Edge",
        BrowserFamily::Firefox      => "Firefox",
        BrowserFamily::Opera        => "Opera",
        BrowserFamily::Safari       => "Safari",
        BrowserFamily::MobileSafari => "Mobile Safari",
        BrowserFamily::Other        => "Other",
    };

    CString::new(family).unwrap().into_raw()
}

/// Free a `CString` pointer owned by Rust
#[no_mangle]
pub extern fn free_string(string: *mut c_char) {
    drop(unsafe { CString::from_raw(string) })
}

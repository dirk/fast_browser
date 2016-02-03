#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate regex;

use libc::c_char;
use std::ffi::{CStr, CString};

mod bot;
mod browser;
mod user_agent;
mod util;

use browser::BrowserFamily;
use user_agent::UserAgent;

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
            if let Some(ref browser) = UserAgent::borrow_from_c(ua).browser {
                browser.family == $family
            } else {
                false
            }
        }
    };
}

is_family!(is_chrome,  BrowserFamily::Chrome);
is_family!(is_edge,    BrowserFamily::Edge);
is_family!(is_firefox, BrowserFamily::Firefox);
is_family!(is_opera,   BrowserFamily::Opera);
is_family!(is_safari,  BrowserFamily::Safari);

#[no_mangle]
pub extern fn is_mobile(ua: *const UserAgent) -> bool {
    let ua = UserAgent::borrow_from_c(ua);

    match ua.browser {
        Some(ref b) => b.family.is_mobile(),
        _ => false,
    }
}

#[no_mangle]
pub extern fn is_bot(ua: *const UserAgent) -> bool {
    let ua = UserAgent::borrow_from_c(ua);

    match ua.bot {
        Some(_) => true,
        _ => false,
    }
}

#[no_mangle]
pub extern fn get_browser_major_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.clone().map_or(0, |b| b.major_version)
}

#[no_mangle]
pub extern fn get_browser_minor_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.clone().map_or(0, |b| b.minor_version)
}

/// Returns the user agent's browser family name as a heap-allocated `CString`
#[no_mangle]
pub extern fn get_browser_family(ua: *const UserAgent) -> *mut c_char {
    let family = UserAgent::borrow_from_c(ua).browser.clone()
        .map_or("Other".to_owned(), |browser| browser.to_string());

    CString::new(family).unwrap().into_raw()
}

#[no_mangle]
pub extern fn get_bot_name(ua: *const UserAgent) -> *mut c_char {
    let name = UserAgent::borrow_from_c(ua).bot.clone()
        .map_or("Other".to_owned(), |bot| bot.name.to_string());

    CString::new(name).unwrap().into_raw()
}

/// Returns the original user agent that was parsed as a `CString` (must free later)
#[no_mangle]
pub extern fn get_user_agent(ua: *const UserAgent) -> *mut c_char {
    let ref ua = UserAgent::borrow_from_c(ua);

    CString::new(ua.source.clone()).unwrap().into_raw()
}

/// Free a `CString` pointer owned by Rust
#[no_mangle]
pub extern fn free_string(string: *mut c_char) {
    drop(unsafe { CString::from_raw(string) })
}

const VERSION: &'static str = "0.1.1";

#[no_mangle]
pub extern fn get_version() -> *const c_char {
    CString::new(VERSION).unwrap().into_raw()
}

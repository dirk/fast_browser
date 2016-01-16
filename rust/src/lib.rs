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

#[cfg(test)]
mod tests {
    use super::browser::Browser;

    #[test]
    fn test_match_firefox() {
        let did_match = Browser::match_firefox("Firefox/1.2");
        assert_eq!(did_match, Some((1, 2)));

        let didnt_match = Browser::match_firefox("NotFirefox/x.y");
        assert_eq!(didnt_match, None)
    }

    #[test]
    fn test_match_safari() {
        let version7 = Browser::match_safari("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A");
        assert_eq!(version7, Some((7, 0)));

        let version5 = Browser::match_safari("Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; en-us) AppleWebKit/534.1+ (KHTML, like Gecko) Version/5.0 Safari/533.16");
        assert_eq!(version5, Some((5, 0)));

        let mobile_version6 = Browser::match_safari("Mozilla/5.0 (iPad; CPU OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5355d Safari/8536.25");
        assert_eq!(mobile_version6, Some((6, 0)))
    }
}

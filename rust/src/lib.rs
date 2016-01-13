extern crate libc;
extern crate regex;

use libc::c_char;
use std::ffi::CStr;
use std::mem;
use std::str::FromStr;
use regex::Regex;

pub struct UserAgent {
    browser: Browser,
}

impl UserAgent {
    pub fn parse(ua: &str) -> UserAgent {
        let browser = Browser::parse(ua);

        UserAgent {
            browser: browser,
        }
    }

    /// Take an externally-owned `Browser` and non-destructively borrow a reference to it.
    ///
    /// **Note**: This will *not* deallocate the instance passed in. So it is safe to call this
    /// over and over again.
    fn borrow_from_c<'a>(ua: *const UserAgent) -> &'a UserAgent {
        unsafe { mem::transmute(ua) }
    }
}

pub struct Browser {
    family: BrowserFamily,
    major_version: i8,
    minor_version: i8,
}

#[derive(PartialEq)]
enum BrowserFamily {
    Chrome,
    Edge,
    Firefox,
    Opera,
    Other,
}

impl Browser {
    pub fn parse(ua: &str) -> Browser {
        let mut major_version = 0;
        let mut minor_version = 0;

        let is_opera                      = Browser::is_opera(ua);
        let is_edge                       = Browser::is_edge(ua);
        let is_chrome                     = ua.contains("Chrome") && !is_opera && !is_edge;
        let (is_firefox, firefox_version) = Browser::match_firefox(ua);

        let family =
            if is_chrome {
                let (major, minor) = Browser::match_chrome(ua);
                major_version = major;
                minor_version = minor;
                BrowserFamily::Chrome

            } else if is_edge {
                BrowserFamily::Edge

            } else if is_opera {
                BrowserFamily::Opera

            } else if is_firefox {
                major_version = firefox_version.unwrap().0;
                minor_version = firefox_version.unwrap().1;
                BrowserFamily::Firefox

            } else {
                BrowserFamily::Other
            };

        Browser {
            family: family,
            major_version: major_version,
            minor_version: minor_version,
        }
    }

    fn is_opera(ua: &str) -> bool {
        ua.contains("Opera") || ua.contains("OPR")
    }

    fn is_edge(ua: &str) -> bool {
        ua.contains("Edge/") || ua.contains("Trident/8")
    }

    /// Search for the Firefox componenet in the user agent and parse out the version if present.
    pub fn match_firefox(ua: &str) -> (bool, Option<(i8, i8)>) {
        let re = Regex::new(r"Firefox/(\d+)\.(\d+)").unwrap();

        match re.captures(ua) {
            Some(captures) => {
                let major_version = i8::from_str(&captures[1]).unwrap();
                let minor_version = i8::from_str(&captures[2]).unwrap();
                (true, Some((major_version, minor_version)))
            },
            None => (false, None)
        }
    }

    pub fn match_chrome(ua: &str) -> (i8, i8) {
        let re            = Regex::new(r"(Chromium|Chrome)/(\d+)\.(\d+)").unwrap();
        let captures      = re.captures(ua).unwrap();
        let major_version = i8::from_str(&captures[2]).unwrap();
        let minor_version = i8::from_str(&captures[3]).unwrap();
        (major_version, minor_version)
    }
}

#[no_mangle]
pub extern "C" fn parse_user_agent(cstring: *const c_char) -> *const UserAgent {
    let string  = unsafe { CStr::from_ptr(cstring) }.to_str().unwrap();
    let browser = UserAgent::parse(string);

    Box::into_raw(Box::new(browser))
}

/// Take back ownership of an externally-owned `Browser` and destructively deallocate it.
#[no_mangle]
pub extern "C" fn free_user_agent(ua: *mut UserAgent) {
    drop(unsafe { Box::from_raw(ua) })
}

#[no_mangle]
pub extern "C" fn is_chrome(ua: *const UserAgent) -> bool {
    UserAgent::borrow_from_c(ua).browser.family == BrowserFamily::Chrome
}

#[no_mangle]
pub extern "C" fn is_edge(ua: *const UserAgent) -> bool {
    UserAgent::borrow_from_c(ua).browser.family == BrowserFamily::Edge
}

#[no_mangle]
pub extern "C" fn is_firefox(ua: *const UserAgent) -> bool {
    UserAgent::borrow_from_c(ua).browser.family == BrowserFamily::Firefox
}

#[no_mangle]
pub extern "C" fn is_opera(ua: *const UserAgent) -> bool {
    UserAgent::borrow_from_c(ua).browser.family == BrowserFamily::Opera
}

#[no_mangle]
pub extern "C" fn get_browser_major_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.major_version
}

#[no_mangle]
pub extern "C" fn get_browser_minor_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.minor_version
}

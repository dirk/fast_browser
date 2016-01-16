extern crate libc;
extern crate regex;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem;
use std::str::FromStr;
use regex::{Captures, Regex};

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
    Safari,
    MobileSafari,
    Other,
}

impl Browser {
    pub fn parse(ua: &str) -> Browser {
        let mut versions: (i8, i8) = (0, 0);

        let is_opera        = Browser::is_opera(ua);
        let matched_edge    = Browser::match_edge(ua);
        let matched_chrome  = if !is_opera && !matched_edge.is_some() { Browser::match_chrome(ua) } else { None };
        let matched_firefox = Browser::match_firefox(ua);
        let matched_safari  = if ua.contains("Safari") { Browser::match_safari(ua) } else { None };

        let family =
            if is_opera {
                BrowserFamily::Opera

            } else if let Some(v) = matched_edge {
                versions = v;
                BrowserFamily::Edge

            } else if let Some(v) = matched_chrome {
                versions = v;
                BrowserFamily::Chrome

            } else if let Some(v) = matched_firefox {
                versions = v;
                BrowserFamily::Firefox

            } else if let Some(v) = matched_safari {
                versions = v;

                if ua.contains("Mobile/") {
                    BrowserFamily::MobileSafari
                } else {
                    BrowserFamily::Safari
                }

            } else {
                BrowserFamily::Other
            };

        Browser {
            family: family,
            major_version: versions.0,
            minor_version: versions.1,
        }
    }

    fn is_opera(ua: &str) -> bool {
        ua.contains("Opera") || ua.contains("OPR")
    }

    /// Takes the first two capture groups from a regex result and turns them into a version
    /// integer 2-tuple
    fn map_first_captures(captures: Captures) -> (i8, i8) {
        let major_version = i8::from_str(&captures[1]).unwrap();
        let minor_version = i8::from_str(&captures[2]).unwrap();
        (major_version, minor_version)
    }

    /// Take a regex and attempt to match it to the browser. The regex must include two capture
    /// groups that capture the version of the matched browser.
    fn match_versions(ua: &str, regex: &str) -> Option<(i8, i8)> {
        Regex::new(regex)
            .unwrap()
            .captures(ua)
            .map(Browser::map_first_captures)
    }

    pub fn match_edge(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, r"Edge/(\d+)\.(\d+)")
    }

    /// Search for the Firefox componenet in the user agent and parse out the version if present.
    pub fn match_firefox(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, r"Firefox/(\d+)\.(\d+)")
    }

    pub fn match_chrome(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, r"(?:Chromium|Chrome)/(\d+)\.(\d+)")
    }

    pub fn match_safari(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, r"Version/(\d+)\.(\d+)(?:\.\d+)?(?: Mobile/\w+)? Safari")
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
pub extern "C" fn is_safari(ua: *const UserAgent) -> bool {
    UserAgent::borrow_from_c(ua).browser.family == BrowserFamily::Safari
}

#[no_mangle]
pub extern "C" fn get_browser_major_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.major_version
}

#[no_mangle]
pub extern "C" fn get_browser_minor_version(ua: *const UserAgent) -> i8 {
    UserAgent::borrow_from_c(ua).browser.minor_version
}

/// Returns the user agent's browser family name as a heap-allocated `CString`
#[no_mangle]
pub extern "C" fn get_browser_family(ua: *const UserAgent) -> *mut c_char {
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
pub extern "C" fn free_string(string: *mut c_char) {
    drop(unsafe { CString::from_raw(string) })
}

#[cfg(test)]
mod tests {
    use super::{Browser};

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

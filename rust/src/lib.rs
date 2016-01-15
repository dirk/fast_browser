extern crate libc;
extern crate regex;

use libc::c_char;
use std::ffi::{CStr, CString};
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
    Safari,
    Other,
}

impl Browser {
    pub fn parse(ua: &str) -> Browser {
        let mut major_version = 0;
        let mut minor_version = 0;

        let is_opera        = Browser::is_opera(ua);
        let is_edge         = Browser::is_edge(ua);
        let matched_chrome  = if !is_opera && !is_edge { Browser::match_chrome(ua) } else { None };
        let matched_firefox = Browser::match_firefox(ua);
        let matched_safari  = if ua.contains("Safari") { Browser::match_safari(ua) } else { None };

        let family =
            if is_edge {
                BrowserFamily::Edge

            } else if is_opera {
                BrowserFamily::Opera

            } else if let Some((major, minor)) = matched_chrome {
                major_version = major;
                minor_version = minor;
                BrowserFamily::Chrome

            } else if let Some((major, minor)) = matched_firefox {
                major_version = major;
                minor_version = minor;
                BrowserFamily::Firefox

            } else if let Some((major, minor)) = matched_safari {
                major_version = major;
                minor_version = minor;
                BrowserFamily::Safari

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
    pub fn match_firefox(ua: &str) -> Option<(i8, i8)> {
        let re = Regex::new(r"Firefox/(\d+)\.(\d+)").unwrap();

        re.captures(ua).map(|captures| {
            let major_version = i8::from_str(&captures[1]).unwrap();
            let minor_version = i8::from_str(&captures[2]).unwrap();
            (major_version, minor_version)
        })
    }

    pub fn match_chrome(ua: &str) -> Option<(i8, i8)> {
        let re = Regex::new(r"(Chromium|Chrome)/(\d+)\.(\d+)").unwrap();

        re.captures(ua).map(|captures| {
            let major_version = i8::from_str(&captures[2]).unwrap();
            let minor_version = i8::from_str(&captures[3]).unwrap();
            (major_version, minor_version)
        })
    }

    pub fn match_safari(ua: &str) -> Option<(i8, i8)> {
        let re = Regex::new(r"Version/(\d+)\.(\d+)(?:\.\d+)? Safari").unwrap();

        re.captures(ua).map(|captures| {
            let major_version = i8::from_str(&captures[1]).unwrap();
            let minor_version = i8::from_str(&captures[2]).unwrap();
            (major_version, minor_version)
        })
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

#[no_mangle]
pub extern "C" fn get_browser_family(ua: *const UserAgent) -> *mut c_char {
    let ua = UserAgent::borrow_from_c(ua);

    let family = match ua.browser.family {
        BrowserFamily::Chrome  => "Chrome",
        BrowserFamily::Edge    => "Edge",
        BrowserFamily::Firefox => "Firefox",
        BrowserFamily::Opera   => "Opera",
        BrowserFamily::Safari  => "Safari",
        BrowserFamily::Other   => "Other",
    };

    CString::new(family).unwrap().into_raw()
}

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
        assert_eq!(mobile_version6, None)
    }
}

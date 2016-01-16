use regex::{Captures, Regex};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum BrowserFamily {
    Chrome,
    Edge,
    Firefox,
    Opera,
    Safari,
    MobileSafari,
    Other,
}

pub struct Browser {
    pub family: BrowserFamily,
    pub major_version: i8,
    pub minor_version: i8,
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
    fn match_versions(ua: &str, regex: &Regex) -> Option<(i8, i8)> {
        regex
            .captures(ua)
            .map(Browser::map_first_captures)
    }
}

lazy_static! {
    static ref CHROME_REGEX: Regex  = Regex::new(r"(?:Chromium|Chrome)/(\d+)\.(\d+)").unwrap();
    static ref EDGE_REGEX: Regex    = Regex::new(r"Edge/(\d+)\.(\d+)").unwrap();
    static ref FIREFOX_REGEX: Regex = Regex::new(r"Firefox/(\d+)\.(\d+)").unwrap();
    static ref SAFARI_REGEX: Regex  = Regex::new(r"Version/(\d+)\.(\d+)(?:\.\d+)?(?: Mobile/\w+)? Safari").unwrap();
}

impl Browser {
    pub fn match_edge(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, &EDGE_REGEX)
    }

    /// Search for the Firefox componenet in the user agent and parse out the version if present
    pub fn match_firefox(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, &FIREFOX_REGEX)
    }

    pub fn match_chrome(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, &CHROME_REGEX)
    }

    pub fn match_safari(ua: &str) -> Option<(i8, i8)> {
        Browser::match_versions(ua, &SAFARI_REGEX)
    }
}

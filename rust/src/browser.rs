use regex::{Captures, Regex};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
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

type MatcherFn = Fn(&str) -> Option<(i8, i8)> + Sync;
type Matcher = (BrowserFamily, Box<MatcherFn>);

lazy_static! {
    // NOTE: Order of tests is significant
    static ref MATCH_SEQUENCE: Vec<Matcher> = vec![
        (BrowserFamily::Opera,  Box::new(Browser::match_opera)),
        (BrowserFamily::Edge,   Box::new(Browser::match_edge)),
        (BrowserFamily::Chrome, Box::new(Browser::match_chrome)),
    ];
}

impl Browser {
    fn new(family: BrowserFamily, versions: (i8, i8)) -> Browser {
        Browser {
            family:        family,
            major_version: versions.0,
            minor_version: versions.1,
        }
    }

    #[allow(unused_parens)]
    pub fn parse(ua: &str) -> Browser {
        // NOTE: Order of these tests is significant because browser vendors are terrible

        for tuple in MATCH_SEQUENCE.iter() {
            let &(ref family, ref matcher) = tuple;

            if let Some(versions) = matcher(ua) {
                return Browser::new(family.clone(), versions)
            }
        }

        let matched_firefox = Browser::match_firefox(ua);
        let matched_safari  = Browser::match_safari(ua);

        return (
            if let Some(versions) = matched_firefox {
                Browser::new(BrowserFamily::Firefox, versions)

            } else if let Some(versions) = matched_safari {
                let family =
                    if ua.contains("Mobile/") {
                        BrowserFamily::MobileSafari
                    } else {
                        BrowserFamily::Safari
                    };
                Browser::new(family, versions)

            } else {
                Browser::new(BrowserFamily::Other, (0, 0))
            }
        )
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
    static ref OPERA_VERSION_REGEX: Regex = Regex::new(r"Version/(\d+)\.(\d+)").unwrap();
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

    pub fn match_opera(ua: &str) -> Option<(i8, i8)> {
        if !ua.contains("Opera") {
            return None
        }

        Browser::match_versions(ua, &OPERA_VERSION_REGEX)
    }

    pub fn match_safari(ua: &str) -> Option<(i8, i8)> {
        // SAFARI_REGEX is sort of expensive so we shortcut it with a simple search first
        if !ua.contains("Safari") {
            return None
        }

        Browser::match_versions(ua, &SAFARI_REGEX)
    }
}

use regex::{Regex};

use util::map_first_captures;

#[derive(Clone, Debug, PartialEq)]
pub enum BrowserFamily {
    Android,
    Chrome,
    Edge,
    Firefox,
    Opera,
    Safari,
    MobileSafari,
}

impl BrowserFamily {
    pub fn is_mobile(&self) -> bool {
        match *self {
            BrowserFamily::Android      => true,
            BrowserFamily::MobileSafari => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
        (BrowserFamily::Opera,        Box::new(Browser::match_opera)),
        (BrowserFamily::Edge,         Box::new(Browser::match_edge)),
        (BrowserFamily::Android,      Box::new(Browser::match_android)),
        (BrowserFamily::Chrome,       Box::new(Browser::match_chrome)),
        (BrowserFamily::Firefox,      Box::new(Browser::match_firefox)),
        (BrowserFamily::MobileSafari, Box::new(Browser::match_mobile_safari)),
        (BrowserFamily::Safari,       Box::new(Browser::match_safari)),
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
    pub fn parse(ua: &str) -> Option<Browser> {
        for tuple in MATCH_SEQUENCE.iter() {
            let &(ref family, ref matcher) = tuple;

            if let Some(versions) = matcher(ua) {
                let browser = Browser::new(family.clone(), versions);

                return Some(browser)
            }
        }

        return None
    }

    /// Take a regex and attempt to match it to the browser. The regex must include two capture
    /// groups that capture the version of the matched browser.
    fn match_versions(ua: &str, regex: &Regex) -> Option<(i8, i8)> {
        regex
            .captures(ua)
            .map(map_first_captures)
    }
}

lazy_static! {
    static ref CHROME_REGEX: Regex          = Regex::new(r"Chrom(?:ium|e)/(\d+)\.(\d+)").unwrap();
    static ref EDGE_REGEX: Regex            = Regex::new(r"Edge/(\d+)\.(\d+)").unwrap();
    static ref FIREFOX_REGEX: Regex         = Regex::new(r"Firefox/(\d+)\.(\d+)").unwrap();
    static ref OPERA_VERSION_REGEX: Regex   = Regex::new(r"Version/(\d+)\.(\d+)").unwrap();
    static ref SAFARI_VERSION_REGEX: Regex  = Regex::new(r"Version/(\d+)\.(\d+)").unwrap();
    static ref ANDROID_VERSION_REGEX: Regex = Regex::new(r"Version/(\d+)\.(\d+)").unwrap();
}

impl Browser {
    pub fn match_android(ua: &str) -> Option<(i8, i8)> {
        if !ua.contains("Android") { return None }

        Browser::match_versions(ua, &ANDROID_VERSION_REGEX)
    }

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
        if !ua.contains("Opera") { return None }

        Browser::match_versions(ua, &OPERA_VERSION_REGEX)
    }

    pub fn match_safari(ua: &str) -> Option<(i8, i8)> {
        if !ua.contains("Safari") { return None }
        if ua.contains("Mobile/") { return None }

        Browser::match_versions(ua, &SAFARI_VERSION_REGEX)
    }

    pub fn match_mobile_safari(ua: &str) -> Option<(i8, i8)> {
        if !ua.contains("Safari")  { return None }
        if !ua.contains("Mobile/") { return None }

        Browser::match_versions(ua, &SAFARI_VERSION_REGEX)
    }
}

#[cfg(test)]
mod tests {
    use super::{Browser, BrowserFamily};

    type StaticStr = &'static str;

    const ANDROID_4: StaticStr       = "Mozilla/5.0 (Linux; U; Android 4.0.3; ko-kr; LG-L160L Build/IML74K) AppleWebkit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30";
    const OPERA_12: StaticStr        = "Opera/9.80 (X11; Linux i686; Ubuntu/14.10) Presto/2.12.388 Version/12.16";
    const OPERA_11: StaticStr        = "Opera/9.80 (Windows NT 6.1; WOW64; U; pt) Presto/2.10.229 Version/11.62";
    const SAFARI_7: StaticStr        = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A";
    const SAFARI_5: StaticStr        = "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; en-us) AppleWebKit/534.1+ (KHTML, like Gecko) Version/5.0 Safari/533.16";
    const MOBILE_SAFARI_6: StaticStr = "Mozilla/5.0 (iPad; CPU OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5355d Safari/8536.25";

    #[test]
    fn test_parse_safari() {
        assert_eq!(
            Browser::new(BrowserFamily::Safari, (7, 0)),
            Browser::parse(SAFARI_7).unwrap()
        );

        assert_eq!(
            Browser::new(BrowserFamily::MobileSafari, (6, 0)),
            Browser::parse(MOBILE_SAFARI_6).unwrap()
        )
    }

    #[test]
    fn test_match_android() {
        let does_match = Browser::match_android(ANDROID_4);
        assert_eq!(does_match, Some((4, 0)));
    }

    #[test]
    fn test_match_firefox() {
        let did_match = Browser::match_firefox("Firefox/1.2");
        assert_eq!(did_match, Some((1, 2)));

        let didnt_match = Browser::match_firefox("NotFirefox/x.y");
        assert_eq!(didnt_match, None)
    }

    #[test]
    fn test_match_safari() {
        let version_7 = Browser::match_safari(SAFARI_7);
        assert_eq!(version_7, Some((7, 0)));

        let version_5 = Browser::match_safari(SAFARI_5);
        assert_eq!(version_5, Some((5, 0)));
    }

    #[test]
    fn test_match_mobile_safari() {
        let mobile_version_6 = Browser::match_mobile_safari(MOBILE_SAFARI_6);
        assert_eq!(mobile_version_6, Some((6, 0)))
    }

    #[test]
    fn test_match_opera() {
        let opera_12 = Browser::match_opera(OPERA_12);
        assert_eq!(opera_12, Some((12, 16)));

        let opera_11 = Browser::match_opera(OPERA_11);
        assert_eq!(opera_11, Some((11, 62)))
    }
}

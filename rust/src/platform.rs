use regex::{Error as RegexError, Regex};
use std::str::FromStr;

use self::MatchPattern::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PlatformName {
    Android,
    IOS,
    Linux,
    Mac,
    Windows,
}

#[derive(Debug, PartialEq)]
pub struct Platform {
    pub name: PlatformName,
    pub major_version: i8,
    pub minor_version: i8,
}

impl Platform {
    pub fn new(name: PlatformName,
               major_version: i8,
               minor_version: i8) -> Platform {
        Platform {
            name: name,
            major_version: major_version,
            minor_version: minor_version,
        }
    }

    pub fn parse(ua: &str) -> Option<Platform> {
        for &(ref match_pattern, ref name, major, minor) in MATCH_SEQUENCE.iter() {
            let mut major = major;
            let mut minor = minor;

            let matched = match match_pattern {
                &MatchRegex(ref regex) => {
                    if let Some(captures) = regex.captures(ua) {
                        // `len()` returns 3 when there are 2 capture groups
                        // because it's including the full match as well
                        // as a capture
                        if captures.len() == 3 {
                            major = i8::from_str(&captures[1]).unwrap();
                            minor = i8::from_str(&captures[2]).unwrap();
                        }
                        true
                    } else {
                        false
                    }
                },
                &MatchString(ref string) => ua.contains(string),
            };

            if matched {
                return Some(Platform::new(name.clone(), major, minor))
            }
        }

        None
    }
}

enum MatchPattern {
    MatchRegex(Regex),
    MatchString(String),
}

impl MatchPattern {
    fn with_regex(regex: &str) -> Result<MatchPattern, RegexError> {
        Regex::new(regex).map(|regex| MatchPattern::MatchRegex(regex))
    }

    fn with_str(string: &str) -> MatchPattern {
        MatchPattern::MatchString(string.to_owned())
    }
}

type MatchTuple = (MatchPattern, PlatformName, i8, i8);

lazy_static! {
    static ref MATCH_SEQUENCE: Vec<MatchTuple> = {
        use self::PlatformName::*;

        let ios_pattern = r"CPU (?:iPhone )?OS (\d+)_(\d+)(?:_\d+)? like Mac OS X";
        let mac_pattern = r"Mac OS X (\d+)_(\d+)(?:_\d+)?";

        vec![
            (MatchPattern::with_regex(ios_pattern).unwrap(), IOS,     -1, -1),
            (MatchPattern::with_regex(mac_pattern).unwrap(), Mac,     -1, -1),
            (MatchPattern::with_str("Android"),              Android, 0, 0),
            (MatchPattern::with_str("Linux"),                Linux,   0, 0),
            (MatchPattern::with_str("Macintosh"),            Mac,     0, 0),
            (MatchPattern::with_str("Windows XP"),           Windows, 5, 1),
            (MatchPattern::with_str("Windows NT 5.1"),       Windows, 5, 1), // Also Windows XP
            (MatchPattern::with_str("Windows NT 6.0"),       Windows, 6, 0), // Windows Vista
            (MatchPattern::with_str("Windows NT 6.1"),       Windows, 7, 0),
            (MatchPattern::with_str("Windows NT 6.2"),       Windows, 8, 0),
            (MatchPattern::with_str("Windows NT 6.3"),       Windows, 8, 1),
            (MatchPattern::with_str("Windows NT 10.0"),      Windows, 10, 0),
            (MatchPattern::with_str("Windows"),              Windows, 0, 0), // Match any other Windows
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::Platform;
    use super::PlatformName::*;

    const ANDROID_444: &'static str = "Mozilla/5.0 (Linux; Android 4.4.4; One Build/KTU84L.H4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/36.0.1985.135 Mobile Safari/537.36";
    const IOS_802: &'static str     = "Mozilla/5.0 (iPhone; CPU iPhone OS 8_0_2 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12A366 Safari/600.1.4";
    const MAC_1093: &'static str    = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A";
    const WINDOWS_81: &'static str  = "Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko";

    #[test]
    fn matches_android_444() {
        assert_eq!(
            Platform::parse(ANDROID_444),
            Some(Platform::new(Android, 0, 0))
        )
    }

    #[test]
    fn matches_ios_802() {
        assert_eq!(
            Platform::parse(IOS_802),
            Some(Platform::new(IOS, 8, 0))
        )
    }

    #[test]
    fn matches_mac1093() {
        assert_eq!(
            Platform::parse(MAC_1093),
            Some(Platform::new(Mac, 10, 9))
        )
    }

    #[test]
    fn matches_windows_81() {
        assert_eq!(
            Platform::parse(WINDOWS_81),
            Some(Platform::new(Windows, 8, 1))
        )
    }
}

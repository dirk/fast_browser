use regex::{Error as RegexError, Regex};

use self::MatchPattern::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PlatformName {
    IOS,
    Mac,
    Windows,
}

#[derive(Debug, PartialEq)]
pub struct Platform {
    pub name: PlatformName,
    pub major_version: u8,
    pub minor_version: u8,
}

impl Platform {
    pub fn new(name: PlatformName,
               major_version: u8,
               minor_version: u8) -> Platform {
        Platform {
            name: name,
            major_version: major_version,
            minor_version: minor_version,
        }
    }

    pub fn parse(ua: &str) -> Option<Platform> {
        for &(ref match_pattern, ref name, major, minor) in MATCH_SEQUENCE.iter() {
            let matched = match match_pattern {
                &MatchRegex(ref regex) => regex.is_match(ua),
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

type MatchTuple = (MatchPattern, PlatformName, u8, u8);

lazy_static! {
    static ref MATCH_SEQUENCE: Vec<MatchTuple> = {
        use self::PlatformName::*;

        vec![
            (MatchPattern::with_regex(r"CPU OS [0-9]_[0-9](_[0-9])? like Mac OS X").unwrap(), IOS, 0, 0),
            (MatchPattern::with_str("Macintosh"),                                             Mac,     0, 0),
            (MatchPattern::with_str("Windows XP"),                                            Windows, 5, 1),
            (MatchPattern::with_str("Windows NT 5.1"),                                        Windows, 5, 1), // Also Windows XP
            (MatchPattern::with_str("Windows NT 6.0"),                                        Windows, 6, 0), // Windows Vista
            (MatchPattern::with_str("Windows NT 6.1"),                                        Windows, 7, 0),
            (MatchPattern::with_str("Windows NT 6.2"),                                        Windows, 8, 0),
            (MatchPattern::with_str("Windows NT 6.3"),                                        Windows, 8, 1),
            (MatchPattern::with_str("Windows NT 10.0"),                                       Windows, 10, 0),
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::Platform;
    use super::PlatformName::*;

    const WINDOWS_81: &'static str = "Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko";

    #[test]
    fn matches_windows_81() {
        assert_eq!(
            Platform::parse(WINDOWS_81),
            Some(Platform::new(Windows, 8, 1))
        )
    }
}

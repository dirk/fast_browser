#[derive(Clone)]
pub enum PlatformName {
    Windows,
}

pub struct Platform {
    pub name: PlatformName,
    pub major_version: u8,
    pub minor_version: u8,
}

impl Platform {
    pub fn parse(ua: &str) -> Option<Platform> {
        for &(match_string, ref name, major, minor) in MATCH_SEQUENCE.iter() {
            if ua.contains(match_string) {
                return Some(Platform {
                    name: name.clone(),
                    major_version: major,
                    minor_version: minor,
                })
            }
        }

        None
    }
}

type MatchString = (&'static str, PlatformName, u8, u8);

lazy_static! {
    static ref MATCH_SEQUENCE: Vec<MatchString> = {
        use self::PlatformName::*;

        vec![
            ("Windows NT 6.3", Windows, 8, 1),
        ]
    };
}

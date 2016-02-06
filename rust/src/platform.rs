pub enum PlatformName {
    Windows,
}

pub struct Platform {
    name: PlatformName,
}

impl Platform {
    pub fn parse(ua: &str) -> Option<Platform> {
        None
    }
}

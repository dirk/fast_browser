use std::mem;
use super::browser::Browser;

pub struct UserAgent {
    pub browser: Option<Browser>,
}

impl UserAgent {
    pub fn parse(ua: &str) -> UserAgent {
        let browser = Browser::parse(ua);

        UserAgent {
            browser: Some(browser),
        }
    }

    /// Take an externally-owned `Browser` and non-destructively borrow a reference to it.
    ///
    /// **Note**: This will *not* deallocate the instance passed in. So it is safe to call this
    /// over and over again.
    pub fn borrow_from_c<'a>(ua: *const UserAgent) -> &'a UserAgent {
        unsafe { mem::transmute(ua) }
    }
}

use std::mem;
use bot::Bot;
use browser::Browser;

pub struct UserAgent {
    pub browser: Option<Browser>,
    pub bot: Option<Bot>,

    /// The string that was parsed to determine the browser, bot, etc.
    pub source: String,
}

impl UserAgent {
    pub fn parse(ua: &str) -> UserAgent {
        let mut bot: Option<Bot>         = None;
        let mut browser: Option<Browser> = None;

        if let Some(has_bot) = Bot::parse(ua) {
            bot = Some(has_bot);
        } else {
            // Only try to parse for a browser if it isn't a bot
            browser = Browser::parse(ua)
        }

        UserAgent {
            browser: browser,
            bot:     bot,
            source:  ua.to_owned(),
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

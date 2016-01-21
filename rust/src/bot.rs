use regex::{Regex};
use util::map_first_captures;

#[derive(Debug, PartialEq)]
pub enum BotName {
    Bingbot,
    Googlebot,
}

#[derive(Debug, PartialEq)]
pub struct Bot {
    pub name: BotName,
}

impl Bot {
    pub fn new(name: BotName) -> Bot {
        Bot { name: name, }
    }

    pub fn parse(ua: &str) -> Option<Bot> {
        for matcher in MATCH_SEQUENCE.iter() {
            if let Some(bot) = matcher(ua) {
                return Some(bot)
            }
        }

        None
    }

    fn match_googlebot(ua: &str) -> Option<Bot> {
        GOOGLEBOT_REGEX
            .captures(ua)
            .map(map_first_captures)
            .map(|_| Bot::new(BotName::Googlebot))
    }

    fn match_bingbot(ua: &str) -> Option<Bot> {
        BINGBOT_REGEX
            .captures(ua)
            .map(map_first_captures)
            .map(|_| Bot::new(BotName::Bingbot))
    }
}

type MatcherFn = Fn(&str) -> Option<Bot> + Sync;
type Matcher = Box<MatcherFn>;

lazy_static! {
    static ref GOOGLEBOT_REGEX: Regex = Regex::new(r"Googlebot/(\d+)\.(\d+)").unwrap();
    static ref BINGBOT_REGEX: Regex   = Regex::new(r"bingbot/(\d+)\.(\d+)").unwrap();

    static ref MATCH_SEQUENCE: Vec<Matcher> = vec![
        Box::new(Bot::match_googlebot),
        Box::new(Bot::match_bingbot),
    ];
}

#[cfg(test)]
mod tests {
    use super::{Bot, BotName};

    const GOOGLEBOT: &'static str = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    const BINGBOT: &'static str   = "Mozilla/5.0 (iPhone; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)";

    #[test]
    fn test_parse_googlebot() {
        assert_eq!(
            Bot::new(BotName::Googlebot),
            Bot::parse(GOOGLEBOT).unwrap()
        )
    }

    #[test]
    fn test_parse_bingbot() {
        assert_eq!(
            Bot::new(BotName::Bingbot),
            Bot::parse(BINGBOT).unwrap()
        )
    }
}

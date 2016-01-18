use regex::{Regex};
use util::map_first_captures;

#[derive(Debug, PartialEq)]
pub enum BotName {
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
        let versions = GOOGLEBOT_REGEX
            .captures(ua)
            .map(map_first_captures);

        if let Some(_) = versions {
            Some(Bot::new(BotName::Googlebot))
        } else {
            None
        }
    }
}

type MatcherFn = Fn(&str) -> Option<Bot> + Sync;
type Matcher = Box<MatcherFn>;

lazy_static! {
    static ref GOOGLEBOT_REGEX: Regex = Regex::new(r"Googlebot/(\d+)\.(\d+)").unwrap();

    static ref MATCH_SEQUENCE: Vec<Matcher> = vec![
        Box::new(Bot::match_googlebot),
    ];
}

#[cfg(test)]
mod tests {
    use super::{Bot, BotName};

    const GOOGLEBOT: &'static str = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";

    #[test]
    fn test_parse_googlebot() {
        assert_eq!(
            Bot::new(BotName::Googlebot),
            Bot::parse(GOOGLEBOT).unwrap()
        )
    }
}

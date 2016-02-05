use regex::{Regex};
use util::map_first_captures;

#[derive(Clone, Debug, PartialEq)]
pub enum BotName {
    Baidu,
    Bingbot,
    DuckDuckBot,
    Go, // Go language's HTTP package
    Googlebot,
}

impl ToString for BotName {
    fn to_string(&self) -> String {
        use self::BotName::*;

        match *self {
            Go => "Go HTTP package".to_owned(),
            _ => format!("{:?}", self)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    fn make_matcher(search: &str, name: BotName) -> Box<Fn(&str) -> Option<Bot> + Sync> {
        let search = search.to_owned();

        Box::new(move |ua: &str| {
            if ua.contains(&search) {
                Some(Bot::new(name.clone()))
            } else {
                None
            }
        })
    }

    fn make_version_regex_matcher(regex: Regex, name: BotName)
                                  -> Box<Fn(&str) -> Option<Bot> + Sync> {
        Box::new(move |ua: &str| {
            regex
                .captures(ua)
                .map(map_first_captures)
                .map(|_| Bot::new(name.clone()))
        })
    }

}

type MatcherFn = Fn(&str) -> Option<Bot> + Sync;
type Matcher = Box<MatcherFn>;

lazy_static! {
    static ref MATCH_SEQUENCE: Vec<Matcher> = {
        let bingbot_regex: Regex   = Regex::new(r"bingbot/(\d+)\.(\d+)").unwrap();
        let googlebot_regex: Regex = Regex::new(r"Googlebot/(\d+)\.(\d+)").unwrap();

        vec![
            Bot::make_version_regex_matcher(googlebot_regex, BotName::Googlebot),
            Bot::make_version_regex_matcher(bingbot_regex, BotName::Bingbot),
            Bot::make_matcher("Baidu", BotName::Baidu),
            Bot::make_matcher("DuckDuckBot", BotName::DuckDuckBot),
            Bot::make_matcher("Go-http-client", BotName::Go),
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::{Bot, BotName};

    const BAIDU: &'static str     = "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)";
    const BINGBOT: &'static str   = "Mozilla/5.0 (iPhone; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)";
    const GOOGLEBOT: &'static str = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";

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

    #[test]
    fn test_parse_baidu() {
        assert_eq!(
            Bot::new(BotName::Baidu),
            Bot::parse(BAIDU).unwrap()
        )
    }
}

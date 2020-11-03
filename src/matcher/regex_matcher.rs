use super::matcher;

pub struct RegexMatcher {
    regexes: regex::RegexSet,
}

impl RegexMatcher {
    pub fn new(patterns: &Vec<String>) -> Result<RegexMatcher, String> {
        Ok(RegexMatcher {
            regexes: regex::RegexSet::new(patterns).map_err(|e| match e {
                regex::Error::Syntax(text) => text,
                _ => "initializing regex failed".to_string(),
            })?,
        })
    }
}

impl matcher::Matcher for RegexMatcher {
    fn matches(&self, line: String) -> bool {
        self.regexes.is_match(line.as_str())
    }
}

pub mod lines;
pub mod regex_matcher;

pub trait Matcher {
    fn matches(&self, line: String) -> bool;
}

pub trait Matcher {
    fn matches(&self, line: String) -> bool;
}

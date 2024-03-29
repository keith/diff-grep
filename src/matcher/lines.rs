extern crate patch;

use super::Matcher;

pub fn only_contains_matching_lines(hunk: &patch::Hunk, matcher: &dyn Matcher) -> bool {
    for line in &hunk.lines {
        match line {
            patch::Line::Add(text) | patch::Line::Remove(text) => {
                if !matcher.matches(text.to_string()) {
                    return false;
                }
            }
            patch::Line::Context(_) => (),
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::super::regex_matcher;
    use super::*;

    fn test_hunk() -> patch::Hunk<'static> {
        let diff_str = r#"
diff --git i/docs/Algorithm::C35.18.3pm.html w/docs/Algorithm::C35.18.3pm.html
index e86bee4d..dd169e50 100644
--- i/docs/Algorithm::C35.18.3pm.html
+++ w/docs/Algorithm::C35.18.3pm.html
@@ -255,7 +255,7 @@ Copyright 2006 by Infinity Interactive, Inc.
 </div>
 <table class="foot">
   <tr>
-    <td class="foot-date">2020-09-20</td>
+    <td class="foot-date">2020-09-30</td>
     <td class="foot-os">perl v5.18.4</td>
   </tr>
 </table>
"#;

        let patch = patch::Patch::from_single(diff_str).unwrap();
        let hunks = patch.hunks;
        assert_eq!(hunks.len(), 1);

        hunks.first().unwrap().clone()
    }

    fn test_detatched_hunk() -> patch::Hunk<'static> {
        let diff_str = r#"
diff --git i/docs/Algorithm::C35.18.3pm.html w/docs/Algorithm::C35.18.3pm.html
index e86bee4d..dd169e50 100644
--- i/docs/Algorithm::C35.18.3pm.html
+++ w/docs/Algorithm::C35.18.3pm.html
@@ -255,7 +255,7 @@ Copyright 2006 by Infinity Interactive, Inc.
 </div>
 <table class="foot">
-    foo
   <tr>
+    bar
     <td class="foot-os">perl v5.18.4</td>
   </tr>
 </table>
"#;

        let patch = patch::Patch::from_single(diff_str).unwrap();
        let hunks = patch.hunks;
        assert_eq!(hunks.len(), 1);

        hunks.first().unwrap().clone()
    }

    #[test]
    fn test_no_match() {
        assert!(!match_helper(&test_hunk(), &vec!["bar".to_string()]));
    }

    #[test]
    fn test_match() {
        assert!(match_helper(&test_hunk(), &vec!["foot-date".to_string()]));
    }

    #[test]
    fn test_regex_match() {
        assert!(match_helper(&test_hunk(), &vec!["f[o]+t-date".to_string()]));
    }

    #[test]
    fn test_entire_regex_match() {
        assert!(match_helper(&test_hunk(), &vec![r"[\w-]+".to_string()]));
    }

    #[test]
    fn test_single_match() {
        assert!(match_helper(
            &test_hunk(),
            &vec!["bar".to_string(), "foot-date".to_string()]
        ));
    }

    #[test]
    fn test_detatched_no_match() {
        assert!(!match_helper(
            &test_detatched_hunk(),
            &vec!["bar".to_string(), "foot-date".to_string()]
        ));
    }

    #[test]
    fn test_detatched_match() {
        assert!(match_helper(
            &test_detatched_hunk(),
            &vec!["bar".to_string(), "foo".to_string()]
        ));
    }

    fn match_helper(hunk: &patch::Hunk, patterns: &Vec<String>) -> bool {
        only_contains_matching_lines(hunk, &regex_matcher::RegexMatcher::new(patterns).unwrap())
    }
}

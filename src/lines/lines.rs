extern crate unidiff;

pub fn only_contains_ignored_patterns(hunk: unidiff::Hunk, patterns: &Vec<String>) -> bool {
    for line in hunk {
        if line.is_added() || line.is_removed() {
            if !patterns.iter().any(|p| line.value.contains(p)) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_hunk() -> unidiff::Hunk {
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

        let mut patch = unidiff::PatchSet::new();
        patch.parse(diff_str).ok().unwrap();
        let modified_files = patch.modified_files();
        assert_eq!(modified_files.len(), 1);

        let file = modified_files.first().unwrap();
        let hunks = file.hunks();
        assert_eq!(hunks.len(), 1);

        hunks.first().unwrap().clone()
    }

    fn test_detatched_hunk() -> unidiff::Hunk {
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

        let mut patch = unidiff::PatchSet::new();
        patch.parse(diff_str).ok().unwrap();
        let modified_files = patch.modified_files();
        assert_eq!(modified_files.len(), 1);

        let file = modified_files.first().unwrap();
        let hunks = file.hunks();
        assert_eq!(hunks.len(), 1);

        hunks.first().unwrap().clone()
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            only_contains_ignored_patterns(test_hunk(), &vec!["bar".to_string()]),
            false
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            only_contains_ignored_patterns(test_hunk(), &vec!["foot-date".to_string()]),
            true
        );
    }

    #[test]
    fn test_single_match() {
        assert_eq!(
            only_contains_ignored_patterns(
                test_hunk(),
                &vec!["bar".to_string(), "foot-date".to_string()]
            ),
            true
        );
    }

    #[test]
    fn test_detatched_no_match() {
        assert_eq!(
            only_contains_ignored_patterns(
                test_detatched_hunk(),
                &vec!["bar".to_string(), "foot-date".to_string()]
            ),
            false
        );
    }

    #[test]
    fn test_detatched_match() {
        assert_eq!(
            only_contains_ignored_patterns(
                test_detatched_hunk(),
                &vec!["bar".to_string(), "foo".to_string()]
            ),
            true
        );
    }
}

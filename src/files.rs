pub fn get_file(file: patch::File) -> patch::File {
    patch::File {
        path: get_filename(file).into(),
        meta: None,
    }
}

// https://github.com/uniphil/patch-rs/issues/11
fn get_filename(f: patch::File) -> String {
    let suffix_hack = match f.meta {
        Some(patch::FileMetadata::Other(b)) => b.to_string(),
        _ => "".into(),
    };

    format!("{} {}", f.path, suffix_hack).trim().into()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_patch() -> patch::Patch<'static> {
        let diff_str = "
--- i/Xcode.app/Contents/Frameworks/IDEFoundation.framework/Versions/A/Resources/Xcode SSH Authentication Agent\t
+++ w/Xcode.app/Contents/Frameworks/IDEFoundation.framework/Versions/A/Resources/Xcode SSH Authentication Agent\t
@@ -1,4 +1,4 @@
-@(#)PROGRAM:Xcode SSH Authentication Agent  PROJECT:IDEFrameworks-17518
+@(#)PROGRAM:Xcode SSH Authentication Agent  PROJECT:IDEFrameworks-17529.2
 IDESourceControlRepositoryRequiresHostIdentityConfirmationNotification
 IDESourceControlRepositoryHostIdentityResponseNotification
 IDESourceControlHostIdentityBundleKey
";

        patch::Patch::from_single(diff_str).unwrap()
    }

    #[test]
    fn test_get_filename() {
        assert_eq!(
        get_filename(get_patch().old),
            "i/Xcode.app/Contents/Frameworks/IDEFoundation.framework/Versions/A/Resources/Xcode SSH Authentication Agent");
    }
}

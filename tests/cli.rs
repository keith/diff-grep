use std::fs;
use std::io::Write;
use std::process::{Command, Output, Stdio};

const DIFF: &str = "\
--- a/example.txt
+++ b/example.txt
@@ -1,2 +1,2 @@
-old alpha
+new alpha
 keep
@@ -4,2 +4,2 @@
-old beta
+new beta
 keep
";

const ALPHA_DIFF: &str = "\
--- a/example.txt
+++ b/example.txt
@@ -1,2 +1,2 @@
-old alpha
+new alpha
 keep
";

const BETA_DIFF: &str = "\
--- a/example.txt
+++ b/example.txt
@@ -4,2 +4,2 @@
-old beta
+new beta
 keep
";

fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_diff-grep"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn filters_hunks_from_stdin() {
    let output = run(&["alpha"], DIFF);
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(String::from_utf8(output.stdout).unwrap(), ALPHA_DIFF);

    let output = run(&["a.ph."], DIFF);
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(String::from_utf8(output.stdout).unwrap(), ALPHA_DIFF);

    let output = run(&["missing"], DIFF);
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty());
}

#[test]
fn ignores_blank_changes_when_matching() {
    let diff = r#"--- a/BUILD.bazel
+++ b/BUILD.bazel
@@ -1,2 +1,3 @@
-
+load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
+
 cc_binary(
"#;
    let output = run(&["cc_binary"], diff);
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(String::from_utf8(output.stdout).unwrap(), diff);

    let output = run(&["missing"], diff);
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty());

    let output = run(&["-v", "cc_binary"], diff);
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty());

    for whitespace in [" ", "\t"] {
        for marker in ["+", "-"] {
            let diff = diff.replace(
                &format!("\n{marker}\n"),
                &format!("\n{marker}{whitespace}\n"),
            );
            let output = run(&["cc_binary"], &diff);
            assert!(output.status.success(), "{:?}", output);
            assert_eq!(String::from_utf8(output.stdout).unwrap(), diff);
        }
    }
}

#[test]
fn supports_file_paths_and_inverted_matches() {
    let dir = std::env::temp_dir().join(format!(
        "diff-grep-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&dir).unwrap();
    let input = dir.join("input.diff");
    let output_path = dir.join("output.diff");
    fs::write(&input, DIFF).unwrap();

    let output = run(
        &[
            "--input",
            input.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
            "-v",
            "alpha",
        ],
        "",
    );
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read_to_string(&output_path).unwrap(), BETA_DIFF);
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rejects_missing_and_invalid_patterns() {
    let output = run(&[], DIFF);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("required"));

    let output = run(&["["], DIFF);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("error:"));
}

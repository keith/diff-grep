extern crate patch;

mod files;
mod io;
mod matcher;
mod opts;

fn print_error_and_exit(msg: String) -> ! {
    eprintln!("error: {}", msg);
    std::process::exit(1);
}

fn main() {
    let options = opts::Opts::parse();
    let diff_str = match io::read(&options.input) {
        Ok(string) => string,
        Err(err) => {
            print_error_and_exit(err);
        }
    };

    let patches = patch::Patch::from_multiple(&diff_str).unwrap_or_else(|_| {
        print_error_and_exit(format!("failed to parse diff from {}", &options.input));
    });

    let matcher = match matcher::regex_matcher::RegexMatcher::new(&options.patterns) {
        Ok(matcher) => matcher,
        Err(err) => {
            print_error_and_exit(err);
        }
    };

    let invert = options.invert_match;
    let mut files: Vec<patch::Patch> = vec![];
    for patched_file in patches {
        let mut hunks_per_file = vec![];
        for hunk in patched_file.hunks {
            let matches = matcher::lines::only_contains_matching_lines(&hunk, &matcher);
            if (matches && !invert) || (invert && !matches) {
                hunks_per_file.push(hunk);
            }
        }

        if !hunks_per_file.is_empty() {
            files.push(patch::Patch {
                old: files::get_file(patched_file.old),
                new: files::get_file(patched_file.new),
                hunks: hunks_per_file,
                end_newline: patched_file.end_newline,
            });
        }
    }

    if let Err(err) = io::write(options.output, files) {
        print_error_and_exit(err);
    }
}

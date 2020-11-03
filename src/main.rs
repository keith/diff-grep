extern crate patch;

mod files;
mod io;
mod matcher;
mod opts;

fn main() {
    let options = opts::Opts::parse();
    let diff_str = match io::read(&options.input) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1)
        }
    };

    let patches = patch::Patch::from_multiple(&diff_str).unwrap_or_else(|_| {
        eprintln!("error: failed to parse diff from {}", &options.input);
        std::process::exit(1)
    });

    let matcher = match matcher::regex_matcher::RegexMatcher::new(&options.patterns) {
        Ok(matcher) => matcher,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1)
        }
    };

    let mut files: Vec<patch::Patch> = vec![];
    for patched_file in patches {
        let mut hunks_per_file = vec![];
        for hunk in patched_file.hunks {
            if matcher::lines::only_contains_matching_lines(&hunk, &matcher) {
                hunks_per_file.push(hunk)
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

    std::process::exit(match io::write(options.output, files) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}

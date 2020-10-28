extern crate unidiff;

mod io;
mod lines;
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

    let mut patch_set = unidiff::PatchSet::new();
    match patch_set.parse(diff_str) {
        Ok(()) => (),
        Err(_) => {
            eprintln!("error: failed to parse diff from {}", &options.input);
            std::process::exit(1)
        }
    };

    let mut files = vec![];
    for patched_file in patch_set {
        let mut hunks_per_file = vec![];
        for hunk in patched_file.clone() {
            if lines::only_contains_ignored_patterns(hunk.clone(), &options.patterns) {
                hunks_per_file.push(hunk)
            }
        }

        if !hunks_per_file.is_empty() {
            files.push(unidiff::PatchedFile::with_hunks(
                patched_file.source_file,
                patched_file.target_file,
                hunks_per_file,
            ));
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

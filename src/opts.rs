extern crate clap;

#[derive(clap::Clap)]
#[clap(version = "0.2.0", author = "Keith Smiley <keithbsmiley@gmail.com>")]
pub struct Opts {
    /// The input file path containing the patch to grep, defaults to stdin
    #[clap(short, long, default_value = "-")]
    pub input: String,
    /// The output file path to write any matches, defaults to stdout
    #[clap(short, long, default_value = "-")]
    pub output: String,
    /// The literal patterns to match against
    #[clap(required = true)]
    pub patterns: Vec<String>,
    /// Select lines that do not match any of the patterns
    #[clap(short = 'v', long)]
    pub invert_match: bool,
}

impl Opts {
    pub fn parse() -> Opts {
        clap::Clap::parse()
    }
}

pub extern crate clap;

#[derive(clap::Clap)]
#[clap(version = "0.1.0", author = "Keith Smiley <keithbsmiley@gmail.com>")]
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
}

impl Opts {
    pub fn parse() -> Opts {
        clap::Clap::parse()
    }
}

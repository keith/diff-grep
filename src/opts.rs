#[derive(clap::Parser)]
#[command(version = "0.2.0", author = "Keith Smiley <keithbsmiley@gmail.com>")]
pub struct Opts {
    /// The input file path containing the patch to grep, defaults to stdin
    #[arg(short, long, default_value = "-")]
    pub input: String,
    /// The output file path to write any matches, defaults to stdout
    #[arg(short, long, default_value = "-")]
    pub output: String,
    /// The literal patterns to match against
    #[arg(required = true)]
    pub patterns: Vec<String>,
    /// Select lines that do not match any of the patterns
    #[arg(short = 'v', long)]
    pub invert_match: bool,
}

impl Opts {
    pub fn parse() -> Opts {
        clap::Parser::parse()
    }
}

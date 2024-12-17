use std::path::PathBuf;

use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Expand configuration into key-value pairs used by the run-integration-test action
pub struct Cli {
    /// path to integration test config file
    #[argh(
        option,
        short = 'c',
        long = "config",
        default = "PathBuf::from(\"tests/interu.yaml\")"
    )]
    pub config: PathBuf,

    /// path to instances file
    #[argh(option, short = 'i', long = "instances")]
    pub instances: PathBuf,

    /// output key=value pairs separated by newlines to file
    #[argh(
        option,
        short = 'o',
        long = "output",
        description = "output key=value pairs separated by newlines to file.
    Useful for CI tools which give a file to write env vars and outputs to which are used in subsequent steps"
    )]
    pub output: Option<PathBuf>,

    /// run without producing output on stdout
    // #[arg(short, long, visible_short_alias('s'), visible_alias("silent"))]
    #[argh(switch, short = 'q', long = "quiet")]
    pub quiet: bool,

    /// which test profile to use
    #[argh(positional)]
    pub profile: String,
}

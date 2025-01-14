use std::path::PathBuf;

use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Expand configuration into key=value pairs used by the run-integration-test action
pub struct Cli {
    /// path to integration test config file
    #[argh(
        option,
        short = 'c',
        long = "config",
        default = r#"PathBuf::from("tests/interu.yaml")"#
    )]
    pub config: PathBuf,

    /// path to instances file
    #[argh(option, short = 'i', long = "instances")]
    pub instances: PathBuf,

    /// write configuration key=value pairs separated by newlines to file
    #[argh(
        option,
        short = 'o',
        long = "output",
        description = "write configuration key=value pairs separated by newlines to file. Useful for CI tools which give a file to write env vars and outputs to which are used in subsequent steps"
    )]
    pub output: Option<PathBuf>,

    /// run without producing output on stdout
    // #[arg(short, long, visible_short_alias('s'), visible_alias("silent"))]
    #[argh(switch, short = 'q', long = "quiet")]
    pub quiet: bool,

    /// validate the beku test definition of the selected profile
    #[argh(switch, long = "check-test-definitions")]
    pub check_test_definitions: bool,

    /// path to beku test-definition file [default = tests/test-definition.yaml]
    #[argh(
        option,
        short = 't',
        long = "test-definitions",
        default = r#"PathBuf::from("tests/test-definition.yaml")"#
    )]
    pub test_definitions: PathBuf,

    /// which test profile to use
    #[argh(positional)]
    pub profile: String,
}

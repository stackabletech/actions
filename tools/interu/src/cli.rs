use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
/// Expand configuration into key=value pairs used by the run-integration-test action
pub struct Cli {
    /// Path to integration test config file
    #[arg(
        short, long,
        global = true,
        default_value_os_t = PathBuf::from("tests/interu.yaml"),
        help_heading = "Global Options"
    )]
    pub config: PathBuf,

    /// Path to instances file
    #[arg(short, long, help_heading = "Global Options")]
    pub instances: PathBuf,

    /// Write configuration key=value pairs separated by newlines to file.
    /// Useful for CI tools which give a file to write env vars and outputs to which are used in subsequent steps.
    #[arg(short, long, global = true, help_heading = "Global Options")]
    pub output: Option<PathBuf>,

    /// Run without producing output on stdout
    #[arg(
        short,
        long,
        global = true,
        visible_short_alias('s'),
        visible_alias("silent"),
        help_heading = "Global Options"
    )]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run test(s) with a specific profile.
    Profile(ProfileArguments),

    /// Run test(s) with custom options on a specific runner.
    Custom(CustomArguments),
}

#[derive(Debug, Args)]
pub struct ProfileArguments {
    /// Validate the beku test definition of the selected profile.
    #[arg(long)]
    pub check_test_definitions: bool,

    /// Path to beku test-definition file.
    #[arg(
        short, long,
        default_value_os_t = PathBuf::from("tests/test-definition.yaml")
    )]
    pub test_definitions: PathBuf,

    /// Which test profile to use.
    pub profile: String,
}

#[derive(Debug, Args)]
pub struct CustomArguments {
    /// The test-suite to expand.
    #[arg(long)]
    pub test_suite: Option<String>,

    /// The test to run.
    #[arg(long)]
    pub test: Option<String>,

    /// The amount of tests to run in parallel.
    #[arg(short, long, default_value = "2")]
    pub parallel: usize,

    /// The runner used to run the tests on.
    pub runner: String,
}

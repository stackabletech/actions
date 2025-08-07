use std::{fs::OpenOptions, io::Write as _, path::PathBuf};

use clap::Parser;
use snafu::{report, ResultExt, Snafu};

use crate::{cli::Cli, config::Config, instances::Instances};

mod cli;
mod config;
mod instances;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to load config"))]
    LoadConfig { source: config::Error },

    #[snafu(display("failed to validate test options"))]
    ValidateTestOptions { source: config::Error },

    #[snafu(display("failed to determine expanded parameters"))]
    DetermineParameters { source: config::Error },

    #[snafu(display("failed to load instances file"))]
    LoadInstances { source: instances::Error },

    #[snafu(display("failed to write to output file"))]
    WriteOutputFile { source: std::io::Error },
}

#[report]
fn main() -> Result<(), Error> {
    tracing::debug!("setup cli from env");
    let cli = Cli::parse();

    tracing::info!("load config and instance mappings file");
    let config = Config::from_file(&cli.config).context(LoadConfigSnafu)?;
    let instances = Instances::from_file(&cli.instances).context(LoadInstancesSnafu)?;

    match cli.command {
        cli::Command::Profile(profile_arguments) => {
            if profile_arguments.check_test_definitions {
                config
                    .validate_test_options(
                        &profile_arguments.profile,
                        &profile_arguments.test_definitions,
                    )
                    .context(ValidateTestOptionsSnafu)?;
            }

            tracing::info!("determine parameters");
            let parameters = config
                .determine_parameters_by_profile(&profile_arguments.profile, &instances)
                .context(DetermineParametersSnafu)?;

            let parameters = parameters.to_string();

            // Optionally write the expanded parameters into an output file
            if let Some(output_path) = cli.output {
                write_to_output_file(output_path, &parameters)?;
            }

            // Output the expanded parameters to stdout if no --quiet flag was passed
            if !cli.quiet {
                print!("{parameters}");
            }
        }
        cli::Command::Custom(custom_arguments) => {
            let parameters = config
                .determine_parameters_by_runner(
                    &custom_arguments.runner,
                    &instances,
                    custom_arguments.parallel,
                    custom_arguments.test_suite.as_deref(),
                    custom_arguments.test.as_deref(),
                )
                .context(DetermineParametersSnafu)?;

            let parameters = parameters.to_string();

            // Optionally write the expanded parameters into an output file
            if let Some(output_path) = cli.output {
                write_to_output_file(output_path, &parameters)?;
            }

            // Output the expanded parameters to stdout if no --quiet flag was passed
            if !cli.quiet {
                print!("{parameters}");
            }
        }
    }

    Ok(())
}

fn write_to_output_file(output_path: PathBuf, parameters: &str) -> Result<(), Error> {
    let parameters = parameters.to_string();
    tracing::info!(output_path = %output_path.display(), "write parameters to output file");

    let mut file = OpenOptions::new()
        .append(true)
        .open(output_path)
        .context(WriteOutputFileSnafu)?;

    file.write(parameters.as_bytes())
        .context(WriteOutputFileSnafu)?;

    Ok(())
}

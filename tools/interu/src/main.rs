use std::{fs::OpenOptions, io::Write as _};

use snafu::{report, ResultExt, Snafu};

use crate::{cli::Cli, config::Config, instances::Instances};

mod cli;
mod config;
mod instances;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to read config"))]
    ReadConfig { source: config::Error },

    #[snafu(display("failed to read instances file"))]
    ReadInstances { source: instances::Error },

    #[snafu(display("failed to write to output file"))]
    WriteOutputFile { source: std::io::Error },
}

#[report]
fn main() -> Result<(), Error> {
    let cli: Cli = argh::from_env();

    let config = Config::from_file(&cli.config).context(ReadConfigSnafu)?;
    let instances = Instances::from_file(&cli.instances).context(ReadInstancesSnafu)?;

    let parameters = config
        .determine_parameters(&cli.profile, instances)
        .unwrap();

    let parameters = parameters.to_string();

    if let Some(output_path) = cli.output {
        let mut file = OpenOptions::new()
            .append(true)
            .open(output_path)
            .context(WriteOutputFileSnafu)?;

        file.write(parameters.as_bytes())
            .context(WriteOutputFileSnafu)?;
    }

    if !cli.quiet {
        print!("{parameters}");
    }

    Ok(())
}

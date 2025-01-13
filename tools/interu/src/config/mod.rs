use std::{
    collections::BTreeMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use rand::{distributions::WeightedIndex, prelude::Distribution as _, thread_rng};
use serde::Deserialize;
use snafu::{OptionExt, ResultExt, Snafu};
use tracing::instrument;

use crate::{
    config::{
        profile::{Profile, StrategyValidationError, TestOptions, TestRun},
        runner::{
            ConvertNodeGroupError, Distribution, ReplicatedNodeGroup, Runner, RunnerValidationError,
        },
    },
    instances::Instances,
};

pub mod profile;
pub mod runner;
pub mod test;

/// Errors which can be encountered when reading and validating the config file.
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read config file at {path}", path = path.display()))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to deserialize config file at {path} as yaml", path = path.display()))]
    Deserialize {
        source: serde_yaml::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to validate config file at {path}", path = path.display()))]
    Validate {
        source: ValidationError,
        path: PathBuf,
    },

    #[snafu(display("failed to validate test options"))]
    ValidateTestOptions { source: StrategyValidationError },

    #[snafu(display("failed to find profile named {profile_name:?}"))]
    UnknownProfileName { profile_name: String },

    #[snafu(display("failed to convert node-group to Replicated format"))]
    ConvertNodeGroup { source: ConvertNodeGroupError },
}

/// Errors which can be encountered during config file validation.
#[derive(Debug, Snafu)]
pub enum ValidationError {
    #[snafu(display("encountered invalid runner config"))]
    InvalidRunnerConfig { source: RunnerValidationError },

    #[snafu(display("encountered invalid profile config"))]
    InvalidProfileConfig { source: StrategyValidationError },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub runners: BTreeMap<String, Runner>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub profiles: BTreeMap<String, Profile>,
}

impl Config {
    /// Read, deserialize and validate a config from a file located at `path`.
    #[instrument(name = "load_config_from_file", skip(path), fields(path = %path.as_ref().display()))]
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(&path).context(ReadFileSnafu {
            path: path.as_ref(),
        })?;

        tracing::debug!("deserialize file contents");
        let config: Self = serde_yaml::from_str(&contents).context(DeserializeSnafu {
            path: path.as_ref(),
        })?;

        config.validate().context(ValidateSnafu {
            path: path.as_ref(),
        })?;
        Ok(config)
    }

    pub fn get_profile(&self, profile_name: &String) -> Result<&Profile, Error> {
        self.profiles
            .get(profile_name)
            .context(UnknownProfileNameSnafu { profile_name })
    }

    pub fn validate_test_options(&self, profile_name: &String) -> Result<(), Error> {
        self.get_profile(profile_name)?
            .validate_test_options(&profile_name)
            .context(ValidateTestOptionsSnafu)
    }

    /// Determines the final expanded parameters based on the provided profile.
    pub fn determine_parameters<'a>(
        &'a self,
        profile_name: &String,
        instances: &'a Instances,
    ) -> Result<Parameters<'a>, Error> {
        // First, lookup the profile by name. Error if the profile does't exist.
        let profile = self.get_profile(profile_name)?;

        // Next, lookup the runner ref based on the profile strategy
        let runner_ref = match &profile.strategy {
            profile::Strategy::Weighted(options) => {
                let weights: Vec<_> = options.weights.iter().map(|w| w.weight).collect();
                let random_distribution = WeightedIndex::new(weights).unwrap();
                let mut rng = thread_rng();

                let index = random_distribution.sample(&mut rng);
                let weight = options.weights.get(index).expect("always valid index");

                &weight.runner
            }
            profile::Strategy::UseRunner(options) => &options.runner,
        };

        // Get the runner based on the runner ref
        let runner = self.runners.get(runner_ref).unwrap();

        // Get test options
        let TestOptions {
            parallelism,
            test_run,
            test_parameter,
        } = profile.strategy.get_test_options();

        // Convert our node groups to replicated node groups
        let node_groups = runner
            .node_groups
            .clone()
            .into_iter()
            .map(|ng| ReplicatedNodeGroup::try_from(ng, instances, &runner.platform.distribution))
            .collect::<Result<Vec<_>, ConvertNodeGroupError>>()
            .context(ConvertNodeGroupSnafu)?;

        Ok(Parameters {
            kubernetes_distribution: &runner.platform.distribution,
            kubernetes_version: &runner.platform.version,
            test_parallelism: *parallelism,
            cluster_ttl: &runner.ttl,
            test_parameter,
            node_groups,
            test_run,
        })
    }

    #[instrument(name = "validate_config", skip(self))]
    fn validate(&self) -> Result<(), ValidationError> {
        for (runner_name, runner) in &self.runners {
            tracing::debug!(runner_name, "validate runner");

            runner
                .validate(runner_name)
                .context(InvalidRunnerConfigSnafu)?;
        }

        for (profile_name, profile) in &self.profiles {
            tracing::debug!(profile_name, "validate profile");

            profile
                .validate(profile_name, &self.runners)
                .context(InvalidProfileConfigSnafu)?;
        }

        Ok(())
    }
}

/// Parameters which will be expanded into environment variables via the [`Display`] implementation.
#[derive(Debug)]
pub struct Parameters<'a> {
    /// Selected Kubernetes distribution available on Replicated.
    kubernetes_distribution: &'a Distribution,

    /// Kubernetes version used for the cluster.
    kubernetes_version: &'a str,

    /// Maximum TTL of the cluster.
    cluster_ttl: &'a str,

    /// Node groups which will be created in the cluster.
    node_groups: Vec<ReplicatedNodeGroup<'a>>,

    /// Number of tests which get run in parallel.
    test_parallelism: usize,

    /// Optional test parameter passed to `test_run`.
    test_parameter: &'a str,

    /// Set of tests to run.
    test_run: &'a TestRun,
}

impl<'a> Display for Parameters<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Destructure all fields so that any additional parameters are handled here.
        // DO NOT USE `..`.
        let Self {
            kubernetes_distribution,
            kubernetes_version,
            cluster_ttl,
            node_groups,
            test_parallelism,
            test_parameter,
            test_run,
        } = self;

        #[rustfmt::skip] // Skip formatting because otherwise the next line would be split into three lines.
        write!(f, "INTERU_KUBERNETES_DISTRIBUTION={kubernetes_distribution}\n")?;
        write!(f, "INTERU_KUBERNETES_VERSION={kubernetes_version}\n")?;
        write!(f, "INTERU_TEST_PARALLELISM={test_parallelism}\n")?;
        write!(f, "INTERU_TEST_PARAMETER={test_parameter}\n")?;
        write!(f, "INTERU_CLUSTER_TTL={cluster_ttl}\n")?;
        write!(f, "INTERU_TEST_RUN={test_run}\n")?;

        // See: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#multiline-strings
        let node_groups = serde_yaml::to_string(&node_groups).expect("must be serializable");
        write!(f, "INTERU_NODE_GROUPS<<EOF\n{node_groups}\nEOF\n")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn deserialize(#[files("fixtures/interu.yaml")] path: PathBuf) {
        let _ = Config::from_file(path).unwrap();
    }
}

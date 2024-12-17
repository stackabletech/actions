use std::{collections::BTreeMap, fmt::Display, path::Path};

use rand::{distributions::WeightedIndex, prelude::Distribution as _, thread_rng};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::{
    config::{
        profile::{Profile, StrategyValidationError, TestRun},
        runner::{ConvertNodeGroupError, Distribution, ReplicatedNodeGroup, Runner},
    },
    instances::Instances,
};

pub mod profile;
pub mod runner;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read file"))]
    ReadFile { source: std::io::Error },

    #[snafu(display("failed to deserialize file"))]
    Deserialize { source: serde_yaml::Error },

    #[snafu(display("failed to validate file"))]
    Validate { source: ValidationError },

    #[snafu(display("failed to find profile named {profile_name:?}"))]
    UnknownProfileName { profile_name: String },

    #[snafu(display("failed to convert node-group to Replicated format"))]
    ConvertNodeGroup { source: ConvertNodeGroupError },
}

#[derive(Debug, Snafu)]
pub enum ValidationError {
    #[snafu(display("invalid profile config"))]
    InvalidProfileConfig { source: StrategyValidationError },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub runners: BTreeMap<String, Runner>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub profiles: BTreeMap<String, Profile>,
}

impl Config {
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).context(ReadFileSnafu)?;
        let config: Config = serde_yaml::from_str(&contents).context(DeserializeSnafu)?;
        config.validate().context(ValidateSnafu)?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        for (profile_name, profile) in &self.profiles {
            profile
                .validate(profile_name, &self.runners)
                .context(InvalidProfileConfigSnafu)?;
        }

        Ok(())
    }

    pub fn determine_parameters(
        &self,
        profile_name: &String,
        instances: Instances,
    ) -> Result<Parameters, Error> {
        // First, lookup the profile by name. Error if the profile does't exist.
        let profile = self
            .profiles
            .get(profile_name)
            .context(UnknownProfileNameSnafu { profile_name })?;

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
        let (test_parallelism, test_run, test_parameter) = profile.strategy.get_test_options();

        // Convert our node groups to replicated node groups
        let node_groups = runner
            .node_groups
            .clone()
            .into_iter()
            .map(|ng| ReplicatedNodeGroup::try_from(ng, &instances, &runner.platform.distribution))
            .collect::<Result<Vec<_>, ConvertNodeGroupError>>()
            .context(ConvertNodeGroupSnafu)?;

        Ok(Parameters {
            kubernetes_distribution: runner.platform.distribution.clone(),
            kubernetes_version: runner.platform.version.clone(),
            test_parameter: test_parameter.to_owned(),
            test_run: test_run.clone(),
            test_parallelism,
            node_groups,
        })
    }
}

#[derive(Debug)]
pub struct Parameters {
    kubernetes_distribution: Distribution,
    kubernetes_version: String,

    node_groups: Vec<ReplicatedNodeGroup>,

    test_parallelism: usize,
    test_parameter: String,
    test_run: TestRun,
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            kubernetes_distribution,
            kubernetes_version,
            node_groups,
            test_parallelism,
            test_parameter,
            test_run,
        } = self;

        write!(f, "KUBERNETES_DISTRIBUTION={kubernetes_distribution}\n")?;
        write!(f, "KUBERNETES_VERSION={kubernetes_version}\n")?;

        // See: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#multiline-strings
        write!(
            f,
            "NODE_GROUPS<<EOF\n{node_groups}\nEOF\n",
            node_groups = serde_yaml::to_string(&node_groups).expect("must be serializable")
        )?;

        write!(f, "TEST_PARALLELISM={test_parallelism}\n")?;
        write!(f, "TEST_PARAMETER={test_parameter}\n")?;
        write!(f, "TEST_RUN={test_run}\n")
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn serde(#[files("../../run-integration-test/config.example.yml")] path: PathBuf) {
        let content = std::fs::read_to_string(path).unwrap();
        let config: Config = serde_yaml::from_str(&content).unwrap();
        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("{yaml}");
    }
}

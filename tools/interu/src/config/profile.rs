use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

use crate::config::runner::Runner;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    #[serde(flatten)]
    pub strategy: Strategy,
}

impl Profile {
    pub fn validate(
        &self,
        profile_name: &str,
        runners: &BTreeMap<String, Runner>,
    ) -> Result<(), StrategyValidationError> {
        self.strategy.validate(profile_name, runners)
    }
}

#[derive(Debug, Snafu)]
pub enum StrategyValidationError {
    #[snafu(display("runner {runner_ref:?} referenced in {at} is not defined"))]
    InvalidRunnerReference { at: String, runner_ref: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "strategy", rename_all = "kebab-case")]
pub enum Strategy {
    Weighted(WeightedOptions),
    UseRunner(UseRunnerOptions),
}

impl Strategy {
    pub fn validate(
        &self,
        profile_name: &str,
        runners: &BTreeMap<String, Runner>,
    ) -> Result<(), StrategyValidationError> {
        match &self {
            Strategy::Weighted(weighted_options) => {
                weighted_options.validate(profile_name, runners)
            }
            Strategy::UseRunner(use_runner_options) => {
                use_runner_options.validate(profile_name, runners)
            }
        }
    }

    pub fn get_test_options(&self) -> (usize, &TestRun, &str) {
        match self {
            Strategy::Weighted(options) => (
                options.options.parallelism,
                &options.options.test_run,
                options.options.test_parameter.as_str(),
            ),
            Strategy::UseRunner(options) => (
                options.options.parallelism,
                &options.options.test_run,
                options.options.test_parameter.as_str(),
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WeightedOptions {
    pub weights: Vec<Weight>,
    pub options: TestOptions,
}

impl WeightedOptions {
    pub fn validate(
        &self,
        profile_name: &str,
        runners: &BTreeMap<String, Runner>,
    ) -> Result<(), StrategyValidationError> {
        for weight in &self.weights {
            if !runners.contains_key(&weight.runner) {
                return InvalidRunnerReferenceSnafu {
                    at: format!(
                        "profile.{profile_name}.weights.{weight}",
                        weight = weight.weight
                    ),
                    runner_ref: weight.runner.clone(),
                }
                .fail();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Weight {
    pub weight: usize,
    pub runner: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UseRunnerOptions {
    pub runner: String,
    pub options: TestOptions,
}

impl UseRunnerOptions {
    pub fn validate(
        &self,
        profile_name: &str,
        runners: &BTreeMap<String, Runner>,
    ) -> Result<(), StrategyValidationError> {
        if !runners.contains_key(&self.runner) {
            return InvalidRunnerReferenceSnafu {
                at: format!("profile.{profile_name}.runner"),
                runner_ref: self.runner.clone(),
            }
            .fail();
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TestOptions {
    pub parallelism: usize,

    #[serde(default, with = "serde_yaml::with::singleton_map")]
    pub test_run: TestRun,

    #[serde(default)]
    pub test_parameter: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum TestRun {
    TestSuite,
    Test,

    #[default]
    All,
}

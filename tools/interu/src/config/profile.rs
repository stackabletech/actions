use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use snafu::{ensure, Snafu};

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

    #[snafu(display(
        r#"strategy {at} must define two or more weights, or use the "use-runner" strategy instead"#
    ))]
    WeightsCount { at: String },

    #[snafu(display("strategy {at} references a runner already referenced by another weight"))]
    NonUniqueWeightRunner { at: String },
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
        ensure!(
            self.weights.len() > 1,
            WeightsCountSnafu {
                at: format!("profiles.{profile_name}.weights")
            }
        );

        for (i, weight) in self.weights.iter().enumerate() {
            ensure!(
                runners.contains_key(&weight.runner),
                InvalidRunnerReferenceSnafu {
                    at: format!(
                        "profiles.{profile_name}.weights.{weight}",
                        weight = weight.weight
                    ),
                    runner_ref: weight.runner.clone(),
                }
            );

            // Ensure that all weights use unique runners
            let before = &mut self.weights[..i].iter().map(|w| w.runner.as_str());
            if before.len() > 0 && before.any(|n| n == weight.runner) {
                return NonUniqueWeightRunnerSnafu {
                    at: format!(
                        "profiles.{profile_name}.weights[{index}].runner",
                        index = i - 1
                    ),
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

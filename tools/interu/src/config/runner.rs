use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use snafu::{ensure, OptionExt, ResultExt as _, Snafu};

use crate::instances::Instances;

#[derive(Debug, Snafu)]
pub enum RunnerValidationError {
    #[snafu(display("{at} must contain at least one node group"))]
    ZeroNodeGroups { at: String },
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Runner {
    #[serde_as(as = "DisplayFromStr")]
    pub platform: PlatformPair,

    // TODO (@Techassi): Allow some kind of inheritance here (size, disk, ttl, etc...)
    /// The time-to-live of the cluster.
    pub ttl: String,

    /// Define one or more node groups.
    pub node_groups: Vec<NodeGroup>,
}

impl Runner {
    pub fn validate(&self, runner_name: &str) -> Result<(), RunnerValidationError> {
        ensure!(
            !self.node_groups.is_empty(),
            ZeroNodeGroupsSnafu {
                at: format!("runners.{runner_name}")
            }
        );

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum ParsePlatformTripleError {
    #[snafu(display("invalid format, expected pair separated by dashes"))]
    InvalidFormat,

    #[snafu(display("failed to parse distribution"))]
    ParseDistribution { source: strum::ParseError },
}

#[derive(Debug)]
pub struct PlatformPair {
    pub distribution: Distribution,
    // Ideally we want to use SemVer here, but cloud vendors make stupid
    // decisions and just use major.minor.
    pub version: String,
}

impl FromStr for PlatformPair {
    type Err = ParsePlatformTripleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (distribution, version) = s.split_once('-').context(InvalidFormatSnafu)?;
        let distribution = Distribution::from_str(distribution).context(ParseDistributionSnafu)?;

        Ok(PlatformPair {
            version: version.to_owned(),
            distribution,
        })
    }
}

impl Display for PlatformPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{distribution}-{version}",
            distribution = self.distribution,
            version = self.version,
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    Eks,
    Gke,
    Aks,
    Kind,
    K3s,
    Rke2,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Architecture {
    Amd64,
    Arm64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NodeGroup {
    name: String,

    #[serde(rename = "arch")]
    architecture: Architecture,

    nodes: usize,
    size: Size,
    disk: usize,
}

#[derive(Debug, Snafu)]
pub enum ConvertNodeGroupError {
    #[snafu(display("unknown distribution {distribution}"))]
    UnknownDistribution { distribution: Distribution },

    #[snafu(display("unknown architecture {architecture}"))]
    UnknownArchitecture { architecture: Architecture },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReplicatedNodeGroup<'a> {
    instance_type: &'a str,
    name: String,
    nodes: usize,
    disk: usize,
}

impl<'a> ReplicatedNodeGroup<'a> {
    pub fn try_from(
        node_group: NodeGroup,
        instances: &'a Instances,
        distribution: &Distribution,
    ) -> Result<Self, ConvertNodeGroupError> {
        let architectures =
            instances
                .get(distribution)
                .with_context(|| UnknownDistributionSnafu {
                    distribution: distribution.clone(),
                })?;

        let sizes = architectures
            .get(&node_group.architecture)
            .with_context(|| UnknownArchitectureSnafu {
                architecture: node_group.architecture.clone(),
            })?;

        let instance_type = match node_group.size {
            Size::Small => sizes.small.as_str(),
            Size::Medium => sizes.medium.as_str(),
            Size::Large => sizes.large.as_str(),
        };

        Ok(Self {
            instance_type,
            name: node_group.name,
            nodes: node_group.nodes,
            disk: node_group.disk,
        })
    }
}

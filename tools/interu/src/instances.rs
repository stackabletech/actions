use std::{collections::HashMap, ops::Deref, path::Path};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu};
use tracing::instrument;

use crate::config::runner::{Architecture, Distribution};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read file"))]
    ReadFile { source: std::io::Error },

    #[snafu(display("failed to deserialize file"))]
    Deserialize { source: serde_yaml::Error },
}

/// Contains distribution to architecture mappings.
#[derive(Debug, Deserialize)]
pub struct Instances(HashMap<Distribution, Architectures>);

impl Deref for Instances {
    type Target = HashMap<Distribution, Architectures>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Contains architecture to size mappings.
#[derive(Debug, Deserialize)]
pub struct Architectures(HashMap<Architecture, Sizes>);

impl Deref for Architectures {
    type Target = HashMap<Architecture, Sizes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Instances {
    /// Reads and deserializes the mappings from a file located at `path`.
    #[instrument(name = "load_instance_mappings_from_file", skip(path), fields(path = %path.as_ref().display()))]
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).context(ReadFileSnafu)?;

        tracing::debug!("deserialize file contents");
        let config: Instances = serde_yaml::from_str(&contents).context(DeserializeSnafu)?;

        Ok(config)
    }
}

// NOTE (@Techassi): Can we somehow re-use the size enum here?
/// Contains size to instance name mappings.
///
/// This mapping translates our sizes into instance names. Every cloud vendor uses a different
/// scheme to name their instances. It removes the need to specify the exact name and instead
/// enables the use of a simple size: small, medium, and large.
#[derive(Debug, Deserialize)]
pub struct Sizes {
    pub small: String,
    pub medium: String,
    pub large: String,
}

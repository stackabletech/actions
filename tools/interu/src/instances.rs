use std::{collections::HashMap, ops::Deref, path::Path};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu};

use crate::config::runner::{Architecture, Distribution};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read file"))]
    ReadFile { source: std::io::Error },

    #[snafu(display("failed to deserialize file"))]
    Deserialize { source: serde_yaml::Error },
}

#[derive(Debug, Deserialize)]
pub struct Instances(HashMap<Distribution, Architectures>);

impl Deref for Instances {
    type Target = HashMap<Distribution, Architectures>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Architectures(HashMap<Architecture, Sizes>);

impl Deref for Architectures {
    type Target = HashMap<Architecture, Sizes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Instances {
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(path).context(ReadFileSnafu)?;
        let config: Instances = serde_yaml::from_str(&contents).context(DeserializeSnafu)?;

        Ok(config)
    }
}

// NOTE (@Techassi): Can we somehow re-use the size enum here?
#[derive(Debug, Deserialize)]
pub struct Sizes {
    pub small: String,
    pub medium: String,
    pub large: String,
}

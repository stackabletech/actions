use std::path::{Path, PathBuf};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu};
use tracing::instrument;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read test definition file at {path}", path = path.display()))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to deserialize test definition file at {path} as yaml", path = path.display()))]
    Deserialize {
        source: serde_yaml::Error,
        path: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
pub struct TestDefinition {
    #[serde(default)]
    pub tests: Vec<Test>,

    #[serde(default)]
    pub suites: Vec<Suite>,
}

impl TestDefinition {
    /// Read and deserialize test definition from a file located at `path`.
    #[instrument(name = "load_test_definition_from_file", skip(path), fields(path = %path.as_ref().display()))]
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(&path).context(ReadFileSnafu {
            path: path.as_ref(),
        })?;

        tracing::debug!("deserialize file contents");
        let test_definition: Self = serde_yaml::from_str(&contents).context(DeserializeSnafu {
            path: path.as_ref(),
        })?;

        Ok(test_definition)
    }
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Suite {
    pub name: String,
}

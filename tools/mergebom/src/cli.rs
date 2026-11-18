use std::path::PathBuf;

use clap::Parser;
use cyclonedx_bom::models::bom::SpecVersion;

/// Finalizes the syft SBOM of a container image into the published SBOM documents.
///
/// The input is the CycloneDX SBOM produced by `syft scan`. mergebom merges duplicate
/// components, completes the dependency graph, sets the metadata of the image (root component)
/// and writes the result as CycloneDX and as SPDX 2.3.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Path of the CycloneDX SBOM produced by syft.
    pub input: PathBuf,

    /// Path the finalized CycloneDX SBOM is written to.
    #[arg(long)]
    pub cyclonedx_output: PathBuf,

    /// CycloneDX specification version of the finalized CycloneDX SBOM.
    #[arg(long, default_value_t = SpecVersion::V1_7, value_parser = parse_spec_version)]
    pub cyclonedx_spec_version: SpecVersion,

    /// Path the SPDX 2.3 SBOM is written to.
    #[arg(long)]
    pub spdx_output: PathBuf,

    /// Package URL of the image, set on the root component of both SBOMs.
    #[arg(long)]
    pub image_purl: String,

    /// Description of the image, set on the root component of both SBOMs.
    #[arg(long)]
    pub image_description: Option<String>,

    /// License of the image as an SPDX license identifier or expression.
    ///
    /// Usually read from the `org.opencontainers.image.licenses` label of the image. It must be
    /// valid, otherwise the CycloneDX SBOM would not match its schema.
    #[arg(long, value_parser = parse_license)]
    pub image_license: Option<String>,
}

fn parse_license(input: &str) -> Result<String, String> {
    match spdx::Expression::parse_mode(input, spdx::ParseMode::STRICT) {
        Ok(_) => Ok(input.to_owned()),
        Err(error) => Err(format!(
            "{input:?} is not a valid SPDX license identifier or expression: {error}"
        )),
    }
}

fn parse_spec_version(input: &str) -> Result<SpecVersion, String> {
    match input.parse() {
        Ok(version @ (SpecVersion::V1_5 | SpecVersion::V1_6 | SpecVersion::V1_7)) => Ok(version),
        _ => Err(format!(
            "unsupported CycloneDX specification version {input:?}, expected 1.5, 1.6 or 1.7"
        )),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Apache-2.0", true)]
    #[case("Apache-2.0 OR MIT", true)]
    #[case("LicenseRef-Stackable", true)]
    #[case("Apache 2.0", false)]
    #[case("GPL-2.0", false)]
    #[case("", false)]
    fn validate_image_license(#[case] input: &str, #[case] is_valid: bool) {
        assert_eq!(parse_license(input).is_ok(), is_valid);
    }
}

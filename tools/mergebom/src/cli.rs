use std::{path::PathBuf, str::FromStr};

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
    #[arg(long, value_parser = parse_purl)]
    pub image_purl: String,

    /// Description of the image, set on the root component of both SBOMs.
    #[arg(long)]
    pub image_description: Option<String>,

    /// License of the image as an SPDX license identifier or expression.
    ///
    /// Usually read from the `org.opencontainers.image.licenses` label of the image. It must be
    /// valid, otherwise the CycloneDX SBOM would not match its schema.
    #[arg(long)]
    pub image_license: Option<spdx::Expression>,
}

/// Validates the purl, but keeps it as given.
///
/// [`purl::GenericPurl`] percent-encodes differently when formatting it again (for example
/// `sha256%3A...` becomes `sha256:...`), which would change the purl of the image.
fn parse_purl(input: &str) -> Result<String, purl::ParseError> {
    purl::GenericPurl::<String>::from_str(input).map(|_purl| input.to_owned())
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
    use clap::CommandFactory;
    use rstest::rstest;

    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[rstest]
    #[case("pkg:oci/kafka@sha256%3Aabc?arch=amd64&repository_url=oci.stackable.tech%2Fsdp%2Fkafka")]
    #[case("pkg:oci/kafka")]
    fn keep_valid_image_purl(#[case] input: &str) {
        assert_eq!(parse_purl(input).expect("the purl should be valid"), input);
    }

    #[rstest]
    #[case("")]
    #[case("oci/kafka")]
    #[case("pkg:oci")]
    fn reject_invalid_image_purl(#[case] input: &str) {
        assert!(parse_purl(input).is_err());
    }

    /// Parses the CLI with the given image license.
    fn parse_with_image_license(license: &str) -> Result<Cli, clap::Error> {
        Cli::try_parse_from([
            "mergebom",
            "sbom_raw.json",
            "--cyclonedx-output=sbom.cdx.json",
            "--spdx-output=sbom.spdx.json",
            "--image-purl=pkg:oci/kafka",
            &format!("--image-license={license}"),
        ])
    }

    #[rstest]
    #[case("Apache-2.0")]
    #[case("Apache-2.0 OR MIT")]
    #[case("LicenseRef-Stackable")]
    fn keep_valid_image_license(#[case] input: &str) {
        let cli = parse_with_image_license(input).expect("the license should be valid");
        let license = cli.image_license.expect("the license should be set");
        assert_eq!(license.to_string(), input);
    }

    #[rstest]
    #[case("Apache 2.0")]
    #[case("GPL-2.0")]
    #[case("")]
    fn reject_invalid_image_license(#[case] input: &str) {
        assert!(parse_with_image_license(input).is_err());
    }
}

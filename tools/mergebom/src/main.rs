use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use cyclonedx_bom::prelude::{Bom, SpecVersion};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::{cli::Cli, cyclonedx::ImageMetadata};

mod cli;
mod cyclonedx;
mod license;
mod merge;
mod spdx_document;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to open the input SBOM {path:?}"))]
    OpenInput {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to parse the input SBOM {path:?}"))]
    ParseInput {
        source: cyclonedx_bom::errors::JsonReadError,
        path: PathBuf,
    },

    #[snafu(display("the input SBOM has no root component with a bom-ref"))]
    MissingRootBomRef,

    #[snafu(display("failed to build the SPDX document"))]
    BuildSpdxDocument { source: spdx_document::Error },

    #[snafu(display("failed to create the output file {path:?}"))]
    CreateOutput {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to write the CycloneDX SBOM to {path:?}"))]
    WriteCycloneDx {
        source: cyclonedx_bom::errors::JsonWriteError,
        path: PathBuf,
    },

    #[snafu(display("failed to write the SPDX SBOM to {path:?}"))]
    WriteSpdx {
        source: serde_json::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to flush the output file {path:?}"))]
    FlushOutput {
        source: std::io::Error,
        path: PathBuf,
    },
}

#[snafu::report]
fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let input = File::open(&cli.input).context(OpenInputSnafu { path: &cli.input })?;
    let mut bom = Bom::parse_from_json(BufReader::new(input))
        .context(ParseInputSnafu { path: &cli.input })?;

    let image = ImageMetadata {
        purl: cli.image_purl,
        description: cli.image_description,
        license: cli.image_license,
    };

    finalize(&mut bom, &image)?;

    let spdx_document = spdx_document::build_document(&bom).context(BuildSpdxDocumentSnafu)?;

    write_cyclonedx(bom, cli.cyclonedx_spec_version, &cli.cyclonedx_output)?;
    write_spdx(&spdx_document, &cli.spdx_output)?;

    Ok(())
}

/// Merges the components of the syft SBOM and sets the metadata of the image.
fn finalize(bom: &mut Bom, image: &ImageMetadata) -> Result<(), Error> {
    let root_bom_ref = bom
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.component.as_ref())
        .and_then(|component| component.bom_ref.clone())
        .context(MissingRootBomRefSnafu)?;

    // Merging reads the syft properties, so they are only stripped afterwards.
    merge::merge_components(bom, &root_bom_ref);
    cyclonedx::strip_syft_properties(bom);
    cyclonedx::set_image_metadata(bom, image);

    Ok(())
}

fn write_cyclonedx(bom: Bom, spec_version: SpecVersion, path: &Path) -> Result<(), Error> {
    let file = File::create(path).context(CreateOutputSnafu { path })?;
    let mut writer = BufWriter::new(file);
    bom.output_as_json(&mut writer, spec_version)
        .context(WriteCycloneDxSnafu { path })?;
    writer.flush().context(FlushOutputSnafu { path })
}

fn write_spdx(document: &spdx_document::Document, path: &Path) -> Result<(), Error> {
    let file = File::create(path).context(CreateOutputSnafu { path })?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, document).context(WriteSpdxSnafu { path })?;
    writer.flush().context(FlushOutputSnafu { path })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use cyclonedx_bom::{models::component::Classification, validation::Validate};
    use rstest::{fixture, rstest};

    use super::*;

    /// The syft SBOM of the Kafka image, trimmed to a few components of each kind.
    const SYFT_SBOM: &str = include_str!("../fixtures/kafka-syft.cdx.json");
    const IMAGE_PURL: &str =
        "pkg:oci/kafka@sha256%3Aabc?arch=amd64&repository_url=oci.stackable.tech%2Fsdp%2Fkafka";

    struct Finalized {
        bom: Bom,
        spdx: serde_json::Value,
    }

    #[fixture]
    fn finalized() -> Finalized {
        let mut bom = Bom::parse_from_json(SYFT_SBOM.as_bytes())
            .expect("the fixture should be a valid CycloneDX SBOM");
        let image = ImageMetadata {
            purl: IMAGE_PURL.to_owned(),
            description: Some("Apache Kafka. The Stackable image for Apache Kafka.".to_owned()),
            license: Some("Apache-2.0".to_owned()),
        };

        finalize(&mut bom, &image).expect("the fixture should be finalized");
        let document = spdx_document::build_document(&bom).expect("the SPDX document should build");
        let spdx = serde_json::to_value(document).expect("the SPDX document should serialize");

        Finalized { bom, spdx }
    }

    fn components(bom: &Bom) -> &[cyclonedx_bom::models::component::Component] {
        bom.components
            .as_ref()
            .map(|components| components.0.as_slice())
            .unwrap_or_default()
    }

    #[rstest]
    fn merge_duplicate_purls_into_embedded_sbom_component(finalized: Finalized) {
        let lz4: Vec<_> = components(&finalized.bom)
            .iter()
            .filter(|component| component.name.to_string() == "lz4-java")
            .collect();

        assert_eq!(lz4.len(), 1);
        assert_eq!(
            lz4[0]
                .properties
                .as_ref()
                .and_then(|properties| properties
                    .0
                    .iter()
                    .find(|p| p.name == "syft:package:foundBy"))
                .map(|property| property.value.to_string()),
            Some("sbom-cataloger".to_owned())
        );
        assert_eq!(components(&finalized.bom).len(), 12);
    }

    #[rstest]
    fn every_component_is_reachable_from_the_root(finalized: Finalized) {
        let dependencies = &finalized
            .bom
            .dependencies
            .as_ref()
            .expect("the SBOM should have dependencies")
            .0;
        let children: HashSet<&str> = dependencies
            .iter()
            .flat_map(|dependency| dependency.dependencies.iter().map(String::as_str))
            .collect();

        for component in components(&finalized.bom) {
            let bom_ref = component
                .bom_ref
                .as_deref()
                .expect("components should have a bom-ref");
            assert!(children.contains(bom_ref), "{bom_ref:?} is not reachable");
        }
    }

    #[rstest]
    fn strip_syft_bookkeeping_properties(finalized: Finalized) {
        let property_names: HashSet<String> = components(&finalized.bom)
            .iter()
            .flat_map(|component| component.properties.iter().flat_map(|p| &p.0))
            .map(|property| {
                property
                    .name
                    .chars()
                    .map(|c| if c.is_ascii_digit() { 'N' } else { c })
                    .collect()
            })
            .collect();

        assert_eq!(
            property_names,
            HashSet::from(
                [
                    "syft:package:foundBy",
                    "syft:location:N:path",
                    "syft:metadata:virtualPath",
                ]
                .map(ToOwned::to_owned)
            )
        );
    }

    #[rstest]
    fn rename_operating_system(finalized: Finalized) {
        let operating_system = components(&finalized.bom)
            .iter()
            .find(|component| component.component_type == Classification::OperatingSystem)
            .expect("the SBOM should have an operating system component");

        assert_eq!(operating_system.name.to_string(), "redhat");
    }

    #[rstest]
    fn set_image_metadata_on_root_component(finalized: Finalized) {
        let root = finalized
            .bom
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.component.as_ref())
            .expect("the SBOM should have a root component");

        assert_eq!(
            root.purl.as_ref().map(ToString::to_string),
            Some(IMAGE_PURL.to_owned())
        );
        assert_eq!(
            root.supplier
                .as_ref()
                .and_then(|supplier| supplier.name.as_ref())
                .map(ToString::to_string),
            Some("Stackable GmbH".to_owned())
        );
    }

    #[rstest]
    fn cyclonedx_output_is_valid(
        finalized: Finalized,
        #[values(SpecVersion::V1_5, SpecVersion::V1_6, SpecVersion::V1_7)] version: SpecVersion,
    ) {
        let result = finalized.bom.validate_version(version);
        assert!(result.passed(), "{result:?}");

        let mut output = Vec::new();
        finalized
            .bom
            .output_as_json(&mut output, version)
            .expect("the SBOM should be written");
        let written: serde_json::Value =
            serde_json::from_slice(&output).expect("the output should be JSON");
        assert_eq!(written["specVersion"], version.to_string());
    }

    #[rstest]
    #[case::root("kafka", Some("Organization: Stackable GmbH"))]
    #[case::rpm("glibc", Some("Organization: Red Hat, Inc."))]
    #[case::crate_without_known_supplier("adler2", Some("NOASSERTION"))]
    #[case::jar_without_known_supplier("lz4-java", Some("NOASSERTION"))]
    fn spdx_package_supplier(
        finalized: Finalized,
        #[case] package_name: &str,
        #[case] expected_supplier: Option<&str>,
    ) {
        let package = finalized.spdx["packages"]
            .as_array()
            .expect("the document should have packages")
            .iter()
            .find(|package| package["name"] == package_name)
            .expect("the package should exist");

        assert_eq!(package["supplier"].as_str(), expected_supplier);
    }

    #[rstest]
    fn spdx_root_package_matches_root_component(finalized: Finalized) {
        let root = &finalized.spdx["packages"][0];

        assert_eq!(root["primaryPackagePurpose"], "CONTAINER");
        assert_eq!(root["externalRefs"][0]["referenceLocator"], IMAGE_PURL);
        assert_eq!(
            root["description"],
            "Apache Kafka. The Stackable image for Apache Kafka."
        );
        assert_eq!(root["licenseDeclared"], "Apache-2.0");
    }

    #[rstest]
    fn spdx_relationships_reference_existing_elements(finalized: Finalized) {
        let mut ids: HashSet<&str> = finalized.spdx["packages"]
            .as_array()
            .expect("the document should have packages")
            .iter()
            .filter_map(|package| package["SPDXID"].as_str())
            .collect();
        ids.insert("SPDXRef-DOCUMENT");

        for relationship in finalized.spdx["relationships"]
            .as_array()
            .expect("the document should have relationships")
        {
            for field in ["spdxElementId", "relatedSpdxElement"] {
                let id = relationship[field]
                    .as_str()
                    .expect("the field should be a string");
                assert!(ids.contains(id), "unknown SPDX ID {id:?}");
            }
        }
    }

    #[rstest]
    fn spdx_license_refs_are_declared(finalized: Finalized) {
        let declared: HashSet<&str> = finalized.spdx["hasExtractedLicensingInfos"]
            .as_array()
            .map(|infos| {
                infos
                    .iter()
                    .filter_map(|info| info["licenseId"].as_str())
                    .collect()
            })
            .unwrap_or_default();

        for package in finalized.spdx["packages"]
            .as_array()
            .expect("the document should have packages")
        {
            let license = package["licenseDeclared"]
                .as_str()
                .expect("the license should be set");
            for token in license.split(|c: char| c.is_whitespace() || c == '(' || c == ')') {
                if token.starts_with("LicenseRef-") {
                    assert!(declared.contains(token), "undeclared {token:?}");
                }
            }
        }
    }
}

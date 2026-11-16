//! Builds an SPDX 2.3 document from the finalized CycloneDX SBOM.
//!
//! This replaces `syft convert`, which copies the supplier of the root component onto every
//! package, drops the root component's metadata and emits deprecated license IDs.

use std::collections::{BTreeMap, HashSet};

use cyclonedx_bom::{
    external_models::uri::Purl,
    models::{
        component::{Classification, Component},
        tool::Tools,
    },
    prelude::Bom,
};
use serde::Serialize;
use snafu::{OptionExt, Snafu};

use crate::{
    cyclonedx::{ORGANIZATION_EMAIL, ORGANIZATION_NAME},
    license::{declared_license, sanitize_id, ExtractedLicenses, NOASSERTION},
    merge::{property, FOUND_BY_PROPERTY},
};

const DOCUMENT_ID: &str = "SPDXRef-DOCUMENT";
const NAMESPACE_BASE_URL: &str = "https://stackable.tech/spdxdocs";

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("the CycloneDX SBOM has no root component"))]
    RootComponent,

    #[snafu(display("the CycloneDX SBOM has no timestamp"))]
    Timestamp,

    #[snafu(display("the CycloneDX SBOM has no serial number"))]
    SerialNumber,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub spdx_version: &'static str,
    pub data_license: &'static str,
    #[serde(rename = "SPDXID")]
    pub spdx_id: &'static str,
    pub name: String,
    pub document_namespace: String,
    pub creation_info: CreationInfo,
    pub packages: Vec<Package>,
    pub relationships: Vec<Relationship>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_extracted_licensing_infos: Vec<ExtractedLicensingInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationInfo {
    pub created: String,
    pub creators: Vec<String>,
    pub license_list_version: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "SPDXID")]
    pub spdx_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<String>,
    pub supplier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,
    pub download_location: &'static str,
    pub files_analyzed: bool,
    pub license_concluded: &'static str,
    pub license_declared: String,
    pub copyright_text: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub external_refs: Vec<ExternalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_package_purpose: Option<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalRef {
    pub reference_category: &'static str,
    pub reference_type: &'static str,
    pub reference_locator: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub spdx_element_id: String,
    pub relationship_type: &'static str,
    pub related_spdx_element: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedLicensingInfo {
    pub license_id: String,
    pub name: String,
    pub extracted_text: &'static str,
}

/// Builds the SPDX document. The CycloneDX SBOM must already be merged and finalized.
///
/// The metadata of the image is read from the root component, so both documents describe the
/// image the same way.
pub fn build_document(bom: &Bom) -> Result<Document, Error> {
    let metadata = bom.metadata.as_ref().context(RootComponentSnafu)?;
    let root = metadata.component.as_ref().context(RootComponentSnafu)?;
    let created = metadata.timestamp.as_ref().context(TimestampSnafu)?;
    let serial_number = bom.serial_number.as_ref().context(SerialNumberSnafu)?;

    let root_name = root.name.to_string();
    let root_version = root.version.as_ref().map(ToString::to_string);
    let root_spdx_id = format!("SPDXRef-DocumentRoot-Image-{}", sanitize_id(&root_name));

    let mut extracted = ExtractedLicenses::default();
    let mut packages = vec![root_package(root, &root_spdx_id, &mut extracted)];

    // Maps bom-refs to SPDX IDs, for the relationships.
    let mut spdx_ids = BTreeMap::<String, String>::new();
    let mut used_spdx_ids = HashSet::from([root_spdx_id.clone()]);
    if let Some(bom_ref) = &root.bom_ref {
        spdx_ids.insert(bom_ref.clone(), root_spdx_id.clone());
    }

    let mut relationships = vec![Relationship {
        spdx_element_id: DOCUMENT_ID.to_owned(),
        relationship_type: "DESCRIBES",
        related_spdx_element: root_spdx_id.clone(),
    }];

    for component in bom.components.iter().flat_map(|components| &components.0) {
        let spdx_id = unique_spdx_id(component, &mut used_spdx_ids);
        if let Some(bom_ref) = &component.bom_ref {
            spdx_ids.insert(bom_ref.clone(), spdx_id.clone());
        }

        relationships.push(Relationship {
            spdx_element_id: root_spdx_id.clone(),
            relationship_type: "CONTAINS",
            related_spdx_element: spdx_id.clone(),
        });
        packages.push(component_package(component, spdx_id, &mut extracted));
    }

    // The root component is connected to every package with CONTAINS above. Its dependsOn entries
    // in CycloneDX only exist to give the graph a single root, so they are not repeated here.
    let root_bom_ref = root.bom_ref.as_deref();
    for dependency in bom
        .dependencies
        .iter()
        .flat_map(|dependencies| &dependencies.0)
    {
        if Some(dependency.dependency_ref.as_str()) == root_bom_ref {
            continue;
        }
        let Some(element) = spdx_ids.get(&dependency.dependency_ref) else {
            continue;
        };
        for dependency_ref in &dependency.dependencies {
            if let Some(related) = spdx_ids.get(dependency_ref) {
                relationships.push(Relationship {
                    spdx_element_id: element.clone(),
                    relationship_type: "DEPENDS_ON",
                    related_spdx_element: related.clone(),
                });
            }
        }
    }

    let uuid = serial_number.to_string();
    let uuid = uuid.strip_prefix("urn:uuid:").unwrap_or(&uuid);
    let document_namespace = format!(
        "{NAMESPACE_BASE_URL}/{name}-{version}-{uuid}",
        name = sanitize_id(&root_name),
        version = sanitize_id(root_version.as_deref().unwrap_or("unknown")),
    );

    let mut creators = vec![
        format!("Organization: {ORGANIZATION_NAME} ({ORGANIZATION_EMAIL})"),
        format!(
            "Tool: {name}-{version}",
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
        ),
    ];
    creators.extend(tool_creators(bom));

    Ok(Document {
        spdx_version: "SPDX-2.3",
        data_license: "CC0-1.0",
        spdx_id: DOCUMENT_ID,
        name: root_name,
        document_namespace,
        creation_info: CreationInfo {
            created: created.to_string(),
            creators,
            license_list_version: license_list_version(),
        },
        packages,
        relationships,
        has_extracted_licensing_infos: extracted
            .0
            .into_iter()
            .map(|(license_id, name)| ExtractedLicensingInfo {
                license_id,
                name,
                extracted_text: NOASSERTION,
            })
            .collect(),
    })
}

fn root_package(root: &Component, spdx_id: &str, extracted: &mut ExtractedLicenses) -> Package {
    let organization = format!("Organization: {ORGANIZATION_NAME}");

    Package {
        spdx_id: spdx_id.to_owned(),
        name: root.name.to_string(),
        version_info: root.version.as_ref().map(ToString::to_string),
        supplier: organization.clone(),
        originator: Some(organization),
        download_location: NOASSERTION,
        files_analyzed: false,
        license_concluded: NOASSERTION,
        license_declared: declared_license(root.licenses.as_ref(), extracted),
        copyright_text: NOASSERTION,
        description: root.description.as_ref().map(ToString::to_string),
        source_info: None,
        external_refs: root.purl.iter().map(purl_reference).collect(),
        primary_package_purpose: Some("CONTAINER"),
    }
}

fn component_package(
    component: &Component,
    spdx_id: String,
    extracted: &mut ExtractedLicenses,
) -> Package {
    let mut external_refs = Vec::new();
    if let Some(cpe) = &component.cpe {
        let cpe = cpe.to_string();
        let reference_type = if cpe.starts_with("cpe:2.3:") {
            "cpe23Type"
        } else {
            "cpe22Type"
        };
        external_refs.push(ExternalRef {
            reference_category: "SECURITY",
            reference_type,
            reference_locator: cpe,
        });
    }
    external_refs.extend(component.purl.iter().map(purl_reference));

    Package {
        spdx_id,
        name: component.name.to_string(),
        version_info: component.version.as_ref().map(ToString::to_string),
        supplier: supplier(component).unwrap_or_else(|| NOASSERTION.to_owned()),
        originator: None,
        download_location: NOASSERTION,
        files_analyzed: false,
        license_concluded: NOASSERTION,
        license_declared: declared_license(component.licenses.as_ref(), extracted),
        copyright_text: NOASSERTION,
        description: component.description.as_ref().map(ToString::to_string),
        source_info: source_info(component),
        external_refs,
        primary_package_purpose: primary_package_purpose(&component.component_type),
    }
}

fn purl_reference(purl: &Purl) -> ExternalRef {
    ExternalRef {
        reference_category: "PACKAGE-MANAGER",
        reference_type: "purl",
        reference_locator: purl.to_string(),
    }
}

/// Returns the supplier of a component, if it is known.
///
/// An explicit CycloneDX supplier is used as is. For RPMs, syft records the vendor (e.g.
/// "Red Hat, Inc.") as publisher, which is the distribution source of the package. Everything
/// else is unknown: the supplier of the image is not the supplier of its contents.
fn supplier(component: &Component) -> Option<String> {
    if let Some(name) = component
        .supplier
        .as_ref()
        .and_then(|supplier| supplier.name.as_ref())
    {
        return Some(format!("Organization: {name}"));
    }

    let is_rpm = component
        .purl
        .as_ref()
        .is_some_and(|purl| purl.as_ref().starts_with("pkg:rpm/"));
    match (&component.publisher, is_rpm) {
        (Some(publisher), true) if !publisher.to_string().trim().is_empty() => {
            Some(format!("Organization: {publisher}"))
        }
        _ => None,
    }
}

fn source_info(component: &Component) -> Option<String> {
    let found_by = property(component, FOUND_BY_PROPERTY)?;
    let paths: Vec<&str> = component
        .properties
        .iter()
        .flat_map(|properties| &properties.0)
        .filter(|property| {
            property.name.starts_with("syft:location:") && property.name.ends_with(":path")
        })
        .map(|property| property.value.as_ref())
        .collect();

    Some(if paths.is_empty() {
        format!("found by {found_by}")
    } else {
        format!("found by {found_by} at {}", paths.join(", "))
    })
}

fn primary_package_purpose(classification: &Classification) -> Option<&'static str> {
    Some(match classification {
        Classification::Application => "APPLICATION",
        Classification::Framework => "FRAMEWORK",
        Classification::Library => "LIBRARY",
        Classification::Container => "CONTAINER",
        Classification::OperatingSystem => "OPERATING-SYSTEM",
        Classification::Device => "DEVICE",
        Classification::Firmware => "FIRMWARE",
        Classification::File => "FILE",
        _ => return None,
    })
}

/// Builds an SPDX ID from the bom-ref (or name), unique within the document.
fn unique_spdx_id(component: &Component, used: &mut HashSet<String>) -> String {
    let base = format!(
        "SPDXRef-Package-{}",
        sanitize_id(
            component
                .bom_ref
                .as_deref()
                .unwrap_or_else(|| component.name.as_ref())
        )
    );

    let mut spdx_id = base.clone();
    let mut counter = 2;
    while !used.insert(spdx_id.clone()) {
        spdx_id = format!("{base}-{counter}");
        counter += 1;
    }
    spdx_id
}

/// Lists the tools recorded in the CycloneDX metadata (e.g. syft), except mergebom itself.
fn tool_creators(bom: &Bom) -> Vec<String> {
    let Some(tools) = bom
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.tools.as_ref())
    else {
        return vec![];
    };

    let name_versions: Vec<(String, Option<String>)> = match tools {
        Tools::List(tools) => tools
            .iter()
            .filter_map(|tool| {
                Some((
                    tool.name.as_ref()?.to_string(),
                    tool.version.as_ref().map(ToString::to_string),
                ))
            })
            .collect(),
        Tools::Object { components, .. } => components
            .iter()
            .flat_map(|components| &components.0)
            .map(|component| {
                (
                    component.name.to_string(),
                    component.version.as_ref().map(ToString::to_string),
                )
            })
            .collect(),
    };

    name_versions
        .into_iter()
        .filter(|(name, _)| name != env!("CARGO_PKG_NAME"))
        .map(|(name, version)| match version {
            Some(version) => format!("Tool: {name}-{version}"),
            None => format!("Tool: {name}"),
        })
        .collect()
}

/// The SPDX license list version (`major.minor`) the license IDs are validated against.
fn license_list_version() -> String {
    let mut parts = spdx::identifiers::VERSION.split('.');
    match (parts.next(), parts.next()) {
        (Some(major), Some(minor)) => format!("{major}.{minor}"),
        _ => spdx::identifiers::VERSION.to_owned(),
    }
}

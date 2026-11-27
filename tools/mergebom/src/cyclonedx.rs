//! Finalizes the CycloneDX SBOM: removes syft bookkeeping and sets the metadata of the image.

use std::str::FromStr;

use cyclonedx_bom::{
    external_models::uri::{Purl, Uri},
    models::{
        component::{Classification, Component, Components},
        license::{License, LicenseChoice, Licenses},
        lifecycle::{Lifecycle, Lifecycles, Phase},
        organization::{OrganizationalContact, OrganizationalEntity},
        tool::Tools,
    },
    prelude::{Bom, NormalizedString, SpdxExpression},
};

use crate::merge::{FOUND_BY_PROPERTY, VIRTUAL_PATH_PROPERTY};

pub const ORGANIZATION_NAME: &str = "Stackable GmbH";
pub const ORGANIZATION_URL: &str = "https://stackable.tech/";
pub const ORGANIZATION_EMAIL: &str = "info@stackable.tech";

/// Metadata of the image, which is the root component of the SBOM.
#[derive(Debug)]
pub struct ImageMetadata {
    pub purl: String,
    pub description: Option<String>,

    /// SPDX license identifier or expression.
    pub license: Option<spdx::Expression>,
}

/// Removes the syft properties that are only needed while merging.
///
/// syft annotates every component with bookkeeping properties (layer IDs, evidence annotations,
/// package IDs, ...). Only the paths a component was found at, the path inside an archive it was
/// unpacked from and the cataloger name are kept, because they are useful for debugging.
pub fn strip_syft_properties(bom: &mut Bom) {
    let Some(components) = bom.components.as_mut() else {
        return;
    };

    for component in &mut components.0 {
        if let Some(properties) = component.properties.as_mut() {
            properties
                .0
                .retain(|property| keep_property(&property.name));
            if properties.0.is_empty() {
                component.properties = None;
            }
        }
    }
}

fn keep_property(name: &str) -> bool {
    if !name.starts_with("syft:") || name == FOUND_BY_PROPERTY || name == VIRTUAL_PATH_PROPERTY {
        return true;
    }

    // syft:location:<index>:path
    name.strip_prefix("syft:location:")
        .and_then(|rest| rest.strip_suffix(":path"))
        .is_some_and(|index| !index.is_empty() && index.chars().all(|c| c.is_ascii_digit()))
}

/// Sets the metadata of the image on the root component and the document metadata.
pub fn set_image_metadata(bom: &mut Bom, image: &ImageMetadata) {
    let metadata = bom.metadata.get_or_insert_with(Default::default);

    // The SBOM is created by analysing the built image.
    metadata.lifecycles = Some(Lifecycles(vec![Lifecycle::Phase(Phase::PostBuild)]));
    metadata.authors = Some(vec![organization_contact()]);
    add_tool(&mut metadata.tools);

    if let Some(component) = metadata.component.as_mut() {
        // Parsing a purl never fails, cyclonedx-bom does not validate it.
        component.purl = Purl::from_str(&image.purl).ok();
        component.supplier = Some(organization_entity());
        component.manufacturer = Some(organization_entity());
        component.publisher = Some(NormalizedString::new(ORGANIZATION_NAME));
        component.authors = Some(vec![organization_contact()]);
        component.description = image.description.as_deref().map(NormalizedString::new);
        component.licenses = image.license.as_ref().map(license_choice);
    }
}

fn organization_entity() -> OrganizationalEntity {
    let mut entity = OrganizationalEntity::new(ORGANIZATION_NAME);
    entity.url = Some(vec![Uri::new(ORGANIZATION_URL)]);
    entity.contact = Some(vec![OrganizationalContact::new(
        ORGANIZATION_NAME,
        Some(ORGANIZATION_EMAIL),
    )]);
    entity
}

fn organization_contact() -> OrganizationalContact {
    OrganizationalContact::new(ORGANIZATION_NAME, Some(ORGANIZATION_EMAIL))
}

/// Builds the license of the image.
///
/// CycloneDX only allows SPDX license IDs as `license.id`, so everything else (compound
/// expressions, `LicenseRef-`s) becomes an expression.
fn license_choice(license: &spdx::Expression) -> Licenses {
    let license = license.as_ref();
    let choice = if spdx::license_id(license).is_some() {
        LicenseChoice::License(License::license_id(license))
    } else {
        LicenseChoice::Expression(SpdxExpression::new(license))
    };

    Licenses(vec![choice])
}

/// Adds mergebom to the tools of the SBOM, next to syft.
fn add_tool(tools: &mut Option<Tools>) {
    let mergebom = Component::new(
        Classification::Application,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        None,
    );

    match tools {
        Some(Tools::Object { components, .. }) => {
            components
                .get_or_insert_with(|| Components(vec![]))
                .0
                .push(mergebom);
        }
        Some(Tools::List(_)) | None => {
            // syft only writes the object form. The legacy list form is deprecated and cannot hold
            // components, so it is replaced.
            *tools = Some(Tools::Object {
                services: None,
                components: Some(Components(vec![mergebom])),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("syft:package:foundBy", true)]
    #[case("syft:location:0:path", true)]
    #[case("syft:location:12:path", true)]
    #[case("syft:metadata:virtualPath", true)]
    #[case("syft:location:0:layerID", false)]
    #[case("syft:location::path", false)]
    #[case("syft:cpe23", false)]
    #[case("syft:package:type", false)]
    #[case("cdx:something", true)]
    fn keep_only_useful_syft_properties(#[case] name: &str, #[case] expected: bool) {
        assert_eq!(keep_property(name), expected);
    }

    #[rstest]
    #[case("Apache-2.0", false)]
    #[case("Apache-2.0 OR MIT", true)]
    #[case("GPL-2.0-only WITH Classpath-exception-2.0", true)]
    #[case("(MIT)", true)]
    #[case("LicenseRef-Stackable", true)]
    fn license_expression_detection(#[case] license: &str, #[case] is_expression: bool) {
        let license = license
            .parse()
            .expect("the license should be a valid SPDX expression");
        let licenses = license_choice(&license);
        assert_eq!(
            matches!(licenses.0.as_slice(), [LicenseChoice::Expression(_)]),
            is_expression
        );
    }
}

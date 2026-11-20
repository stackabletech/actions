//! Converts CycloneDX license information into SPDX license expressions.

use std::collections::BTreeMap;

use cyclonedx_bom::{
    models::license::{LicenseChoice, LicenseIdentifier, Licenses},
    prelude::{Bom, SpdxExpression},
};
use spdx::{Expression, ParseMode};

pub const NOASSERTION: &str = "NOASSERTION";

const LICENSE_REF_PREFIX: &str = "LicenseRef-";

/// Deprecated SPDX IDs that strict validators reject, with their current equivalent.
///
/// `Expression::canonicalize` does not know these. Extend the list if more show up.
const DEPRECATED_IDS: &[(&str, &str)] = &[(
    "GPL-2.0-with-classpath-exception",
    "GPL-2.0-only WITH Classpath-exception-2.0",
)];

/// Licenses that are not on the SPDX license list, keyed by their `LicenseRef-` ID.
///
/// Every `LicenseRef-` used in an SPDX document needs an entry in `hasExtractedLicensingInfos`,
/// otherwise the document is invalid. The value is the original license name.
#[derive(Debug, Default)]
pub struct ExtractedLicenses(pub BTreeMap<String, String>);

/// Builds the SPDX license expression for the licenses of a CycloneDX component.
///
/// - SPDX IDs and expressions are kept if they are valid. Deprecated or imprecise ones are
///   canonicalised (e.g. `GPL-2.0+` becomes `GPL-2.0-or-later`).
/// - Everything else, including free-text license names, becomes a `LicenseRef-`. Names are never
///   guessed into SPDX IDs: `BSD` could be one of several licenses.
/// - Several licenses are combined with `AND`, as syft does.
///
/// Returns `NOASSERTION` if the component has no license.
pub fn declared_license(licenses: Option<&Licenses>, extracted: &mut ExtractedLicenses) -> String {
    let mut parts: Vec<String> = Vec::new();

    for choice in licenses
        .map(|licenses| licenses.0.as_slice())
        .unwrap_or_default()
    {
        let part = match choice {
            LicenseChoice::Expression(expression) => {
                spdx_expression_or_license_ref(&expression.to_string(), extracted)
            }
            LicenseChoice::License(license) => match &license.license_identifier {
                LicenseIdentifier::SpdxId(id) => {
                    spdx_expression_or_license_ref(&id.to_string(), extracted)
                }
                LicenseIdentifier::Name(name) => license_ref(name.as_ref(), extracted),
            },
        };

        if let Some(part) = part {
            if !parts.contains(&part) {
                parts.push(part);
            }
        }
    }

    match parts.as_slice() {
        [] => NOASSERTION.to_owned(),
        [single] => single.clone(),
        _ => parts
            .iter()
            .map(|part| {
                if part.contains(char::is_whitespace) {
                    format!("({part})")
                } else {
                    part.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" AND "),
    }
}

/// Canonicalises the license expressions of the components in place, so that the CycloneDX
/// document states the same licenses as the SPDX document.
///
/// syft copies license expressions from the package metadata as they are. RPMs in particular do
/// not always spell them the way the SPDX license list does, for example
/// `Apache-2.0 WITH LLVM-Exception` instead of `Apache-2.0 WITH LLVM-exception`. SPDX itself
/// matches IDs case-insensitively, but cyclonedx-bom rejects such an expression when it validates
/// the document.
///
/// Each expression goes through `normalize_expression`, the same step that [`declared_license`]
/// uses for the SPDX document:
///
/// - IDs are spelled as on the license list: `LLVM-Exception` becomes `LLVM-exception`.
/// - Known deprecated IDs are replaced: `GPL-2.0-with-classpath-exception` becomes
///   `GPL-2.0-only WITH Classpath-exception-2.0`.
/// - Imprecise forms are canonicalised: `GPL-2.0+` becomes `GPL-2.0-or-later`.
///
/// An expression that cannot be normalised is kept as it is. The SPDX document turns such an
/// expression into a `LicenseRef-` and declares its text in `hasExtractedLicensingInfos`.
/// CycloneDX has no such list, free text belongs in the `name` of a license instead, and this
/// function does not change how the licenses of a component are structured.
///
/// License IDs (`license.id`) are not touched, only expressions. In our SBOMs syft only wrote IDs
/// that are on the license list.
pub fn normalize_licenses(bom: &mut Bom) {
    let Bom {
        components,
        metadata,
        ..
    } = bom;
    let components = components
        .iter_mut()
        .flat_map(|components| components.0.iter_mut())
        .chain(
            metadata
                .as_mut()
                .and_then(|metadata| metadata.component.as_mut()),
        );

    for component in components {
        let choices = component
            .licenses
            .iter_mut()
            .flat_map(|licenses| licenses.0.iter_mut());
        for choice in choices {
            let LicenseChoice::Expression(expression) = choice else {
                continue;
            };
            let original = expression.to_string();
            if let Some(normalized) = normalize_expression(&original) {
                if normalized != original {
                    *expression = SpdxExpression::new(&normalized);
                }
            }
        }
    }
}

fn spdx_expression_or_license_ref(
    expression: &str,
    extracted: &mut ExtractedLicenses,
) -> Option<String> {
    match normalize_expression(expression) {
        Some(expression) => {
            register_license_refs(&expression, extracted);
            Some(expression)
        }
        None => license_ref(expression, extracted),
    }
}

/// Returns the expression in a form that SPDX validators accept, if there is one.
///
/// In order of preference: the expression as is, with known deprecated IDs replaced and IDs
/// spelled as on the license list, in its canonical form (e.g. `GPL-2.0+` becomes
/// `GPL-2.0-or-later`), or as is if it only uses deprecated IDs, which are still on the SPDX
/// license list.
fn normalize_expression(expression: &str) -> Option<String> {
    let expression = expression.trim();
    if expression.is_empty() {
        return None;
    }

    let is_strict =
        |expression: &str| Expression::parse_mode(expression, ParseMode::STRICT).is_ok();

    if is_strict(expression) {
        return Some(expression.to_owned());
    }

    let replaced = fix_id_case(&replace_deprecated_ids(expression));
    if is_strict(&replaced) {
        return Some(replaced);
    }

    if let Some(canonical) = Expression::canonicalize(&replaced).ok().flatten() {
        if is_strict(&canonical) {
            return Some(canonical);
        }
    }

    let allow_deprecated = ParseMode {
        allow_deprecated: true,
        ..ParseMode::STRICT
    };
    Expression::parse_mode(&replaced, allow_deprecated)
        .is_ok()
        .then_some(replaced)
}

/// Spells license and exception IDs as the SPDX license list does, e.g. `LLVM-Exception` becomes
/// `LLVM-exception`.
///
/// SPDX matches IDs case-insensitively (SPDX 2.3, Annex D), but the spdx crate and therefore
/// cyclonedx-bom only accept the spelling of the license list. RPMs do not always use it.
fn fix_id_case(expression: &str) -> String {
    expression
        .split_whitespace()
        .map(|token| {
            // Parentheses stick to the IDs next to them.
            let start = token.find(|c| c != '(').unwrap_or(token.len());
            let end = token.rfind(|c| c != ')').map_or(start, |index| index + 1);
            let (open, rest) = token.split_at(start);
            let (id, close) = rest.split_at(end.saturating_sub(start));

            let canonical = spdx::identifiers::LICENSES
                .iter()
                .map(|license| license.name)
                .chain(
                    spdx::identifiers::EXCEPTIONS
                        .iter()
                        .map(|exception| exception.name),
                )
                .find(|name| name.eq_ignore_ascii_case(id))
                .unwrap_or(id);
            format!("{open}{canonical}{close}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Replaces deprecated IDs from [`DEPRECATED_IDS`], parenthesised if they are part of a larger
/// expression.
fn replace_deprecated_ids(expression: &str) -> String {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    let is_compound = tokens.len() > 1;

    tokens
        .into_iter()
        .map(|token| {
            match DEPRECATED_IDS
                .iter()
                .find(|(deprecated, _)| *deprecated == token)
            {
                Some((_, replacement)) if is_compound => format!("({replacement})"),
                Some((_, replacement)) => (*replacement).to_owned(),
                None => token.to_owned(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Turns a license name into a `LicenseRef-` and registers it as an extracted license.
fn license_ref(name: &str, extracted: &mut ExtractedLicenses) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let id = format!("{LICENSE_REF_PREFIX}{}", sanitize_id(name));
    extracted
        .0
        .entry(id.clone())
        .or_insert_with(|| name.to_owned());
    Some(id)
}

/// Registers `LicenseRef-`s that already appear in a valid expression.
fn register_license_refs(expression: &str, extracted: &mut ExtractedLicenses) {
    for token in expression.split(|c: char| c.is_whitespace() || c == '(' || c == ')') {
        if let Some(name) = token.strip_prefix(LICENSE_REF_PREFIX) {
            extracted
                .0
                .entry(token.to_owned())
                .or_insert_with(|| name.to_owned());
        }
    }
}

/// Replaces every character that is not allowed in an SPDX ID (`[A-Za-z0-9.-]`) with `-`.
pub fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use cyclonedx_bom::models::license::License;
    use rstest::rstest;
    use serde_json::json;

    use super::*;

    fn license_id(id: &str) -> LicenseChoice {
        LicenseChoice::License(License::license_id(id))
    }

    fn license_name(name: &str) -> LicenseChoice {
        LicenseChoice::License(License::named_license(name))
    }

    fn expression(expression: &str) -> LicenseChoice {
        LicenseChoice::Expression(SpdxExpression::new(expression))
    }

    #[rstest]
    #[case::none(vec![], "NOASSERTION", vec![])]
    #[case::single_id(vec![license_id("Apache-2.0")], "Apache-2.0", vec![])]
    #[case::multiple_ids(
        vec![license_id("BSD-2-Clause"), license_id("CC0-1.0")],
        "BSD-2-Clause AND CC0-1.0",
        vec![],
    )]
    #[case::duplicate_ids(
        vec![license_id("MIT"), license_id("MIT")],
        "MIT",
        vec![],
    )]
    #[case::deprecated_id(
        vec![license_id("GPL-2.0-with-classpath-exception")],
        "GPL-2.0-only WITH Classpath-exception-2.0",
        vec![],
    )]
    #[case::deprecated_id_in_expression(
        vec![expression("CDDL-1.1 OR GPL-2.0-with-classpath-exception")],
        "CDDL-1.1 OR (GPL-2.0-only WITH Classpath-exception-2.0)",
        vec![],
    )]
    #[case::gpl_plus(vec![license_id("GPL-2.0+")], "GPL-2.0-or-later", vec![])]
    #[case::exception_in_wrong_case(
        vec![expression("Apache-2.0 WITH LLVM-Exception")],
        "Apache-2.0 WITH LLVM-exception",
        vec![],
    )]
    #[case::ids_in_wrong_case_in_parentheses(
        vec![expression("(apache-2.0 WITH LLVM-Exception) AND mit")],
        "(Apache-2.0 WITH LLVM-exception) AND MIT",
        vec![],
    )]
    #[case::slash_as_or(vec![expression("MIT/Apache-2.0")], "MIT OR Apache-2.0", vec![])]
    #[case::expression(
        vec![expression("0BSD OR MIT OR Apache-2.0")],
        "0BSD OR MIT OR Apache-2.0",
        vec![],
    )]
    #[case::expression_combined_with_id(
        vec![expression("MIT OR Apache-2.0"), license_id("BSD-3-Clause")],
        "(MIT OR Apache-2.0) AND BSD-3-Clause",
        vec![],
    )]
    #[case::name_is_not_guessed(
        vec![license_name("BSD")],
        "LicenseRef-BSD",
        vec![("LicenseRef-BSD", "BSD")],
    )]
    #[case::rpm_name(
        vec![license_name("GPLv2+ and BSD")],
        "LicenseRef-GPLv2--and-BSD",
        vec![("LicenseRef-GPLv2--and-BSD", "GPLv2+ and BSD")],
    )]
    #[case::invalid_expression(
        vec![expression("Apache License, Version 2.0")],
        "LicenseRef-Apache-License--Version-2.0",
        vec![("LicenseRef-Apache-License--Version-2.0", "Apache License, Version 2.0")],
    )]
    #[case::license_ref_inside_expression(
        vec![expression("MIT AND LicenseRef-Fedora-Public-Domain")],
        "MIT AND LicenseRef-Fedora-Public-Domain",
        vec![("LicenseRef-Fedora-Public-Domain", "Fedora-Public-Domain")],
    )]
    fn convert_licenses(
        #[case] licenses: Vec<LicenseChoice>,
        #[case] expected_expression: &str,
        #[case] expected_extracted: Vec<(&str, &str)>,
    ) {
        let mut extracted = ExtractedLicenses::default();
        let licenses = Licenses(licenses);

        let expression = declared_license(Some(&licenses), &mut extracted);

        assert_eq!(expression, expected_expression);
        assert_eq!(
            extracted.0,
            expected_extracted
                .into_iter()
                .map(|(id, name)| (id.to_owned(), name.to_owned()))
                .collect::<BTreeMap<_, _>>()
        );
    }

    #[rstest]
    #[case::wrong_case("Apache-2.0 WITH LLVM-Exception", "Apache-2.0 WITH LLVM-exception")]
    #[case::deprecated_id(
        "CDDL-1.1 OR GPL-2.0-with-classpath-exception",
        "CDDL-1.1 OR (GPL-2.0-only WITH Classpath-exception-2.0)"
    )]
    #[case::valid("0BSD OR MIT OR Apache-2.0", "0BSD OR MIT OR Apache-2.0")]
    // Still on the SPDX license list, and there is no replacement to put in.
    #[case::deprecated_id_without_replacement(
        "MIT AND BSD-2-Clause-FreeBSD",
        "MIT AND BSD-2-Clause-FreeBSD"
    )]
    #[case::not_an_expression("Apache License, Version 2.0", "Apache License, Version 2.0")]
    fn normalize_component_license_expressions(#[case] expression: &str, #[case] expected: &str) {
        let sbom = json!({
            "bomFormat": "CycloneDX",
            "specVersion": "1.5",
            "version": 1,
            "metadata": {
                "component": {
                    "type": "container",
                    "name": "image",
                    "licenses": [{ "expression": expression }]
                }
            },
            "components": [{
                "type": "library",
                "name": "library",
                "licenses": [{ "expression": expression }]
            }],
        });
        let mut bom = Bom::parse_from_json(sbom.to_string().as_bytes())
            .expect("the test SBOM should be a valid CycloneDX SBOM");

        normalize_licenses(&mut bom);

        let root = bom
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.component.as_ref());
        let component = bom
            .components
            .as_ref()
            .and_then(|components| components.0.first());
        for component in [root, component] {
            let licenses = component
                .and_then(|component| component.licenses.as_ref())
                .expect("the component should have licenses");
            assert_eq!(
                licenses.0,
                vec![LicenseChoice::Expression(SpdxExpression::new(expected))]
            );
        }
    }
}

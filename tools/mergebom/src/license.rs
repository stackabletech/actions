//! Converts CycloneDX license information into SPDX license expressions.

use std::collections::BTreeMap;

use cyclonedx_bom::models::license::{LicenseChoice, LicenseIdentifier, Licenses};
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
/// In order of preference: the expression as is, with known deprecated IDs replaced, in its
/// canonical form (e.g. `GPL-2.0+` becomes `GPL-2.0-or-later`), or as is if it only uses
/// deprecated IDs, which are still on the SPDX license list.
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

    let replaced = replace_deprecated_ids(expression);
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
    use cyclonedx_bom::{models::license::License, prelude::SpdxExpression};
    use rstest::rstest;

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
}

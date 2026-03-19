/// Parser and utilities for complex Rust enum variants in qleany manifests.
///
/// Supports three variant kinds:
/// - Simple:  `Active`
/// - Tuple:   `Text(String)` or `Pair(i64, Option<String>)`
/// - Struct:  `Image { name: String, width: i64, quality: Option<f64> }`
///
/// Inner types use final Rust types: `bool`, `i32`, `i64`, `u32`, `u64`,
/// `f32`, `f64`, `String`, etc. Three shorthands expand to qualified paths:
/// `Uuid` → `uuid::Uuid`, `DateTime` → `chrono::DateTime<chrono::Utc>`,
/// `EntityId` → `EntityId`.
/// `Option<T>` and `Vec<T>` wrappers are supported.
/// Any other PascalCase name is treated as an enum reference (used as-is).

use anyhow::{Result, anyhow};

// ─────────────────────────────────────────────────────────────────────────────
// Known Rust scalar types accepted inside complex variants
// ─────────────────────────────────────────────────────────────────────────────

const RUST_SCALARS: &[&str] = &[
    "bool", "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
    "isize", "usize", "String",
];

fn is_rust_scalar(s: &str) -> bool {
    RUST_SCALARS.contains(&s)
}

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum VariantFieldType {
    /// A known Rust scalar type, emitted as-is (e.g. `"i64"`, `"String"`, `"bool"`).
    Scalar(std::string::String),
    /// Shorthand for `uuid::Uuid`.
    Uuid,
    /// Shorthand for `chrono::DateTime<chrono::Utc>`.
    DateTime,
    /// The `EntityId` type (alias for `u64`).
    EntityId,
    /// A PascalCase enum name — emitted as-is in generated code.
    EnumRef(std::string::String),
    /// `Option<T>`
    Option(Box<VariantFieldType>),
    /// `Vec<T>`
    Vec(Box<VariantFieldType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumVariantKind {
    Simple,
    Tuple(Vec<VariantFieldType>),
    Struct(Vec<(std::string::String, VariantFieldType)>),
}

#[derive(Debug, Clone)]
pub struct ParsedEnumVariant {
    pub name: std::string::String,
    pub kind: EnumVariantKind,
}

// ─────────────────────────────────────────────────────────────────────────────
// Parsing
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a single enum variant string like `"Active"`, `"Text(String)"`,
/// or `"Image { name: String, width: i64 }"`.
pub fn parse_enum_variant(raw: &str) -> Result<ParsedEnumVariant> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(anyhow!("Empty enum variant"));
    }

    // Find where variant name ends (first '(' or '{')
    let name_end = raw
        .find(|c: char| c == '(' || c == '{')
        .unwrap_or(raw.len());
    let name = raw[..name_end].trim().to_string();

    if name.is_empty() {
        return Err(anyhow!("Enum variant name is empty"));
    }

    let rest = raw[name_end..].trim();

    if rest.is_empty() {
        return Ok(ParsedEnumVariant {
            name,
            kind: EnumVariantKind::Simple,
        });
    }

    if rest.starts_with('(') {
        let close = find_matching(rest, '(', ')')
            .map_err(|_| anyhow!("Unmatched '(' in variant '{}'", name))?;
        let trailing = rest[close + 1..].trim();
        if !trailing.is_empty() {
            return Err(anyhow!(
                "Unexpected characters after ')' in variant '{}': {}",
                name,
                trailing
            ));
        }
        let inner = rest[1..close].trim();
        if inner.is_empty() {
            return Err(anyhow!("Empty tuple in variant '{}'", name));
        }
        let fields = parse_comma_separated_types(inner)?;
        Ok(ParsedEnumVariant {
            name,
            kind: EnumVariantKind::Tuple(fields),
        })
    } else if rest.starts_with('{') {
        let close = find_matching(rest, '{', '}')
            .map_err(|_| anyhow!("Unmatched '{{' in variant '{}'", name))?;
        let trailing = rest[close + 1..].trim();
        if !trailing.is_empty() {
            return Err(anyhow!(
                "Unexpected characters after '}}' in variant '{}': {}",
                name,
                trailing
            ));
        }
        let inner = rest[1..close].trim();
        if inner.is_empty() {
            return Err(anyhow!("Empty struct in variant '{}'", name));
        }
        let fields = parse_comma_separated_named_fields(inner)?;
        Ok(ParsedEnumVariant {
            name,
            kind: EnumVariantKind::Struct(fields),
        })
    } else {
        Err(anyhow!(
            "Unexpected characters after variant name '{}': {}",
            name,
            rest
        ))
    }
}

/// Find the index of the matching closing delimiter, respecting nested `<>`.
fn find_matching(s: &str, open: char, close: char) -> Result<usize> {
    let mut depth: i32 = 0;
    let mut angle: i32 = 0;

    for (i, c) in s.char_indices() {
        if c == open && angle == 0 {
            depth += 1;
        } else if c == close && angle == 0 {
            depth -= 1;
            if depth == 0 {
                return Ok(i);
            }
        } else if c == '<' {
            angle += 1;
        } else if c == '>' {
            angle -= 1;
        }
    }
    Err(anyhow!("Unmatched '{}'", open))
}

/// Split a string by commas, respecting `<>` nesting.
fn split_respecting_angles(s: &str) -> Vec<std::string::String> {
    let mut result = Vec::new();
    let mut current = std::string::String::new();
    let mut angle: i32 = 0;

    for c in s.chars() {
        match c {
            '<' => {
                angle += 1;
                current.push(c);
            }
            '>' => {
                angle -= 1;
                current.push(c);
            }
            ',' if angle == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    result.push(trimmed);
                }
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        result.push(trimmed);
    }
    result
}

/// Parse a type string like `"i64"`, `"Option<String>"`, `"Vec<u32>"`, `"Uuid"`.
fn parse_type(s: &str) -> Result<VariantFieldType> {
    let s = s.trim();
    if s.is_empty() {
        return Err(anyhow!("Empty type"));
    }

    // Option<T> wrapper
    if let Some(inner) = s.strip_prefix("Option<").and_then(|r| r.strip_suffix('>')) {
        let inner_type = parse_type(inner)?;
        return Ok(VariantFieldType::Option(Box::new(inner_type)));
    }
    // Vec<T> wrapper
    if let Some(inner) = s.strip_prefix("Vec<").and_then(|r| r.strip_suffix('>')) {
        let inner_type = parse_type(inner)?;
        return Ok(VariantFieldType::Vec(Box::new(inner_type)));
    }

    // Shorthands
    match s {
        "Uuid" => return Ok(VariantFieldType::Uuid),
        "DateTime" => return Ok(VariantFieldType::DateTime),
        "EntityId" => return Ok(VariantFieldType::EntityId),
        _ => {}
    }

    // Known Rust scalar types
    if is_rust_scalar(s) {
        return Ok(VariantFieldType::Scalar(s.to_string()));
    }

    // Anything else must be a valid identifier — treated as an enum reference
    if s.chars()
        .all(|c| c.is_alphanumeric() || c == '_')
        && s.chars().next().is_some_and(|c| c.is_alphabetic())
    {
        Ok(VariantFieldType::EnumRef(s.to_string()))
    } else {
        Err(anyhow!(
            "Unknown type '{}': expected a Rust type (bool, i32, i64, u64, f64, String, ...), \
             a shorthand (Uuid, DateTime, EntityId), or a PascalCase enum name",
            s
        ))
    }
}

/// Parse comma-separated types: `"i64, String, Option<f64>"`
fn parse_comma_separated_types(s: &str) -> Result<Vec<VariantFieldType>> {
    let parts = split_respecting_angles(s);
    let mut result = Vec::new();
    for part in &parts {
        result.push(parse_type(part)?);
    }
    Ok(result)
}

/// Parse comma-separated named fields: `"name: String, width: i64"`
fn parse_comma_separated_named_fields(
    s: &str,
) -> Result<Vec<(std::string::String, VariantFieldType)>> {
    let parts = split_respecting_angles(s);
    let mut result = Vec::new();
    for part in &parts {
        let colon_pos = part
            .find(':')
            .ok_or_else(|| anyhow!("Struct field '{}' missing ':' separator", part))?;
        let field_name = part[..colon_pos].trim().to_string();
        let field_type_str = part[colon_pos + 1..].trim();
        if field_name.is_empty() {
            return Err(anyhow!("Empty field name in struct variant"));
        }
        let field_type = parse_type(field_type_str)?;
        result.push((field_name, field_type));
    }
    Ok(result)
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility: collect references, check flags
// ─────────────────────────────────────────────────────────────────────────────

/// Collect all EnumRef names from a parsed variant.
pub fn collect_references(variant: &ParsedEnumVariant) -> Vec<std::string::String> {
    let mut refs = Vec::new();
    match &variant.kind {
        EnumVariantKind::Simple => {}
        EnumVariantKind::Tuple(fields) => {
            for f in fields {
                collect_type_references(f, &mut refs);
            }
        }
        EnumVariantKind::Struct(fields) => {
            for (_, f) in fields {
                collect_type_references(f, &mut refs);
            }
        }
    }
    refs
}

fn collect_type_references(vft: &VariantFieldType, out: &mut Vec<std::string::String>) {
    match vft {
        VariantFieldType::EnumRef(name) => out.push(name.clone()),
        VariantFieldType::Option(inner) | VariantFieldType::Vec(inner) => {
            collect_type_references(inner, out);
        }
        _ => {}
    }
}

/// Check if a variant type tree contains Uuid anywhere.
pub fn type_needs_uuid(vft: &VariantFieldType) -> bool {
    match vft {
        VariantFieldType::Uuid => true,
        VariantFieldType::Option(inner) | VariantFieldType::Vec(inner) => {
            type_needs_uuid(inner)
        }
        _ => false,
    }
}

/// Check if a variant type tree contains DateTime anywhere.
pub fn type_needs_chrono(vft: &VariantFieldType) -> bool {
    match vft {
        VariantFieldType::DateTime => true,
        VariantFieldType::Option(inner) | VariantFieldType::Vec(inner) => {
            type_needs_chrono(inner)
        }
        _ => false,
    }
}

/// Check if a variant type tree contains EntityId anywhere.
pub fn type_needs_entity_id(vft: &VariantFieldType) -> bool {
    match vft {
        VariantFieldType::EntityId => true,
        VariantFieldType::Option(inner) | VariantFieldType::Vec(inner) => {
            type_needs_entity_id(inner)
        }
        _ => false,
    }
}

/// Check if a whole variant needs uuid/chrono/entity_id imports.
pub fn variant_needs_uuid(variant: &ParsedEnumVariant) -> bool {
    variant_fields_iter(&variant.kind).any(|f| type_needs_uuid(f))
}

pub fn variant_needs_chrono(variant: &ParsedEnumVariant) -> bool {
    variant_fields_iter(&variant.kind).any(|f| type_needs_chrono(f))
}

pub fn variant_needs_entity_id(variant: &ParsedEnumVariant) -> bool {
    variant_fields_iter(&variant.kind).any(|f| type_needs_entity_id(f))
}

fn variant_fields_iter(kind: &EnumVariantKind) -> Box<dyn Iterator<Item = &VariantFieldType> + '_> {
    match kind {
        EnumVariantKind::Simple => Box::new(std::iter::empty()),
        EnumVariantKind::Tuple(fields) => Box::new(fields.iter()),
        EnumVariantKind::Struct(fields) => Box::new(fields.iter().map(|(_, f)| f)),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rust type mapping (mostly identity — only shorthands expand)
// ─────────────────────────────────────────────────────────────────────────────

/// Convert a VariantFieldType to its Rust type string.
/// Scalars and enum refs pass through; shorthands expand to qualified paths.
pub fn type_to_rust(vft: &VariantFieldType) -> std::string::String {
    match vft {
        VariantFieldType::Scalar(s) => s.clone(),
        VariantFieldType::Uuid => "uuid::Uuid".to_string(),
        VariantFieldType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
        VariantFieldType::EntityId => "EntityId".to_string(),
        VariantFieldType::EnumRef(name) => name.clone(),
        VariantFieldType::Option(inner) => {
            format!("Option<{}>", type_to_rust(inner))
        }
        VariantFieldType::Vec(inner) => {
            format!("Vec<{}>", type_to_rust(inner))
        }
    }
}

/// Convert an entire variant to its Rust definition line.
/// E.g. `"Text(String)"`, `"Image { name: String, width: i64 }"`, `"Active"`.
pub fn variant_to_rust_line(variant: &ParsedEnumVariant) -> std::string::String {
    match &variant.kind {
        EnumVariantKind::Simple => variant.name.clone(),
        EnumVariantKind::Tuple(fields) => {
            let types: Vec<std::string::String> =
                fields.iter().map(|f| type_to_rust(f)).collect();
            format!("{}({})", variant.name, types.join(", "))
        }
        EnumVariantKind::Struct(fields) => {
            let field_strs: Vec<std::string::String> = fields
                .iter()
                .map(|(name, typ)| format!("{}: {}", name, type_to_rust(typ)))
                .collect();
            format!("{} {{ {} }}", variant.name, field_strs.join(", "))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mobile type mapping (for UniFFI bridge)
// ─────────────────────────────────────────────────────────────────────────────

/// Convert a VariantFieldType to its mobile Rust type string.
/// EntityId → u64, Uuid → String, DateTime → MobileDateTime, EnumRef → Mobile{Name}.
pub fn type_to_mobile_rust(vft: &VariantFieldType) -> std::string::String {
    match vft {
        VariantFieldType::Scalar(s) => s.clone(),
        VariantFieldType::Uuid => "String".to_string(),
        VariantFieldType::DateTime => "MobileDateTime".to_string(),
        VariantFieldType::EntityId => "u64".to_string(),
        VariantFieldType::EnumRef(name) => format!("Mobile{}", name),
        VariantFieldType::Option(inner) => {
            format!("Option<{}>", type_to_mobile_rust(inner))
        }
        VariantFieldType::Vec(inner) => {
            format!("Vec<{}>", type_to_mobile_rust(inner))
        }
    }
}

/// Convert an entire variant to its mobile definition line.
pub fn variant_to_mobile_line(variant: &ParsedEnumVariant) -> std::string::String {
    match &variant.kind {
        EnumVariantKind::Simple => variant.name.clone(),
        EnumVariantKind::Tuple(fields) => {
            let types: Vec<std::string::String> = fields
                .iter()
                .map(|f| type_to_mobile_rust(f))
                .collect();
            format!("{}({})", variant.name, types.join(", "))
        }
        EnumVariantKind::Struct(fields) => {
            let field_strs: Vec<std::string::String> = fields
                .iter()
                .map(|(name, typ)| format!("{}: {}", name, type_to_mobile_rust(typ)))
                .collect();
            format!("{} {{ {} }}", variant.name, field_strs.join(", "))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Match pattern and conversion expressions (for From impls)
// ─────────────────────────────────────────────────────────────────────────────

/// Generate the destructuring match pattern for a variant.
/// E.g. `"Active"`, `"Text(v0)"`, `"Image { name, width }"`.
pub fn variant_match_pattern(variant: &ParsedEnumVariant) -> std::string::String {
    match &variant.kind {
        EnumVariantKind::Simple => variant.name.clone(),
        EnumVariantKind::Tuple(fields) => {
            let vars: Vec<std::string::String> =
                (0..fields.len()).map(|i| format!("v{}", i)).collect();
            format!("{}({})", variant.name, vars.join(", "))
        }
        EnumVariantKind::Struct(fields) => {
            let names: Vec<&str> = fields.iter().map(|(n, _)| n.as_str()).collect();
            format!("{} {{ {} }}", variant.name, names.join(", "))
        }
    }
}

/// Generate the forwarding construction for mobile-to-core From impl.
/// Handles type conversions for Uuid (String → uuid::Uuid) and DateTime (MobileDateTime → chrono).
pub fn variant_mobile_to_core_construct(variant: &ParsedEnumVariant) -> std::string::String {
    match &variant.kind {
        EnumVariantKind::Simple => variant.name.clone(),
        EnumVariantKind::Tuple(fields) => {
            let args: Vec<std::string::String> = fields
                .iter()
                .enumerate()
                .map(|(i, f)| mobile_to_core_expr(&format!("v{}", i), f))
                .collect();
            format!("{}({})", variant.name, args.join(", "))
        }
        EnumVariantKind::Struct(fields) => {
            let args: Vec<std::string::String> = fields
                .iter()
                .map(|(name, f)| {
                    let expr = mobile_to_core_expr(name, f);
                    if expr == *name {
                        name.clone()
                    } else {
                        format!("{}: {}", name, expr)
                    }
                })
                .collect();
            format!("{} {{ {} }}", variant.name, args.join(", "))
        }
    }
}

/// Generate the forwarding construction for core-to-mobile From impl.
pub fn variant_core_to_mobile_construct(variant: &ParsedEnumVariant) -> std::string::String {
    match &variant.kind {
        EnumVariantKind::Simple => variant.name.clone(),
        EnumVariantKind::Tuple(fields) => {
            let args: Vec<std::string::String> = fields
                .iter()
                .enumerate()
                .map(|(i, f)| core_to_mobile_expr(&format!("v{}", i), f))
                .collect();
            format!("{}({})", variant.name, args.join(", "))
        }
        EnumVariantKind::Struct(fields) => {
            let args: Vec<std::string::String> = fields
                .iter()
                .map(|(name, f)| {
                    let expr = core_to_mobile_expr(name, f);
                    if expr == *name {
                        name.clone()
                    } else {
                        format!("{}: {}", name, expr)
                    }
                })
                .collect();
            format!("{} {{ {} }}", variant.name, args.join(", "))
        }
    }
}

/// Expression to convert a mobile value to core Rust type.
fn mobile_to_core_expr(var: &str, vft: &VariantFieldType) -> std::string::String {
    match vft {
        VariantFieldType::Uuid => {
            format!(
                "uuid::Uuid::parse_str(&{}).unwrap_or_default()",
                var
            )
        }
        VariantFieldType::DateTime => format!("{}.0", var),
        VariantFieldType::EnumRef(_) => {
            format!("{}.into()", var)
        }
        VariantFieldType::Option(inner) => {
            let inner_expr = mobile_to_core_expr("x", inner);
            if inner_expr == "x" {
                var.to_string()
            } else {
                format!("{}.map(|x| {})", var, inner_expr)
            }
        }
        VariantFieldType::Vec(inner) => {
            let inner_expr = mobile_to_core_expr("x", inner);
            if inner_expr == "x" {
                var.to_string()
            } else {
                format!("{}.into_iter().map(|x| {}).collect()", var, inner_expr)
            }
        }
        _ => var.to_string(), // identity for Scalar and EntityId
    }
}

/// Expression to convert a core Rust value to mobile type.
fn core_to_mobile_expr(var: &str, vft: &VariantFieldType) -> std::string::String {
    match vft {
        VariantFieldType::Uuid => format!("{}.to_string()", var),
        VariantFieldType::DateTime => format!("MobileDateTime({})", var),
        VariantFieldType::EnumRef(_) => {
            format!("{}.into()", var)
        }
        VariantFieldType::Option(inner) => {
            let inner_expr = core_to_mobile_expr("x", inner);
            if inner_expr == "x" {
                var.to_string()
            } else {
                format!("{}.map(|x| {})", var, inner_expr)
            }
        }
        VariantFieldType::Vec(inner) => {
            let inner_expr = core_to_mobile_expr("x", inner);
            if inner_expr == "x" {
                var.to_string()
            } else {
                format!("{}.into_iter().map(|x| {}).collect()", var, inner_expr)
            }
        }
        _ => var.to_string(),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_variant() {
        let v = parse_enum_variant("Active").unwrap();
        assert_eq!(v.name, "Active");
        assert_eq!(v.kind, EnumVariantKind::Simple);
    }

    #[test]
    fn test_parse_tuple_variant_single() {
        let v = parse_enum_variant("Text(String)").unwrap();
        assert_eq!(v.name, "Text");
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![VariantFieldType::Scalar("String".to_string())])
        );
    }

    #[test]
    fn test_parse_tuple_variant_multi() {
        let v = parse_enum_variant("Pair(i64, String)").unwrap();
        assert_eq!(v.name, "Pair");
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![
                VariantFieldType::Scalar("i64".to_string()),
                VariantFieldType::Scalar("String".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_struct_variant() {
        let v = parse_enum_variant("Image { name: String, width: i64 }").unwrap();
        assert_eq!(v.name, "Image");
        assert_eq!(
            v.kind,
            EnumVariantKind::Struct(vec![
                ("name".to_string(), VariantFieldType::Scalar("String".to_string())),
                ("width".to_string(), VariantFieldType::Scalar("i64".to_string())),
            ])
        );
    }

    #[test]
    fn test_parse_option_type() {
        let v = parse_enum_variant("Note(Option<String>)").unwrap();
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![VariantFieldType::Option(Box::new(
                VariantFieldType::Scalar("String".to_string())
            ))])
        );
    }

    #[test]
    fn test_parse_vec_type() {
        let v = parse_enum_variant("Items(Vec<i32>)").unwrap();
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![VariantFieldType::Vec(Box::new(
                VariantFieldType::Scalar("i32".to_string())
            ))])
        );
    }

    #[test]
    fn test_parse_option_vec() {
        let v = parse_enum_variant("Data(Option<Vec<String>>)").unwrap();
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![VariantFieldType::Option(Box::new(
                VariantFieldType::Vec(Box::new(VariantFieldType::Scalar("String".to_string())))
            ))])
        );
    }

    #[test]
    fn test_parse_shorthands() {
        let v = parse_enum_variant("Stamped(Uuid, DateTime, EntityId)").unwrap();
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![
                VariantFieldType::Uuid,
                VariantFieldType::DateTime,
                VariantFieldType::EntityId,
            ])
        );
    }

    #[test]
    fn test_parse_enum_reference() {
        let v = parse_enum_variant("Tagged(ProjectStatus)").unwrap();
        assert_eq!(
            v.kind,
            EnumVariantKind::Tuple(vec![VariantFieldType::EnumRef(
                "ProjectStatus".to_string()
            )])
        );
    }

    #[test]
    fn test_unmatched_paren() {
        assert!(parse_enum_variant("Bad(String").is_err());
    }

    #[test]
    fn test_unmatched_brace() {
        assert!(parse_enum_variant("Bad { name: String").is_err());
    }

    #[test]
    fn test_empty_tuple() {
        assert!(parse_enum_variant("Bad()").is_err());
    }

    #[test]
    fn test_empty_struct() {
        assert!(parse_enum_variant("Bad {}").is_err());
    }

    #[test]
    fn test_missing_colon_in_struct() {
        assert!(parse_enum_variant("Bad { name String }").is_err());
    }

    #[test]
    fn test_unknown_type_rejected() {
        // Types with special chars are rejected
        assert!(parse_enum_variant("Bad(foo::bar)").is_err());
    }

    #[test]
    fn test_variant_to_rust_line_simple() {
        let v = parse_enum_variant("Active").unwrap();
        assert_eq!(variant_to_rust_line(&v), "Active");
    }

    #[test]
    fn test_variant_to_rust_line_tuple() {
        let v = parse_enum_variant("Text(i64)").unwrap();
        assert_eq!(variant_to_rust_line(&v), "Text(i64)");
    }

    #[test]
    fn test_variant_to_rust_shorthand_expansion() {
        let v = parse_enum_variant("Stamped(Uuid, DateTime)").unwrap();
        assert_eq!(
            variant_to_rust_line(&v),
            "Stamped(uuid::Uuid, chrono::DateTime<chrono::Utc>)"
        );
    }

    #[test]
    fn test_variant_to_rust_line_struct() {
        let v = parse_enum_variant("Image { name: String, width: i64 }").unwrap();
        assert_eq!(
            variant_to_rust_line(&v),
            "Image { name: String, width: i64 }"
        );
    }

    #[test]
    fn test_variant_to_rust_enum_ref() {
        let v = parse_enum_variant("Tagged(ProjectStatus)").unwrap();
        assert_eq!(variant_to_rust_line(&v), "Tagged(ProjectStatus)");
    }

    #[test]
    fn test_match_pattern() {
        let v = parse_enum_variant("Image { name: String, width: i64 }").unwrap();
        assert_eq!(variant_match_pattern(&v), "Image { name, width }");

        let v2 = parse_enum_variant("Text(String, i64)").unwrap();
        assert_eq!(variant_match_pattern(&v2), "Text(v0, v1)");

        let v3 = parse_enum_variant("Active").unwrap();
        assert_eq!(variant_match_pattern(&v3), "Active");
    }

    #[test]
    fn test_collect_references() {
        let v = parse_enum_variant("Mixed(ProjectStatus, Option<TaskDifficulty>)").unwrap();
        let refs = collect_references(&v);
        assert_eq!(refs, vec!["ProjectStatus", "TaskDifficulty"]);
    }

    #[test]
    fn test_variant_needs_flags() {
        let v = parse_enum_variant("Data(Uuid, DateTime)").unwrap();
        assert!(variant_needs_uuid(&v));
        assert!(variant_needs_chrono(&v));

        let v2 = parse_enum_variant("Simple(String)").unwrap();
        assert!(!variant_needs_uuid(&v2));
        assert!(!variant_needs_chrono(&v2));

        let v3 = parse_enum_variant("HasId(EntityId)").unwrap();
        assert!(variant_needs_entity_id(&v3));
    }
}

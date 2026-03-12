use crate::use_cases::common::cpp_qt_code_generator::GenerationOps;
use common::entities::Workspace;
use common::types::EntityId;

pub fn get_system_id(uow: &dyn GenerationOps) -> anyhow::Result<EntityId> {
    use anyhow::anyhow;
    let roots = uow.get_all_root()?;
    let root = roots
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_system_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::System,
    )?;

    let system_id = all_system_ids
        .first()
        .cloned()
        .ok_or(anyhow!("No system found"))?;
    Ok(system_id)
}

pub fn get_workspace_id(uow: &dyn GenerationOps) -> anyhow::Result<EntityId> {
    use anyhow::anyhow;
    let roots = uow.get_all_root()?;
    let root = roots
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace,
    )?;

    let workspace_id = all_workspace_ids
        .first()
        .cloned()
        .ok_or(anyhow!("No workspace found"))?;
    Ok(workspace_id)
}

pub fn get_workspace(uow: &dyn GenerationOps) -> anyhow::Result<Workspace> {
    use anyhow::anyhow;
    let roots = uow.get_all_root()?;
    let root = roots
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Root entity not found"))?;

    let all_workspace_ids = uow.get_root_relationship(
        &root.id,
        &common::direct_access::root::RootRelationshipField::Workspace,
    )?;

    let workspace_id = all_workspace_ids
        .first()
        .cloned()
        .ok_or(anyhow!("No workspace found"))?;

    let workspace = uow
        .get_workspace(&workspace_id)?
        .ok_or_else(|| anyhow!("Workspace entity not found"))?;
    Ok(workspace)
}

pub fn strip_leading_and_trailing_slashes(path: &str) -> String {
    let trimmed = path.trim_matches(|c: char| c == '/' || c == '\\' || c.is_whitespace());
    trimmed.to_string()
}

/// Pluralizes a single English word (no underscores).
fn pluralize_single(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let lower = word.to_lowercase();

    // Irregular plurals
    let irregular: &[(&str, &str)] = &[
        ("child", "children"),
        ("person", "people"),
        ("man", "men"),
        ("woman", "women"),
        ("mouse", "mice"),
        ("goose", "geese"),
        ("foot", "feet"),
        ("tooth", "teeth"),
        ("ox", "oxen"),
        ("datum", "data"),
        ("index", "indices"),
        ("matrix", "matrices"),
        ("vertex", "vertices"),
        ("appendix", "appendices"),
        ("criterion", "criteria"),
        ("phenomenon", "phenomena"),
        ("medium", "media"),
        ("curriculum", "curricula"),
        ("die", "dice"),
    ];

    for &(singular, plural) in irregular {
        if lower == singular {
            if word.chars().next().unwrap().is_uppercase() {
                let mut chars = plural.chars();
                let first = chars.next().unwrap().to_uppercase().to_string();
                return format!("{}{}", first, chars.as_str());
            }
            return plural.to_string();
        }
    }

    // Uncountable / already-plural words
    let uncountable = [
        "sheep", "fish", "deer", "species", "series", "aircraft", "offspring", "moose",
    ];
    if uncountable.contains(&lower.as_str()) {
        return word.to_string();
    }

    // Words ending in -fe → -ves
    let fe_to_ves = ["knife", "life", "wife", "midwife"];
    if fe_to_ves.contains(&lower.as_str()) {
        let stem = &word[..word.len() - 2];
        return format!("{}ves", stem);
    }

    // Words ending in -f → -ves (common cases)
    let f_to_ves = [
        "leaf", "half", "wolf", "shelf", "self", "calf", "loaf", "thief", "sheaf", "elf",
        "scarf",
    ];
    if f_to_ves.contains(&lower.as_str()) {
        let stem = &word[..word.len() - 1];
        return format!("{}ves", stem);
    }

    // Words ending in -sis or -xis → -ses / -xes (Latin/Greek)
    if lower.ends_with("sis") || lower.ends_with("xis") {
        return format!("{}es", &word[..word.len() - 2]);
    }

    // Words ending in -us → -i (Latin, common cases)
    let us_to_i = [
        "focus", "radius", "fungus", "cactus", "stimulus", "syllabus", "nucleus", "alumnus",
    ];
    if us_to_i.contains(&lower.as_str()) {
        return format!("{}i", &word[..word.len() - 2]);
    }

    // Words ending in -o preceded by a consonant → -oes (common cases)
    let o_to_oes = [
        "hero", "potato", "tomato", "echo", "torpedo", "veto", "embargo", "volcano", "mosquito",
        "cargo",
    ];
    if o_to_oes.contains(&lower.as_str()) {
        return format!("{}es", word);
    }

    // Words ending in 'y' preceded by a consonant: change 'y' to 'ies'
    if word.ends_with('y') && word.len() > 1 {
        let second_last = word.chars().nth(word.len() - 2).unwrap();
        if !"aeiou".contains(second_last) {
            return format!("{}ies", &word[..word.len() - 1]);
        }
    }

    // Words ending in 's', 'x', 'z', 'ch', 'sh': add 'es'
    if word.ends_with('s')
        || word.ends_with('x')
        || word.ends_with('z')
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        return format!("{}es", word);
    }

    // Default case: add 's'
    format!("{}s", word)
}

/// Transforms an English word (possibly snake_case) to its plural form.
/// For snake_case words like "deleted_tag", only the last segment is pluralized → "deleted_tags".
pub fn to_plural(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    if let Some(pos) = word.rfind('_') {
        let prefix = &word[..pos];
        let last = &word[pos + 1..];
        format!("{}_{}", prefix, pluralize_single(last))
    } else {
        pluralize_single(word)
    }
}

pub fn to_sql_safe_identifier(name: &str) -> String {
    // Replace spaces and special characters with underscores
    let mut identifier = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    // Ensure it doesn't start with a digit
    if identifier.chars().next().map_or(false, |c| c.is_digit(10)) {
        identifier = format!("_{}", identifier);
    }

    // avoid reserved keywords
    let reserved_keywords = [
        "ABORT",
        "ACTION",
        "ADD",
        "AFTER",
        "ALL",
        "ALTER",
        "ALWAYS",
        "ANALYZE",
        "AND",
        "AS",
        "ASC",
        "ATTACH",
        "AUTOINCREMENT",
        "BEFORE",
        "BEGIN",
        "BETWEEN",
        "BY",
        "CASCADE",
        "CASE",
        "CAST",
        "CHECK",
        "COLLATE",
        "COLUMN",
        "COMMIT",
        "CONFLICT",
        "CONSTRAINT",
        "CREATE",
        "CROSS",
        "CURRENT",
        "CURRENT_DATE",
        "CURRENT_TIME",
        "CURRENT_TIMESTAMP",
        "DATABASE",
        "DEFAULT",
        "DEFERRABLE",
        "DEFERRED",
        "DELETE",
        "DESC",
        "DETACH",
        "DISTINCT",
        "DO",
        "DROP",
        "EACH",
        "ELSE",
        "END",
        "ESCAPE",
        "EXCEPT",
        "EXCLUDE",
        "EXCLUSIVE",
        "EXISTS",
        "EXPLAIN",
        "FAIL",
        "FILTER",
        "FIRST",
        "FOLLOWING",
        "FOR",
        "FOREIGN",
        "FROM",
        "FULL",
        "GENERATED",
        "GLOB",
        "GROUP",
        "GROUPS",
        "HAVING",
        "IF",
        "IGNORE",
        "IMMEDIATE",
        "IN",
        "INDEX",
        "INDEXED",
        "INITIALLY",
        "INNER",
        "INSERT",
        "INSTEAD",
        "INTERSECT",
        "INTO",
        "IS",
        "ISNULL",
        "JOIN",
        "KEY",
        "LAST",
        "LEFT",
        "LIKE",
        "LIMIT",
        "MATCH",
        "MATERIALIZED",
        "NATURAL",
        "NO",
        "NOT",
        "NOTHING",
        "NOTNULL",
        "NULL",
        "NULLS",
        "OF",
        "OFFSET",
        "ON",
        "OR",
        "ORDER",
        "OTHERS",
        "OUTER",
        "OVER",
        "PARTITION",
        "PLAN",
        "PRAGMA",
        "PRECEDING",
        "PRIMARY",
        "QUERY",
        "RAISE",
        "RANGE",
        "RECURSIVE",
        "REFERENCES",
        "REGEXP",
        "REINDEX",
        "RELEASE",
        "RENAME",
        "REPLACE",
        "RESTRICT",
        "RETURNING",
        "RIGHT",
        "ROLLBACK",
        "ROW",
        "ROWS",
        "SAVEPOINT",
        "SELECT",
        "SET",
        "TABLE",
        "TEMP",
        "TEMPORARY",
        "THEN",
        "TIES",
        "TO",
        "TRANSACTION",
        "TRIGGER",
        "UNBOUNDED",
        "UNION",
        "UNIQUE",
        "UPDATE",
        "USING",
        "VACUUM",
        "VALUES",
        "VIEW",
        "VIRTUAL",
        "WHEN",
        "WHERE",
        "WINDOW",
        "WITH",
        "WITHOUT",
    ];

    if reserved_keywords.contains(&identifier.to_uppercase().as_str()) {
        identifier = format!("_{}", identifier);
    }
    identifier
}

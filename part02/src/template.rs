use crate::cli::TemplateChoice;

/// Resolve what content to write into the file.
///
/// Priority order:
///   1. If the user passed `--content "..."`, use that verbatim.
///   2. If the user passed `--template <name>`, use that template.
///   3. Try to detect the template from the file's extension.
///   4. Fall back to an empty string.
///
/// `crate::` means "start from the root of this crate" — it's how you
/// reference items in sibling modules.
pub fn resolve_content(
    name: &str,
    content: Option<String>,
    template: Option<TemplateChoice>,
) -> String {
    // Priority 1: explicit content wins over everything
    if let Some(text) = content {
        return text;
    }

    // Priority 2: explicit template flag
    if let Some(choice) = template {
        return render_template(choice);
    }

    // Priority 3: detect from extension
    if let Some(detected) = detect_from_extension(name) {
        return render_template(detected);
    }

    // Priority 4: empty file
    String::new()
}

/// Detect a TemplateChoice from the file's extension.
///
/// Returns None if no matching template exists for that extension.
/// This is intentional — not every extension maps to a template.
fn detect_from_extension(name: &str) -> Option<TemplateChoice> {
    // rsplit_once splits on the LAST occurrence of '.'
    // "archive.tar.gz" → Some(("archive.tar", "gz"))
    // "README"         → None
    let ext = name.rsplit_once('.')?.1;

    match ext.to_lowercase().as_str() {
        "json" => Some(TemplateChoice::Json),
        "csv"  => Some(TemplateChoice::Csv),
        "env"  => Some(TemplateChoice::Env),
        "toml" => Some(TemplateChoice::Toml),
        "txt" | "text" | "md" => Some(TemplateChoice::Plain),
        // Any other extension → no template detected
        _ => None,
    }
}

/// Turn a TemplateChoice into actual file content.
pub fn render_template(choice: TemplateChoice) -> String {
    match choice {
        TemplateChoice::Plain => {
            String::from("# New file\n")
        }

        TemplateChoice::Json => {
            // A minimal, valid JSON object skeleton
            String::from("{\n  \"\": \"\"\n}\n")
        }

        TemplateChoice::Csv => {
            // A CSV file with example column headers
            String::from("id,name,value\n1,example,0\n")
        }

        TemplateChoice::Env => {
            // .env format: KEY=VALUE, with commented-out examples
            String::from(
                "# Application environment variables\n\
                 # Copy this file to .env and fill in your values\n\
                 \n\
                 APP_NAME=myapp\n\
                 APP_ENV=development\n\
                 # SECRET_KEY=changeme\n"
            )
        }
        // ← Level 1 implementation
        TemplateChoice::Toml => {
            String::from("[section]\nkey = \"value\"\n" )
            }
    }
}


/// List all templates with their example output (for --list-templates)
pub fn list_all_templates() -> Vec<(TemplateChoice, &'static str)> {
    use TemplateChoice::*;
    vec![
        (Plain, "# New file\n"),
        (Json, "{\n  \"\": \"\"\n}\n"),
        (Csv, "id,name,value\n1,example,0\n"),
        (Env, "# APP_NAME=myapp\nAPP_ENV=development\n"),
        (Toml, "[section]\nkey = \"value\"\n"),
    ]
}

// ─── Unit tests ──────────────────────────────────────────────────────────────
//
// Tests live right next to the code they test in Rust. The `#[cfg(test)]`
// attribute means this block is compiled ONLY when running `cargo test` —
// it does not end up in your release binary.

#[cfg(test)]
mod tests {
    use super::*; // bring everything from the parent module into scope

    #[test]
    fn detect_json_extension() {
        let result = detect_from_extension("config.json");
        assert!(matches!(result, Some(TemplateChoice::Json)));
    }

    #[test]
    fn detect_toml_extension() {
        let result = detect_from_extension("config.toml");
        assert!(matches!(result, Some(TemplateChoice::Toml)));
    }

    #[test]
    fn render_toml_template() {
        let content = render_template(TemplateChoice::Toml);
        assert!(content.contains("[section]"));
        assert!(content.contains("key = \"value\""));
    }

    #[test]
    fn detect_env_extension() {
        let result = detect_from_extension(".env");
        // ".env" has no text before the dot, so rsplit_once('.') gives ("", "env")
        assert!(matches!(result, Some(TemplateChoice::Env)));
    }

    #[test]
    fn unknown_extension_returns_none() {
        let result = detect_from_extension("archive.zip");
        assert!(result.is_none());
    }

    #[test]
    fn no_extension_returns_none() {
        let result = detect_from_extension("Makefile");
        assert!(result.is_none());
    }

    #[test]
    fn explicit_content_wins_over_template() {
        let content = resolve_content(
            "data.json",
            Some("custom content".to_string()),
            Some(TemplateChoice::Json),
        );
        assert_eq!(content, "custom content");
    }

    #[test]
    fn explicit_template_wins_over_extension() {
        // File is named .env but user forced --template csv
        let content = resolve_content(
            ".env",
            None,
            Some(TemplateChoice::Csv),
        );
        assert!(content.contains("id,name,value"));
    }

    #[test]
    fn extension_detection_fallback() {
        // No content, no explicit template → detect from .csv extension
        let content = resolve_content("report.csv", None, None);
        assert!(content.contains("id,name,value"));
    }

    #[test]
    fn empty_for_unknown_extension() {
        let content = resolve_content("archive.zip", None, None);
        assert_eq!(content, "");
    }
}
use crate::app_context::AppContext;
use crate::cli::{DocsArgs, DocsTarget, OutputContext};
use include_dir::{Dir, include_dir};
use std::sync::Arc;

static DOCS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../docs/");

static README: &str = include_str!("../../../../README.md");

pub fn execute(
    app_context: &Arc<AppContext>,
    args: &DocsArgs,
    output: &OutputContext,
) -> anyhow::Result<()> {
    let target = args.target.as_ref().cloned().unwrap_or(DocsTarget::All);

    let skin = termimad::MadSkin::default();

    // When showing all docs, prepend a listing of available commands
    if !output.quiet {
        let listing = "\
**Available documentation commands:**

|:-|:-|:-|
|`qleany docs introduction`|`intro`|Introduction (README)|
|`qleany docs quick-start-rust`|`start-rust`|Quick Start — Rust|
|`qleany docs quick-start-cpp-qt`|`start-cpp`|Quick Start — C++/Qt|
|`qleany docs manifest-reference`|`manifest`|Manifest Reference|
|`qleany docs design-philosophy`|`design`|Design Philosophy|
|`qleany docs regeneration-workflow`|`regen`|Regeneration Workflow|
|`qleany docs undo-redo-architecture`|`undo`|Undo/Redo Architecture|
|`qleany docs qml-integration`|`qml`|QML Integration|
|`qleany docs generated-code-cpp-qt`|`cpp`|Generated Code — C++/Qt|
|`qleany docs generated-code-rust`|`rust`|Generated Code — Rust|
|`qleany docs troubleshooting`|`trouble`|Troubleshooting|
|-|-|-|

---
";
        skin.print_text(listing);
        println!();
    }

    // Handle Intro specially (README.md is not in docs/)
    if matches!(target, DocsTarget::Introduction | DocsTarget::All) {
        if !output.quiet {
            skin.print_text(README);
            println!();
        }
        if matches!(target, DocsTarget::Introduction) {
            return Ok(());
        }
    }

    let files: Vec<&str> = match target {
        DocsTarget::All => vec![
            "quick-start-rust.md",
            "quick-start-cpp-qt.md",
            "manifest-reference.md",
            "design-philosophy.md",
            "regeneration-workflow.md",
            "undo-redo-architecture.md",
            "qml-integration.md",
            "generated-code-cpp-qt.md",
            "generated-code-rust.md",
            "troubleshooting.md",
        ],
        DocsTarget::Introduction => unreachable!(),
        DocsTarget::ManifestReference => vec!["manifest-reference.md"],
        DocsTarget::DesignPhilosophy => vec!["design-philosophy.md"],
        DocsTarget::UndoRedoArchitecture => vec!["undo-redo-architecture.md"],
        DocsTarget::GeneratedCodeCppQt => vec!["generated-code-cpp-qt.md"],
        DocsTarget::GeneratedCodeRust => vec!["generated-code-rust.md"],
        DocsTarget::QuickStartCppQt => vec!["quick-start-cpp-qt.md"],
        DocsTarget::QuickStartRust => vec!["quick-start-rust.md"],
        DocsTarget::QmlIntegration => vec!["qml-integration.md"],
        DocsTarget::Troubleshooting => vec!["troubleshooting.md"],
        DocsTarget::RegenerationWorkflow => vec!["regeneration-workflow.md"],
    };

    for filename in files {
        match DOCS_DIR.get_file(filename) {
            Some(file) => {
                let content = file.contents_utf8().unwrap_or("[binary content]");
                if !output.quiet {
                    skin.print_text(content);
                    println!();
                }
            }
            None => {
                output.warn(&format!("Documentation file not found: {}", filename));
            }
        }
    }

    Ok(())
}

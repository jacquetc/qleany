use std::collections::BTreeMap;

/// Represents a node in the file tree structure.
/// Uses BTreeMap to maintain alphabetical ordering of children.
#[derive(Default)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
    is_file: bool,
}

impl TreeNode {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            is_file: false,
        }
    }

    /// Inserts a path into the tree, creating intermediate nodes as needed.
    fn insert(&mut self, path: &str) {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        self.insert_parts(&parts, 0);
    }

    fn insert_parts(&mut self, parts: &[&str], index: usize) {
        if index >= parts.len() {
            return;
        }

        let part = parts[index];
        let child = self.children.entry(part.to_string()).or_default();

        if index == parts.len() - 1 {
            child.is_file = true;
        } else {
            child.insert_parts(parts, index + 1);
        }
    }
}

/// Renders a list of file paths as an ASCII tree.
///
/// # Arguments
///
/// * `files` - A slice of file path strings (e.g., `["src/main.rs", "src/lib.rs"]`)
///
/// # Returns
///
/// A `String` containing the formatted ASCII tree representation.
///
/// # Example
///
/// ```
/// let files = vec![
///     "src/main.rs",
///     "src/cli/mod.rs",
///     "src/cli/args.rs",
///     "Cargo.toml",
/// ];
/// let tree = render_file_tree(&files);
/// println!("{}", tree);
/// ```
///
/// Output:
/// ```text
/// Cargo.toml
/// src/
/// ├── cli/
/// │   ├── args.rs
/// │   └── mod.rs
/// └── main.rs
/// ```
pub fn render_file_tree(files: &[impl AsRef<str>]) -> String {
    let mut root = TreeNode::new();

    for file in files {
        root.insert(file.as_ref());
    }

    let mut output = String::new();
    render_children(&root, "", &mut output);
    output
}

/// Renders a list of file paths as an ASCII tree with a root label.
///
/// # Arguments
///
/// * `files` - A slice of file path strings
/// * `root_label` - Label to display as the tree root (e.g., project name)
///
/// # Returns
///
/// A `String` containing the formatted ASCII tree with the root label.
pub fn render_file_tree_with_root(files: &[impl AsRef<str>], root_label: &str) -> String {
    let mut root = TreeNode::new();

    for file in files {
        root.insert(file.as_ref());
    }

    let mut output = String::new();
    output.push_str(root_label);
    output.push('\n');
    render_children(&root, "", &mut output);
    output
}

fn render_children(node: &TreeNode, prefix: &str, output: &mut String) {
    let children: Vec<_> = node.children.iter().collect();
    let count = children.len();

    for (index, (name, child)) in children.into_iter().enumerate() {
        let is_last = index == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        // Append directory indicator if this node has children
        let display_name = if child.children.is_empty() {
            name.clone()
        } else {
            format!("{}/", name)
        };

        output.push_str(prefix);
        output.push_str(connector);
        output.push_str(&display_name);
        output.push('\n');

        // Recursively render children
        if !child.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            render_children(child, &new_prefix, output);
        }
    }
}

/// Renders a file tree to stdout for convenience.
pub fn print_file_tree(files: &[impl AsRef<str>]) {
    print!("{}", render_file_tree(files));
}

/// Renders a file tree with root label to stdout for convenience.
pub fn print_file_tree_with_root(files: &[impl AsRef<str>], root_label: &str) {
    print!("{}", render_file_tree_with_root(files, root_label));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_file() {
        let files = vec!["main.rs"];
        let result = render_file_tree(&files);
        assert_eq!(result, "└── main.rs\n");
    }

    #[test]
    fn test_flat_structure() {
        let files = vec!["Cargo.toml", "README.md", "main.rs"];
        let result = render_file_tree(&files);
        let expected = "\
├── Cargo.toml
├── main.rs
└── README.md
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_structure() {
        let files = vec![
            "src/main.rs",
            "src/lib.rs",
            "src/cli/mod.rs",
            "src/cli/args.rs",
            "Cargo.toml",
        ];
        let result = render_file_tree(&files);
        let expected = "\
├── Cargo.toml
└── src/
    ├── cli/
    │   ├── args.rs
    │   └── mod.rs
    ├── lib.rs
    └── main.rs
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deeply_nested() {
        let files = vec!["a/b/c/d/file.txt", "a/b/c/other.txt", "a/b/sibling.txt"];
        let result = render_file_tree(&files);
        let expected = "\
└── a/
    └── b/
        ├── c/
        │   ├── d/
        │   │   └── file.txt
        │   └── other.txt
        └── sibling.txt
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_root_label() {
        let files = vec!["src/main.rs", "Cargo.toml"];
        let result = render_file_tree_with_root(&files, "my-project");
        let expected = "\
my-project
├── Cargo.toml
└── src/
    └── main.rs
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let files: Vec<&str> = vec![];
        let result = render_file_tree(&files);
        assert_eq!(result, "");
    }

    #[test]
    fn test_string_slices() {
        let files = vec![String::from("src/main.rs"), String::from("Cargo.toml")];
        let result = render_file_tree(&files);
        assert!(result.contains("src/"));
        assert!(result.contains("main.rs"));
        assert!(result.contains("Cargo.toml"));
    }

    #[test]
    fn test_multiple_root_directories() {
        let files = vec![
            "docs/guide.md",
            "docs/api.md",
            "src/main.rs",
            "tests/test_main.rs",
        ];
        let result = render_file_tree(&files);
        let expected = "\
├── docs/
│   ├── api.md
│   └── guide.md
├── src/
│   └── main.rs
└── tests/
    └── test_main.rs
";
        assert_eq!(result, expected);
    }
}

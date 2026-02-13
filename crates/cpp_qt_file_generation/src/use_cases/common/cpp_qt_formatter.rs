use std::borrow::Cow;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Run clang-format on multiple files in a single invocation (much faster than per-file).
/// This formats files in-place. If clang-format is not available, this is a no-op.
pub fn clang_format_files_batch(files: &[PathBuf]) {
    if files.is_empty() {
        return;
    }

    let clang_format_path = match find_clang_format() {
        Ok(p) => p,
        Err(_) => return, // clang-format not found, skip formatting
    };

    // clang-format can handle many files at once, but there may be OS limits on command line length.
    // Process files in batches to avoid hitting argument limits.
    const BATCH_SIZE: usize = 100;

    for chunk in files.chunks(BATCH_SIZE) {
        let mut cmd = Command::new(&clang_format_path);
        cmd.arg("-i");
        cmd.arg("--style=Microsoft");

        // Add all file paths as arguments
        for file in chunk {
            cmd.arg(file);
        }

        // Run clang-format and ignore errors (best-effort formatting)
        let _ = cmd.status();
    }
}

/// Find the clang-format executable path.
pub(crate) fn find_clang_format() -> io::Result<PathBuf> {
    // 1) Respect CLANG_FORMAT env var if set
    if let Ok(clang_format) = env::var("CLANG_FORMAT") {
        let pb: PathBuf = clang_format.into();
        return Ok(pb);
    }
    // 2) Try `which` crate to find it in PATH
    match which::which("clang-format") {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("clang-format not found: {}", e),
        )),
    }
}

/// Run clang-format on the provided source and return the formatted code.
/// If clang-format is not available or fails, returns the original source.
pub(crate) fn clang_format_string<'a>(
    source: &'a str,
    _config_path: Option<&'a str>,
) -> Cow<'a, str> {
    // Best-effort: if we can't find clang-format, just return the original
    let clang_format_path = match find_clang_format() {
        Ok(p) => p,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut cmd = Command::new(clang_format_path);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    cmd.arg("--style=Microsoft");

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut stdin = child.stdin.take();
    let stdout = child.stdout.take();

    // Write the source to clang-format stdin
    if let Some(mut child_stdin) = stdin.take() {
        let _ = child_stdin.write_all(source.as_bytes());
    }

    // Read formatted output
    let mut output = Vec::new();
    if let Some(mut child_stdout) = stdout {
        let _ = io::copy(&mut child_stdout, &mut output);
    }

    let status = match child.wait() {
        Ok(s) => s,
        Err(_) => return Cow::Borrowed(source),
    };

    match String::from_utf8(output) {
        Ok(formatted) => {
            if status.success() {
                Cow::Owned(formatted)
            } else {
                Cow::Borrowed(source)
            }
        }
        Err(_) => Cow::Borrowed(source),
    }
}
#[cfg(test)]
mod tests {
    use super::{clang_format_string, find_clang_format};

    #[test]
    fn test_find_clang_format_best_effort() {
        // We can't guarantee clang-format is installed on CI, but the function should not panic
        let _ = find_clang_format();
    }

    #[test]
    fn test_clang_format_string_idempotent_when_not_found_or_on_error() {
        let src = "int main() { return 0; }\n";
        let formatted = clang_format_string(src, None);
        // If clang-format is present, the result should differ (depending on style);
        // if not, it should be equal. In both cases it must be valid UTF-8 and non-empty.
        assert!(!formatted.is_empty());
    }
}

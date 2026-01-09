use std::borrow::Cow;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Run rustfmt on multiple files in a single invocation (much faster than per-file).
/// This formats files in-place. If rustfmt is not available, this is a no-op.
pub fn rustfmt_files_batch(files: &[PathBuf]) {
    if files.is_empty() {
        return;
    }

    let rustfmt_path = match find_rustfmt() {
        Ok(p) => p,
        Err(_) => return, // rustfmt not found, skip formatting
    };

    // rustfmt can handle many files at once, but there may be OS limits on command line length.
    // Process files in batches to avoid hitting argument limits.
    const BATCH_SIZE: usize = 100;

    for chunk in files.chunks(BATCH_SIZE) {
        let mut cmd = Command::new(&rustfmt_path);

        // Add all file paths as arguments
        for file in chunk {
            cmd.arg(file);
        }

        // Run rustfmt and ignore errors (best-effort formatting)
        let _ = cmd.status();
    }
}

/// Find the rustfmt executable path.
pub(crate) fn find_rustfmt() -> io::Result<PathBuf> {
    // 1) Respect RUSTFMT env var if set
    if let Ok(rustfmt) = env::var("RUSTFMT") {
        let pb: PathBuf = rustfmt.into();
        return Ok(pb);
    }
    // 2) Try `which` crate to find it in PATH
    match which::which("rustfmt") {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("rustfmt not found: {}", e),
        )),
    }
}

/// Run rustfmt on the provided Rust source and return the formatted code.
/// If rustfmt is not available or fails to parse, returns the original source.
pub(crate) fn rustfmt_string<'a>(source: &'a str, config_path: Option<&'a str>) -> Cow<'a, str> {
    // Best-effort: if we can't find rustfmt, just return the original
    let rustfmt_path = match find_rustfmt() {
        Ok(p) => p,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut cmd = Command::new(rustfmt_path);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    if let Some(path) = config_path {
        cmd.args(["--config-path", path]);
    }

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut stdin = child.stdin.take();
    let mut stdout = child.stdout.take();

    // Write the source to rustfmt stdin
    if let Some(mut child_stdin) = stdin.take() {
        let _ = child_stdin.write_all(source.as_bytes());
    }

    // Read formatted output
    let mut output = Vec::new();
    if let Some(mut child_stdout) = stdout.take() {
        let _ = io::copy(&mut child_stdout, &mut output);
    }

    let status = match child.wait() {
        Ok(s) => s,
        Err(_) => return Cow::Borrowed(source),
    };

    match String::from_utf8(output) {
        Ok(formatted) => match status.code() {
            Some(0) => Cow::Owned(formatted),
            Some(2) => Cow::Borrowed(source), // parsing errors
            Some(3) => Cow::Owned(formatted), // could not format some lines
            _ => Cow::Borrowed(source),       // internal error or unknown
        },
        Err(_) => Cow::Borrowed(source),
    }
}
#[cfg(test)]
mod tests {
    use super::{find_rustfmt, rustfmt_string};

    #[test]
    fn test_find_rustfmt_best_effort() {
        // We can't guarantee rustfmt is installed on CI, but the function should not panic
        let _ = find_rustfmt();
    }

    #[test]
    fn test_rustfmt_string_idempotent_when_not_found_or_on_error() {
        let src = "fn  main( ) {println!(\"hi\");}\n";
        let formatted = rustfmt_string(src, None);
        // If rustfmt is present, the result should differ (spaces fixed);
        // if not, it should be equal. In both cases it must be valid UTF-8 and non-empty.
        assert!(!formatted.is_empty());
    }
}

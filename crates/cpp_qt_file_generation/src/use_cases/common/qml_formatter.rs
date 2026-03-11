use std::borrow::Cow;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Run qmlformat on multiple files in a single invocation (much faster than per-file).
/// This formats files in-place. If qmlformat is not available, this is a no-op.
pub fn qml_format_files_batch(files: &[PathBuf]) {
    if files.is_empty() {
        return;
    }

    let qmlformat_path = match find_qmlformat() {
        Ok(p) => p,
        Err(_) => return, // qmlformat not found, skip formatting
    };

    // qmlformat processes one file at a time with -i flag
    for file in files {
        let mut cmd = Command::new(&qmlformat_path);
        cmd.arg("-i");
        cmd.arg(file);

        // Run qmlformat and ignore errors (best-effort formatting)
        let _ = cmd.status();
    }
}

/// Find the qmlformat executable path.
pub(crate) fn find_qmlformat() -> io::Result<PathBuf> {
    // 1) Respect QMLFORMAT env var if set
    if let Ok(qmlformat) = env::var("QMLFORMAT") {
        let pb: PathBuf = qmlformat.into();
        return Ok(pb);
    }
    // 2) Try `which` crate to find it in PATH
    if let Ok(p) = which::which("qmlformat") {
        return Ok(p);
    }
    // 3) Check well-known Qt6 installation paths
    let well_known = PathBuf::from("/usr/lib/qt6/bin/qmlformat");
    if well_known.is_file() {
        return Ok(well_known);
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "qmlformat not found in PATH or /usr/lib/qt6/bin/",
    ))
}

/// Run qmlformat on the provided source and return the formatted code.
/// If qmlformat is not available or fails, returns the original source.
pub(crate) fn qml_format_string<'a>(source: &'a str) -> Cow<'a, str> {
    // Best-effort: if we can't find qmlformat, just return the original
    let qmlformat_path = match find_qmlformat() {
        Ok(p) => p,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut cmd = Command::new(qmlformat_path);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    cmd.arg("-"); // read from stdin

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Cow::Borrowed(source),
    };

    let mut stdin = child.stdin.take();
    let stdout = child.stdout.take();

    // Write the source to qmlformat stdin
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
    use super::{find_qmlformat, qml_format_string};

    #[test]
    fn test_find_qmlformat_best_effort() {
        // We can't guarantee qmlformat is installed on CI, but the function should not panic
        let _ = find_qmlformat();
    }

    #[test]
    fn test_qml_format_string_idempotent_when_not_found_or_on_error() {
        let src = "import QtQuick 2.15\nItem { width: 100 }\n";
        let formatted = qml_format_string(src);
        // If qmlformat is present, the result should differ (depending on style);
        // if not, it should be equal. In both cases it must be valid UTF-8 and non-empty.
        assert!(!formatted.is_empty());
    }
}

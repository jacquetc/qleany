fn main() {
    // Warn developers when building with Slint live preview enabled; the live preview
    // runtime can emit transient ids like 0 for UI interactions which may be
    // interpreted as invalid entity ids by the application. This warning helps
    // identify that configuration during development.
    if std::env::var("SLINT_LIVE_PREVIEW").ok().as_deref() == Some("1") {
        // Cargo build scripts can emit warnings via println!("cargo:warning=...");
        println!(
            "cargo:warning=SLINT_LIVE_PREVIEW is set: live preview builds may emit transient ids (e.g., 0) that are not valid entity ids. Consider disabling live preview for normal runs."
        );
    }

    slint_build::compile("ui/app.slint").unwrap();
}

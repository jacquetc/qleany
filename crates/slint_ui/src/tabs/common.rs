use crate::app_context::AppContext;
use crate::commands::workspace_commands;
use crate::{App, AppState};
use common::types::EntityId;
use slint::ComponentHandle;
use std::sync::Arc;

/// Helper function to get the global_id from root
pub fn get_global_id(app: &App, app_context: &Arc<AppContext>) -> Option<EntityId> {
    let workspace_id = app.global::<AppState>().get_workspace_id() as EntityId;
    if workspace_id > 0
        && let Ok(Some(workspace)) = workspace_commands::get_workspace(app_context, &workspace_id)
        && workspace.global > 0
    {
        log::trace!("Found global_id: {}", workspace.global);
        return Some(workspace.global);
    }
    None
}

/// Copy text to the system clipboard.
///
/// On Linux (X11/Wayland) the clipboard is "owned" by the process, so we must
/// keep the `Clipboard` object alive long enough for clipboard managers to
/// grab the contents. We use `SetExtLinux::wait_until` on a background thread
/// to serve clipboard requests for a few seconds without blocking the UI.
pub fn set_clipboard_text(text: String) {
    std::thread::spawn(move || {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                #[cfg(target_os = "linux")]
                {
                    use arboard::SetExtLinux;
                    use std::time::{Duration, Instant};

                    let deadline = Instant::now() + Duration::from_secs(5);
                    if let Err(e) = clipboard.set().wait_until(deadline).text(text) {
                        log::error!("Failed to set clipboard text: {}", e);
                    }
                }
                #[cfg(not(target_os = "linux"))]
                {
                    if let Err(e) = clipboard.set_text(text) {
                        log::error!("Failed to set clipboard text: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to access clipboard: {}", e);
            }
        }
    });
}

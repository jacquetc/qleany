//! Check Widget module
//!
//! Runs manifest validation checks with debouncing and updates the
//! floating check indicator (FAB + side panel) in the Slint UI.

use crate::app_context::AppContext;
use crate::commands::handling_manifest_commands;
use crate::event_hub_client::EventHubClient;
use crate::{App, AppState, CheckState};
use common::event::{DirectAccessEntity, EntityEvent, HandlingManifestEvent, Origin};
use slint::{ComponentHandle, Timer, TimerMode, Weak};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const DEBOUNCE_MS: u64 = 500;

fn run_check(app: &App, app_context: &Arc<AppContext>) {
    if !app.global::<AppState>().get_manifest_is_open() {
        return;
    }

    match handling_manifest_commands::check(app_context) {
        Ok(result) => {
            let warning_count = result.warnings.len() as i32;
            let critical_count = result.critical_errors.len() as i32;

            let status = if critical_count > 0 {
                "critical"
            } else if warning_count > 0 {
                "warning"
            } else {
                "ok"
            };

            let state = app.global::<CheckState>();
            state.set_check_status(status.into());
            state.set_warning_count(warning_count);
            state.set_critical_count(critical_count);

            let warnings: Vec<slint::SharedString> =
                result.warnings.into_iter().map(|s| s.into()).collect();
            let criticals: Vec<slint::SharedString> = result
                .critical_errors
                .into_iter()
                .map(|s| s.into())
                .collect();

            state.set_warnings(Rc::new(slint::VecModel::from(warnings)).into());
            state.set_critical_errors(Rc::new(slint::VecModel::from(criticals)).into());

            log::info!(
                "Check completed: {} critical, {} warnings",
                critical_count,
                warning_count
            );
        }
        Err(e) => {
            log::error!("Check failed: {}", e);
        }
    }
}

fn clear_check(app: &App) {
    let state = app.global::<CheckState>();
    state.set_check_status("none".into());
    state.set_warning_count(0);
    state.set_critical_count(0);
    state.set_warnings(Rc::new(slint::VecModel::from(Vec::<slint::SharedString>::new())).into());
    state.set_critical_errors(
        Rc::new(slint::VecModel::from(Vec::<slint::SharedString>::new())).into(),
    );
    state.set_panel_visible(false);
}

/// UI-thread-local state holding the debounce timer.
struct CheckTimerState {
    timer: Timer,
}

thread_local! {
    static CHECK_TIMER: RefCell<Option<CheckTimerState>> = const { RefCell::new(None) };
}

fn ensure_timer_initialized() {
    CHECK_TIMER.with(|cell| {
        if cell.borrow().is_none() {
            *cell.borrow_mut() = Some(CheckTimerState {
                timer: Timer::default(),
            });
        }
    });
}

fn schedule_check_debounced(app_weak: &Weak<App>, app_context: &Arc<AppContext>) {
    ensure_timer_initialized();

    let app_weak = app_weak.clone();
    let ctx = Arc::clone(app_context);

    CHECK_TIMER.with(|cell| {
        if let Some(state) = cell.borrow().as_ref() {
            state.timer.start(
                TimerMode::SingleShot,
                std::time::Duration::from_millis(DEBOUNCE_MS),
                move || {
                    if let Some(app) = app_weak.upgrade() {
                        run_check(&app, &ctx);
                    }
                },
            );
        }
    });
}

pub fn init(event_hub_client: &EventHubClient, app: &App, app_context: &Arc<AppContext>) {
    // Manual re-check callback
    app.global::<CheckState>().on_request_check({
        let ctx = Arc::clone(app_context);
        let app_weak = app.as_weak();
        move || {
            if let Some(app) = app_weak.upgrade() {
                run_check(&app, &ctx);
            }
        }
    });

    // Clear check on manifest close
    event_hub_client.subscribe(Origin::HandlingManifest(HandlingManifestEvent::Close), {
        let app_weak = app.as_weak();
        move |_event| {
            let app_weak = app_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak.upgrade() {
                    clear_check(&app);
                }
            });
        }
    });

    // Subscribe to all events that should trigger a debounced check
    let triggers = vec![
        Origin::HandlingManifest(HandlingManifestEvent::Load),
        Origin::HandlingManifest(HandlingManifestEvent::Create),
        Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::Entity(EntityEvent::Removed)),
        Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::Field(EntityEvent::Removed)),
        Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Removed)),
        Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::UseCase(EntityEvent::Removed)),
        Origin::DirectAccess(DirectAccessEntity::Workspace(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::Global(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::Dto(EntityEvent::Updated)),
        Origin::DirectAccess(DirectAccessEntity::DtoField(EntityEvent::Updated)),
    ];

    for origin in triggers {
        event_hub_client.subscribe(origin, {
            let app_weak = app.as_weak();
            let ctx = Arc::clone(app_context);
            move |_event| {
                let app_weak = app_weak.clone();
                let ctx = Arc::clone(&ctx);
                let _ = slint::invoke_from_event_loop(move || {
                    schedule_check_debounced(&app_weak, &ctx);
                });
            }
        });
    }
}

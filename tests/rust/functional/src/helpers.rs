// Shared test helpers: context creation, scaffold builders.

use common::database::db_context::DbContext;
use common::event::EventHub;
use common::undo_redo::UndoRedoManager;
use std::sync::Arc;

use direct_access::*;

/// All the infrastructure a test needs.
pub struct TestContext {
    pub db: DbContext,
    pub hub: Arc<EventHub>,
    pub undo: UndoRedoManager,
}

impl TestContext {
    pub fn new() -> Self {
        let db = DbContext::new().expect("Failed to create in-memory DB");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        TestContext { db, hub, undo }
    }
}

/// IDs returned by `create_scaffold`.
pub struct Scaffold {
    pub root_id: u64,
    pub workspace_id: u64,
    pub project_id: u64,
}

/// Build Root → Workspace → Project.
pub fn create_scaffold(ctx: &mut TestContext) -> Scaffold {
    // Root (non-undoable orphan)
    let root = root_controller::create_orphan(
        &ctx.db,
        &ctx.hub,
        &CreateRootDto::default(),
    )
    .unwrap();

    // Workspace (undoable, child of Root)
    let ws = workspace_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &CreateWorkspaceDto::default(),
        root.id,
        -1,
    )
    .unwrap();

    // Project (undoable, child of Workspace)
    let proj = project_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &CreateProjectDto {
            title: "TestProject".into(),
            ..Default::default()
        },
        ws.id,
        -1,
    )
    .unwrap();

    Scaffold {
        root_id: root.id,
        workspace_id: ws.id,
        project_id: proj.id,
    }
}

/// Create a task under the given project.
pub fn create_task(ctx: &mut TestContext, project_id: u64, title: &str) -> u64 {
    let dto = CreateTaskDto {
        title: title.into(),
        ..Default::default()
    };
    let task = task_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &dto,
        project_id,
        -1,
    )
    .unwrap();
    task.id
}

/// Create a tag under the given workspace.
pub fn create_tag(ctx: &mut TestContext, ws_id: u64, name: &str, color: &str) -> u64 {
    let dto = CreateTagDto {
        name: name.into(),
        color: color.into(),
        ..Default::default()
    };
    let tag = tag_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &dto,
        ws_id,
        -1,
    )
    .unwrap();
    tag.id
}

/// Create a comment under the given task.
pub fn create_comment(ctx: &mut TestContext, task_id: u64, text: &str) -> u64 {
    let dto = CreateCommentDto {
        text: text.into(),
        author_name: "Tester".into(),
        ..Default::default()
    };
    let comment = comment_controller::create(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &dto,
        task_id,
        -1,
    )
    .unwrap();
    comment.id
}

// Functional tests for Undo/Redo snapshot/restore system.
// Builds entity trees and verifies undo/redo correctly restores
// entities and relationships.

use crate::helpers::{self, TestContext};
use direct_access::*;

fn setup() -> (TestContext, helpers::Scaffold) {
    let mut ctx = TestContext::new();
    let scaffold = helpers::create_scaffold(&mut ctx);
    (ctx, scaffold)
}

// ---------------------------------------------------------------------------
// Basic undo/redo
// ---------------------------------------------------------------------------

#[test]
fn test_undo_create_task() {
    let (mut ctx, s) = setup();
    let task_id = helpers::create_task(&mut ctx, s.project_id, "UndoMe");
    assert!(task_controller::get(&ctx.db, &task_id).unwrap().is_some());

    ctx.undo.undo(None).unwrap();

    assert!(task_controller::get(&ctx.db, &task_id).unwrap().is_none());
    let rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert!(!rel.contains(&task_id));
}

#[test]
fn test_redo_create_task() {
    let (mut ctx, s) = setup();
    let _task_id = helpers::create_task(&mut ctx, s.project_id, "RedoMe");

    ctx.undo.undo(None).unwrap();
    ctx.undo.redo(None).unwrap();

    let rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert!(!rel.is_empty());
    let restored = task_controller::get(&ctx.db, &rel[0]).unwrap().unwrap();
    assert_eq!(restored.title, "RedoMe");
}

#[test]
fn test_undo_remove_task() {
    let (mut ctx, s) = setup();
    let task_id = helpers::create_task(&mut ctx, s.project_id, "RemoveAndRestore");

    task_controller::remove(&ctx.db, &ctx.hub, &mut ctx.undo, None, &task_id).unwrap();
    assert!(task_controller::get(&ctx.db, &task_id).unwrap().is_none());

    ctx.undo.undo(None).unwrap();

    let fetched = task_controller::get(&ctx.db, &task_id).unwrap();
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().title, "RemoveAndRestore");

    let rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert!(rel.contains(&task_id));
}

#[test]
fn test_undo_update_task() {
    let (mut ctx, s) = setup();
    let task_id = helpers::create_task(&mut ctx, s.project_id, "OriginalTitle");

    let mut dto = task_controller::get(&ctx.db, &task_id).unwrap().unwrap();
    dto.title = "UpdatedTitle".into();
    dto.content = "UpdatedContent".into();
    task_controller::update(&ctx.db, &ctx.hub, &mut ctx.undo, None, &dto).unwrap();

    ctx.undo.undo(None).unwrap();

    let fetched = task_controller::get(&ctx.db, &task_id).unwrap().unwrap();
    assert_eq!(fetched.title, "OriginalTitle");
}

// ---------------------------------------------------------------------------
// Relationship undo/redo
// ---------------------------------------------------------------------------

#[test]
fn test_undo_set_relationship_ids() {
    let (mut ctx, s) = setup();
    let tag_a = helpers::create_tag(&mut ctx, s.workspace_id, "TagA", "#AA0000");
    let tag_b = helpers::create_tag(&mut ctx, s.workspace_id, "TagB", "#00BB00");

    project_controller::set_relationship(
        &ctx.db, &ctx.hub, &mut ctx.undo, None,
        &ProjectRelationshipDto {
            id: s.project_id,
            field: ProjectRelationshipField::Tags,
            right_ids: vec![tag_a, tag_b],
        },
    ).unwrap();

    let rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tags,
    ).unwrap();
    assert_eq!(rel.len(), 2);

    ctx.undo.undo(None).unwrap();

    let after = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tags,
    ).unwrap();
    assert!(after.is_empty());
}

#[test]
fn test_undo_move_relationship_ids() {
    let (mut ctx, s) = setup();
    let a = helpers::create_task(&mut ctx, s.project_id, "A");
    let b = helpers::create_task(&mut ctx, s.project_id, "B");
    let c = helpers::create_task(&mut ctx, s.project_id, "C");

    let orig = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert_eq!(orig, vec![a, b, c]);

    project_controller::move_relationship(
        &ctx.db, &ctx.hub, &mut ctx.undo, None,
        &s.project_id, &ProjectRelationshipField::Tasks, &[c], 0,
    ).unwrap();

    let moved = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert_eq!(moved, vec![c, a, b]);

    ctx.undo.undo(None).unwrap();

    let restored = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert_eq!(restored, vec![a, b, c]);
}

// ---------------------------------------------------------------------------
// Cascade delete undo
// ---------------------------------------------------------------------------

#[test]
fn test_undo_cascade_remove_project() {
    let (mut ctx, s) = setup();
    let task_a = helpers::create_task(&mut ctx, s.project_id, "CascadeA");
    let task_b = helpers::create_task(&mut ctx, s.project_id, "CascadeB");
    let comment_id = helpers::create_comment(&mut ctx, task_a, "Important");

    project_controller::remove(&ctx.db, &ctx.hub, &mut ctx.undo, None, &s.project_id).unwrap();

    assert!(project_controller::get(&ctx.db, &s.project_id).unwrap().is_none());
    assert!(task_controller::get(&ctx.db, &task_a).unwrap().is_none());
    assert!(task_controller::get(&ctx.db, &task_b).unwrap().is_none());
    assert!(comment_controller::get(&ctx.db, &comment_id).unwrap().is_none());

    ctx.undo.undo(None).unwrap();

    let proj = project_controller::get(&ctx.db, &s.project_id).unwrap().unwrap();
    assert_eq!(proj.title, "TestProject");

    assert_eq!(task_controller::get(&ctx.db, &task_a).unwrap().unwrap().title, "CascadeA");
    assert_eq!(task_controller::get(&ctx.db, &task_b).unwrap().unwrap().title, "CascadeB");
    assert_eq!(comment_controller::get(&ctx.db, &comment_id).unwrap().unwrap().text, "Important");

    let task_rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert!(task_rel.contains(&task_a));
    assert!(task_rel.contains(&task_b));

    let comment_rel = task_controller::get_relationship(
        &ctx.db, &task_a, &TaskRelationshipField::Comments,
    ).unwrap();
    assert!(comment_rel.contains(&comment_id));
}

// ---------------------------------------------------------------------------
// Multiple operations
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_undo_redo() {
    let (mut ctx, s) = setup();

    // Clear undo stack from scaffold operations
    ctx.undo.clear_all_stacks();

    let a = helpers::create_task(&mut ctx, s.project_id, "MultiA");
    let b = helpers::create_task(&mut ctx, s.project_id, "MultiB");

    let mut dto = task_controller::get(&ctx.db, &a).unwrap().unwrap();
    dto.title = "MultiA_Updated".into();
    task_controller::update(&ctx.db, &ctx.hub, &mut ctx.undo, None, &dto).unwrap();

    // Undo update
    ctx.undo.undo(None).unwrap();
    assert_eq!(task_controller::get(&ctx.db, &a).unwrap().unwrap().title, "MultiA");

    // Redo update
    ctx.undo.redo(None).unwrap();
    assert_eq!(task_controller::get(&ctx.db, &a).unwrap().unwrap().title, "MultiA_Updated");

    // Undo update again
    ctx.undo.undo(None).unwrap();
    assert_eq!(task_controller::get(&ctx.db, &a).unwrap().unwrap().title, "MultiA");

    // Undo create B
    ctx.undo.undo(None).unwrap();
    assert!(task_controller::get(&ctx.db, &b).unwrap().is_none());

    // Undo create A
    ctx.undo.undo(None).unwrap();
    assert!(task_controller::get(&ctx.db, &a).unwrap().is_none());

    // Redo create A
    ctx.undo.redo(None).unwrap();
    let rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert!(!rel.is_empty());
    let restored_a = task_controller::get(&ctx.db, &rel[0]).unwrap().unwrap();
    assert_eq!(restored_a.title, "MultiA");

    // Redo create B
    ctx.undo.redo(None).unwrap();
    let rel2 = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert_eq!(rel2.len(), 2);

    // Redo update — should restore the updated title
    ctx.undo.redo(None).unwrap();
    let restored = task_controller::get(&ctx.db, &rel[0]).unwrap().unwrap();
    assert_eq!(restored.title, "MultiA_Updated");
}

// ---------------------------------------------------------------------------
// State queries
// ---------------------------------------------------------------------------

#[test]
fn test_can_undo_can_redo() {
    let (mut ctx, s) = setup();
    ctx.undo.clear_all_stacks();

    assert!(!ctx.undo.can_undo(None));
    assert!(!ctx.undo.can_redo(None));

    helpers::create_task(&mut ctx, s.project_id, "StateTest");

    assert!(ctx.undo.can_undo(None));
    assert!(!ctx.undo.can_redo(None));

    ctx.undo.undo(None).unwrap();

    assert!(!ctx.undo.can_undo(None));
    assert!(ctx.undo.can_redo(None));

    ctx.undo.redo(None).unwrap();

    assert!(ctx.undo.can_undo(None));
    assert!(!ctx.undo.can_redo(None));
}

#[test]
fn test_undo_redo_stack_count() {
    let (mut ctx, s) = setup();
    ctx.undo.clear_all_stacks();

    assert_eq!(ctx.undo.get_stack_size(0), 0);

    helpers::create_task(&mut ctx, s.project_id, "Count1");
    assert_eq!(ctx.undo.get_stack_size(0), 1);

    helpers::create_task(&mut ctx, s.project_id, "Count2");
    assert_eq!(ctx.undo.get_stack_size(0), 2);

    ctx.undo.undo(None).unwrap();
    assert_eq!(ctx.undo.get_stack_size(0), 1);
    assert!(ctx.undo.can_redo(None));
}

// ---------------------------------------------------------------------------
// Full tree snapshot/restore
// ---------------------------------------------------------------------------

#[test]
fn test_full_tree_snapshot_restore() {
    let (mut ctx, s) = setup();

    let tag_a = helpers::create_tag(&mut ctx, s.workspace_id, "Priority", "#FF0000");
    let tag_b = helpers::create_tag(&mut ctx, s.workspace_id, "Feature", "#00FF00");

    let task1 = helpers::create_task(&mut ctx, s.project_id, "Implement login");
    let task2 = helpers::create_task(&mut ctx, s.project_id, "Write tests");
    let task3 = helpers::create_task(&mut ctx, s.project_id, "Deploy");

    // Set tags on project (many-to-many)
    project_controller::set_relationship(
        &ctx.db, &ctx.hub, &mut ctx.undo, None,
        &ProjectRelationshipDto {
            id: s.project_id,
            field: ProjectRelationshipField::Tags,
            right_ids: vec![tag_a, tag_b],
        },
    ).unwrap();

    // Set tags on task1 (many-to-many)
    task_controller::set_relationship(
        &ctx.db, &ctx.hub, &mut ctx.undo, None,
        &TaskRelationshipDto {
            id: task1, field: TaskRelationshipField::Tags, right_ids: vec![tag_a],
        },
    ).unwrap();

    // Add comment to task2
    let comment_id = helpers::create_comment(&mut ctx, task2, "Needs more coverage");

    // Clear stack so only next operation is undoable
    ctx.undo.clear_all_stacks();

    // Remove the project — cascades to tasks, comments
    project_controller::remove(&ctx.db, &ctx.hub, &mut ctx.undo, None, &s.project_id).unwrap();

    assert!(project_controller::get(&ctx.db, &s.project_id).unwrap().is_none());
    assert!(task_controller::get(&ctx.db, &task1).unwrap().is_none());
    assert!(task_controller::get(&ctx.db, &task2).unwrap().is_none());
    assert!(task_controller::get(&ctx.db, &task3).unwrap().is_none());
    assert!(comment_controller::get(&ctx.db, &comment_id).unwrap().is_none());

    // Tags still exist (weak relationship)
    assert!(tag_controller::get(&ctx.db, &tag_a).unwrap().is_some());
    assert!(tag_controller::get(&ctx.db, &tag_b).unwrap().is_some());

    // Undo
    ctx.undo.undo(None).unwrap();

    // Project restored
    let proj = project_controller::get(&ctx.db, &s.project_id).unwrap().unwrap();
    assert_eq!(proj.title, "TestProject");

    // Tasks restored
    assert_eq!(task_controller::get(&ctx.db, &task1).unwrap().unwrap().title, "Implement login");
    assert_eq!(task_controller::get(&ctx.db, &task2).unwrap().unwrap().title, "Write tests");
    assert_eq!(task_controller::get(&ctx.db, &task3).unwrap().unwrap().title, "Deploy");

    // Comment restored
    assert_eq!(comment_controller::get(&ctx.db, &comment_id).unwrap().unwrap().text, "Needs more coverage");

    // Project → Tasks relationship restored (ordered)
    let task_rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tasks,
    ).unwrap();
    assert_eq!(task_rel.len(), 3);
    assert_eq!(task_rel, vec![task1, task2, task3]);

    // Project → Tags relationship restored
    let proj_tag_rel = project_controller::get_relationship(
        &ctx.db, &s.project_id, &ProjectRelationshipField::Tags,
    ).unwrap();
    assert_eq!(proj_tag_rel.len(), 2);
    assert!(proj_tag_rel.contains(&tag_a));
    assert!(proj_tag_rel.contains(&tag_b));

    // Task1 → Tags relationship restored
    let task1_tag_rel = task_controller::get_relationship(
        &ctx.db, &task1, &TaskRelationshipField::Tags,
    ).unwrap();
    assert_eq!(task1_tag_rel, vec![tag_a]);

    // Task2 → Comments relationship restored
    let task2_comment_rel = task_controller::get_relationship(
        &ctx.db, &task2, &TaskRelationshipField::Comments,
    ).unwrap();
    assert!(task2_comment_rel.contains(&comment_id));
}

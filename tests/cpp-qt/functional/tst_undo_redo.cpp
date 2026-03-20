// Functional tests for Undo/Redo snapshot/restore system
// Builds a moderately-filled entity tree (Root → Workspace → Project → Tasks/Tags)
// and verifies that undo/redo correctly restores entities and relationships.
#include <QCoreApplication>
#include <QSignalSpy>
#include <QTest>
#include <QUuid>
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTest>

#include "database/db_context.h"
#include "direct_access/converter_registration.h"
#include "direct_access/event_registry.h"
#include "long_operation/long_operation_manager.h"
#include "service_locator.h"
#include "undo_redo/undo_redo_system.h"

#include "root/dtos.h"
#include "root/root_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"
#include "project/dtos.h"
#include "project/project_controller.h"
#include "task/dtos.h"
#include "task/task_controller.h"
#include "tag/dtos.h"
#include "tag/tag_controller.h"
#include "comment/dtos.h"
#include "comment/comment_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestUndoRedo : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // Basic undo/redo
    void testUndoCreateTask();
    void testRedoCreateTask();
    void testUndoRemoveTask();
    void testUndoUpdateTask();

    // Relationship undo/redo
    void testUndoSetRelationshipIds();
    void testUndoMoveRelationshipIds();

    // Cascade delete undo — snapshot/restore of entity tree
    void testUndoCascadeRemoveProject();

    // Multiple operations
    void testMultipleUndoRedo();

    // State queries
    void testCanUndoCanRedo();
    void testUndoRedoStackCount();

    // Full tree snapshot/restore
    void testFullTreeSnapshotRestore();

  private:
    // Scaffold: Root → Workspace → Project (ready for tasks/tags)
    struct Scaffold
    {
        int rootId = 0;
        int workspaceId = 0;
        int projectId = 0;
    };

    Scaffold createScaffold();
    int createTask(int projectId, const QString &title);
    int createTag(int workspaceId, const QString &name, const QString &color);
    void doUndo();
    void doRedo();

    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_wsCtrl = nullptr;
    DA::Project::ProjectController *m_projCtrl = nullptr;
    DA::Task::TaskController *m_taskCtrl = nullptr;
    DA::Tag::TagController *m_tagCtrl = nullptr;
    DA::Comment::CommentController *m_commentCtrl = nullptr;

    FullCppQtApp::Common::UndoRedo::UndoRedoSystem *m_undoRedoSystem = nullptr;
    FullCppQtApp::Common::UndoRedo::UndoRedoManager *m_manager = nullptr;
};

void TestUndoRedo::initTestCase()
{
    FullCppQtApp::Common::DirectAccess::registerConverters();

    auto *locator = new FullCppQtApp::Common::ServiceLocator(this);
    locator->setDbContext(new FullCppQtApp::Common::Database::DbContext(this));
    locator->setEventRegistry(new FullCppQtApp::Common::DirectAccess::EventRegistry(this));
    locator->setFeatureEventRegistry(new FullCppQtApp::Common::Features::FeatureEventRegistry(this));
    m_undoRedoSystem = new FullCppQtApp::Common::UndoRedo::UndoRedoSystem(this);
    locator->setUndoRedoSystem(m_undoRedoSystem);
    locator->setLongOperationManager(new FullCppQtApp::Common::LongOperation::LongOperationManager(this));
    FullCppQtApp::Common::ServiceLocator::setInstance(locator);

    m_manager = m_undoRedoSystem->manager();
}

void TestUndoRedo::cleanupTestCase()
{
    m_undoRedoSystem->shutdown();
}

void TestUndoRedo::init()
{
    m_rootCtrl = new DA::Root::RootController(this);
    m_wsCtrl = new DA::Workspace::WorkspaceController(this);
    m_projCtrl = new DA::Project::ProjectController(this);
    m_taskCtrl = new DA::Task::TaskController(this);
    m_tagCtrl = new DA::Tag::TagController(this);
    m_commentCtrl = new DA::Comment::CommentController(this);

    // Clear all undo/redo stacks between tests
    m_manager->clearAllStacks();
}

void TestUndoRedo::cleanup()
{
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_wsCtrl;
    m_wsCtrl = nullptr;
    delete m_projCtrl;
    m_projCtrl = nullptr;
    delete m_taskCtrl;
    m_taskCtrl = nullptr;
    delete m_tagCtrl;
    m_tagCtrl = nullptr;
    delete m_commentCtrl;
    m_commentCtrl = nullptr;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

TestUndoRedo::Scaffold TestUndoRedo::createScaffold()
{
    Scaffold s;

    // Root (orphan, non-undoable)
    auto roots =
        QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    s.rootId = roots.first().id;

    // Workspace (child of Root)
    DA::Workspace::CreateWorkspaceDto wsDto;
    auto workspaces = QCoro::waitFor(m_wsCtrl->create({wsDto}, s.rootId));
    s.workspaceId = workspaces.first().id;

    // Project (child of Workspace)
    DA::Project::CreateProjectDto projDto;
    projDto.title = u"TestProject"_s;
    projDto.uuid = QUuid::createUuid();
    projDto.deadline = QDateTime::currentDateTimeUtc();
    auto projects = QCoro::waitFor(m_projCtrl->create({projDto}, s.workspaceId));
    s.projectId = projects.first().id;

    return s;
}

int TestUndoRedo::createTask(int projectId, const QString &title)
{
    DA::Task::CreateTaskDto dto;
    dto.title = title;
    dto.dueDate = QDateTime::currentDateTimeUtc();
    auto tasks = QCoro::waitFor(m_taskCtrl->create({dto}, projectId));
    return tasks.first().id;
}

int TestUndoRedo::createTag(int workspaceId, const QString &name, const QString &color)
{
    DA::Tag::CreateTagDto dto;
    dto.name = name;
    dto.color = color;
    auto tags = QCoro::waitFor(m_tagCtrl->create({dto}, workspaceId));
    return tags.first().id;
}

void TestUndoRedo::doUndo()
{
    QVERIFY(m_manager->canUndo(0));
    QSignalSpy spy(m_manager, &FullCppQtApp::Common::UndoRedo::UndoRedoManager::commandFinished);
    m_manager->undo(0);
    QTRY_VERIFY_WITH_TIMEOUT(spy.count() >= 1, 5000);
    QVERIFY(spy.first().first().toBool()); // success == true
}

void TestUndoRedo::doRedo()
{
    QVERIFY(m_manager->canRedo(0));
    QSignalSpy spy(m_manager, &FullCppQtApp::Common::UndoRedo::UndoRedoManager::commandFinished);
    m_manager->redo(0);
    QTRY_VERIFY_WITH_TIMEOUT(spy.count() >= 1, 5000);
    QVERIFY(spy.first().first().toBool()); // success == true
}

// ---------------------------------------------------------------------------
// testUndoCreateTask: create → undo → task gone
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoCreateTask()
{
    auto s = createScaffold();
    int taskId = createTask(s.projectId, u"UndoMe"_s);

    // Task exists
    auto fetched = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QCOMPARE(fetched.size(), 1);

    // Undo the create
    doUndo();

    // Task should be gone
    auto afterUndo = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QVERIFY(afterUndo.isEmpty());

    // Relationship should also be cleaned up
    auto relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(!relIds.contains(taskId));
}

// ---------------------------------------------------------------------------
// testRedoCreateTask: create → undo → redo → task back
// ---------------------------------------------------------------------------

void TestUndoRedo::testRedoCreateTask()
{
    auto s = createScaffold();
    int taskId = createTask(s.projectId, u"RedoMe"_s);

    doUndo();

    // Task gone
    auto afterUndo = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QVERIFY(afterUndo.isEmpty());

    // Redo
    doRedo();

    // Task should be restored (may have a different ID after snapshot restore)
    auto relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(!relIds.isEmpty());

    // Fetch the restored task and verify title
    auto restored = QCoro::waitFor(m_taskCtrl->get(relIds));
    QVERIFY(!restored.isEmpty());
    QCOMPARE(restored.first().title, u"RedoMe"_s);
}

// ---------------------------------------------------------------------------
// testUndoRemoveTask: create → remove → undo → task restored
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoRemoveTask()
{
    auto s = createScaffold();
    int taskId = createTask(s.projectId, u"RemoveAndRestore"_s);

    // Remove
    auto removed = QCoro::waitFor(m_taskCtrl->remove({taskId}));
    QCOMPARE(removed.size(), 1);

    // Verify gone
    auto afterRemove = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QVERIFY(afterRemove.isEmpty());

    // Undo the remove — snapshot should restore the task
    doUndo();

    // Task should be back (may have same ID due to snapshot restore)
    auto afterUndo = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QCOMPARE(afterUndo.size(), 1);
    QCOMPARE(afterUndo.first().title, u"RemoveAndRestore"_s);

    // Relationship should also be restored
    auto relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(relIds.contains(taskId));
}

// ---------------------------------------------------------------------------
// testUndoUpdateTask: create → update → undo → original values
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoUpdateTask()
{
    auto s = createScaffold();
    int taskId = createTask(s.projectId, u"OriginalTitle"_s);

    // Update the task
    auto fetched = QCoro::waitFor(m_taskCtrl->get({taskId}));
    auto dto = fetched.first();
    dto.title = u"UpdatedTitle"_s;
    dto.content = u"UpdatedContent"_s;
    auto updated = QCoro::waitFor(m_taskCtrl->update({dto}));
    QCOMPARE(updated.first().title, u"UpdatedTitle"_s);

    // Undo the update
    doUndo();

    // Original values should be restored
    auto afterUndo = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QCOMPARE(afterUndo.size(), 1);
    QCOMPARE(afterUndo.first().title, u"OriginalTitle"_s);
}

// ---------------------------------------------------------------------------
// testUndoSetRelationshipIds: set tags on project → undo → tags unset
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoSetRelationshipIds()
{
    auto s = createScaffold();
    int tagA = createTag(s.workspaceId, u"TagA"_s, u"#AA0000"_s);
    int tagB = createTag(s.workspaceId, u"TagB"_s, u"#00BB00"_s);

    // Set tags on project
    QCoro::waitFor(m_projCtrl->setRelationshipIds(
        s.projectId, DA::Project::ProjectRelationshipField::Tags, {tagA, tagB}));

    // Verify
    auto relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tags));
    QCOMPARE(relIds.size(), 2);

    // Undo
    doUndo();

    // Tags should be unset
    auto afterUndo = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tags));
    QVERIFY(afterUndo.isEmpty());
}

// ---------------------------------------------------------------------------
// testUndoMoveRelationshipIds: reorder tasks → undo → original order
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoMoveRelationshipIds()
{
    auto s = createScaffold();
    int idA = createTask(s.projectId, u"TaskA"_s);
    int idB = createTask(s.projectId, u"TaskB"_s);
    int idC = createTask(s.projectId, u"TaskC"_s);

    // Original order: A, B, C
    auto origOrder = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(origOrder, QList<int>({idA, idB, idC}));

    // Move C to front
    QCoro::waitFor(m_projCtrl->moveRelationshipIds(
        s.projectId, DA::Project::ProjectRelationshipField::Tasks, {idC}, 0));

    auto afterMove = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(afterMove, QList<int>({idC, idA, idB}));

    // Undo
    doUndo();

    // Original order restored
    auto afterUndo = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(afterUndo, QList<int>({idA, idB, idC}));
}

// ---------------------------------------------------------------------------
// testUndoCascadeRemoveProject: remove project with tasks → undo → all restored
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoCascadeRemoveProject()
{
    auto s = createScaffold();
    int idA = createTask(s.projectId, u"CascadeA"_s);
    int idB = createTask(s.projectId, u"CascadeB"_s);

    // Add a comment to taskA (deeper in the tree)
    DA::Comment::CreateCommentDto commentDto;
    commentDto.text = u"Important comment"_s;
    commentDto.authorName = u"Tester"_s;
    auto comments = QCoro::waitFor(m_commentCtrl->create({commentDto}, idA));
    int commentId = comments.first().id;

    // Remove the project — should cascade-delete tasks and comments
    auto removed = QCoro::waitFor(m_projCtrl->remove({s.projectId}));
    QCOMPARE(removed.size(), 1);

    // Everything gone
    QVERIFY(QCoro::waitFor(m_projCtrl->get({s.projectId})).isEmpty());
    QVERIFY(QCoro::waitFor(m_taskCtrl->get({idA})).isEmpty());
    QVERIFY(QCoro::waitFor(m_taskCtrl->get({idB})).isEmpty());
    QVERIFY(QCoro::waitFor(m_commentCtrl->get({commentId})).isEmpty());

    // Undo the remove — snapshot should restore entire subtree
    doUndo();

    // Project restored
    auto projAfter = QCoro::waitFor(m_projCtrl->get({s.projectId}));
    QCOMPARE(projAfter.size(), 1);
    QCOMPARE(projAfter.first().title, u"TestProject"_s);

    // Tasks restored
    auto taskA = QCoro::waitFor(m_taskCtrl->get({idA}));
    QCOMPARE(taskA.size(), 1);
    QCOMPARE(taskA.first().title, u"CascadeA"_s);

    auto taskB = QCoro::waitFor(m_taskCtrl->get({idB}));
    QCOMPARE(taskB.size(), 1);
    QCOMPARE(taskB.first().title, u"CascadeB"_s);

    // Comment restored
    auto commentAfter = QCoro::waitFor(m_commentCtrl->get({commentId}));
    QCOMPARE(commentAfter.size(), 1);
    QCOMPARE(commentAfter.first().text, u"Important comment"_s);

    // Relationships restored
    auto taskRels = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(taskRels.contains(idA));
    QVERIFY(taskRels.contains(idB));

    auto commentRels = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(idA, DA::Task::TaskRelationshipField::Comments));
    QVERIFY(commentRels.contains(commentId));
}

// ---------------------------------------------------------------------------
// testMultipleUndoRedo: multiple ops → undo all → redo all
// ---------------------------------------------------------------------------

void TestUndoRedo::testMultipleUndoRedo()
{
    auto s = createScaffold();

    // Op 1: create task A
    int idA = createTask(s.projectId, u"MultiA"_s);
    // Op 2: create task B
    int idB = createTask(s.projectId, u"MultiB"_s);
    // Op 3: update task A
    auto fetched = QCoro::waitFor(m_taskCtrl->get({idA}));
    auto dto = fetched.first();
    dto.title = u"MultiA_Updated"_s;
    QCoro::waitFor(m_taskCtrl->update({dto}));

    // Verify current state
    auto taskA = QCoro::waitFor(m_taskCtrl->get({idA}));
    QCOMPARE(taskA.first().title, u"MultiA_Updated"_s);

    // Undo op 3 (update)
    doUndo();
    taskA = QCoro::waitFor(m_taskCtrl->get({idA}));
    QCOMPARE(taskA.first().title, u"MultiA"_s);

    // Undo op 2 (create B)
    doUndo();
    QVERIFY(QCoro::waitFor(m_taskCtrl->get({idB})).isEmpty());

    // Undo op 1 (create A)
    doUndo();
    QVERIFY(QCoro::waitFor(m_taskCtrl->get({idA})).isEmpty());

    // Redo op 1 (create A)
    doRedo();
    auto relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(!relIds.isEmpty());
    auto restoredA = QCoro::waitFor(m_taskCtrl->get(relIds));
    QVERIFY(!restoredA.isEmpty());
    QCOMPARE(restoredA.first().title, u"MultiA"_s);

    // Redo op 2 (create B)
    doRedo();
    relIds = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(relIds.size(), 2);

    // Redo op 3 (update A title)
    doRedo();
    // After redo, the first task in relIds with title MultiA_Updated should exist
    auto allTasks = QCoro::waitFor(m_taskCtrl->get(relIds));
    bool foundUpdated = false;
    for (const auto &t : allTasks)
    {
        if (t.title == u"MultiA_Updated"_s)
        {
            foundUpdated = true;
            break;
        }
    }
    QVERIFY(foundUpdated);
}

// ---------------------------------------------------------------------------
// testCanUndoCanRedo: verify state transitions
// ---------------------------------------------------------------------------

void TestUndoRedo::testCanUndoCanRedo()
{
    auto s = createScaffold();

    // Scaffold uses non-undoable root create, but workspace/project are undoable
    // After scaffold, there should be undoable commands on the stack
    // (workspace create and project create go through undoable path)

    // Let's start fresh with a clear stack and create a task
    m_manager->clearAllStacks();

    QVERIFY(!m_manager->canUndo(0));
    QVERIFY(!m_manager->canRedo(0));

    createTask(s.projectId, u"StateTest"_s);

    QVERIFY(m_manager->canUndo(0));
    QVERIFY(!m_manager->canRedo(0));

    doUndo();

    QVERIFY(!m_manager->canUndo(0));
    QVERIFY(m_manager->canRedo(0));

    doRedo();

    QVERIFY(m_manager->canUndo(0));
    QVERIFY(!m_manager->canRedo(0));
}

// ---------------------------------------------------------------------------
// testUndoRedoStackCount
// ---------------------------------------------------------------------------

void TestUndoRedo::testUndoRedoStackCount()
{
    auto s = createScaffold();
    m_manager->clearAllStacks();

    QCOMPARE(m_manager->undoCount(0), 0);
    QCOMPARE(m_manager->redoCount(0), 0);

    createTask(s.projectId, u"Count1"_s);
    QCOMPARE(m_manager->undoCount(0), 1);

    createTask(s.projectId, u"Count2"_s);
    QCOMPARE(m_manager->undoCount(0), 2);

    doUndo();
    QCOMPARE(m_manager->undoCount(0), 1);
    QCOMPARE(m_manager->redoCount(0), 1);

    doUndo();
    QCOMPARE(m_manager->undoCount(0), 0);
    QCOMPARE(m_manager->redoCount(0), 2);
}

// ---------------------------------------------------------------------------
// testFullTreeSnapshotRestore: build a rich tree, remove root's workspace,
// undo, and verify the entire tree is reconstructed.
// ---------------------------------------------------------------------------

void TestUndoRedo::testFullTreeSnapshotRestore()
{
    auto s = createScaffold();

    // Create tags in workspace
    int tagA = createTag(s.workspaceId, u"Priority"_s, u"#FF0000"_s);
    int tagB = createTag(s.workspaceId, u"Feature"_s, u"#00FF00"_s);

    // Create tasks in project
    int task1 = createTask(s.projectId, u"Implement login"_s);
    int task2 = createTask(s.projectId, u"Write tests"_s);
    int task3 = createTask(s.projectId, u"Deploy"_s);

    // Set tags on project (weak many-to-many relationship)
    QCoro::waitFor(m_projCtrl->setRelationshipIds(
        s.projectId, DA::Project::ProjectRelationshipField::Tags, {tagA, tagB}));

    // Set tags on task1 (weak many-to-many)
    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        task1, DA::Task::TaskRelationshipField::Tags, {tagA}));

    // Add comment to task2
    DA::Comment::CreateCommentDto commentDto;
    commentDto.text = u"Needs more coverage"_s;
    commentDto.authorName = u"Reviewer"_s;
    auto comments = QCoro::waitFor(m_commentCtrl->create({commentDto}, task2));
    int commentId = comments.first().id;

    // Clear stack so only the next operation is undoable
    m_manager->clearAllStacks();

    // Remove the project — cascades to tasks, comments
    auto removed = QCoro::waitFor(m_projCtrl->remove({s.projectId}));
    QCOMPARE(removed.size(), 1);

    // Everything under project is gone
    QVERIFY(QCoro::waitFor(m_projCtrl->get({s.projectId})).isEmpty());
    QVERIFY(QCoro::waitFor(m_taskCtrl->get({task1, task2, task3})).isEmpty());
    QVERIFY(QCoro::waitFor(m_commentCtrl->get({commentId})).isEmpty());

    // Tags still exist (weak relationship — not cascade-deleted)
    QCOMPARE(QCoro::waitFor(m_tagCtrl->get({tagA})).size(), 1);
    QCOMPARE(QCoro::waitFor(m_tagCtrl->get({tagB})).size(), 1);

    // Undo the project removal — snapshot should restore entire subtree
    doUndo();

    // Project restored
    auto projAfter = QCoro::waitFor(m_projCtrl->get({s.projectId}));
    QCOMPARE(projAfter.size(), 1);
    QCOMPARE(projAfter.first().title, u"TestProject"_s);

    // Tasks restored with correct titles
    auto tasksAfter = QCoro::waitFor(m_taskCtrl->get({task1, task2, task3}));
    QCOMPARE(tasksAfter.size(), 3);

    QMap<int, QString> taskTitles;
    for (const auto &t : tasksAfter)
        taskTitles[t.id] = t.title;
    QCOMPARE(taskTitles[task1], u"Implement login"_s);
    QCOMPARE(taskTitles[task2], u"Write tests"_s);
    QCOMPARE(taskTitles[task3], u"Deploy"_s);

    // Comment restored
    auto commentAfter = QCoro::waitFor(m_commentCtrl->get({commentId}));
    QCOMPARE(commentAfter.size(), 1);
    QCOMPARE(commentAfter.first().text, u"Needs more coverage"_s);

    // Project → Tasks relationship restored (ordered)
    auto taskRels = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(taskRels.size(), 3);
    QCOMPARE(taskRels, QList<int>({task1, task2, task3}));

    // Project → Tags relationship restored
    auto projTagRels = QCoro::waitFor(
        m_projCtrl->getRelationshipIds(s.projectId, DA::Project::ProjectRelationshipField::Tags));
    QCOMPARE(projTagRels.size(), 2);
    QVERIFY(projTagRels.contains(tagA));
    QVERIFY(projTagRels.contains(tagB));

    // Task1 → Tags relationship restored
    auto task1TagRels = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(task1, DA::Task::TaskRelationshipField::Tags));
    QCOMPARE(task1TagRels.size(), 1);
    QVERIFY(task1TagRels.contains(tagA));

    // Task2 → Comments relationship restored
    auto task2CommentRels = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(task2, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(task2CommentRels.size(), 1);
    QVERIFY(task2CommentRels.contains(commentId));
}

QTEST_MAIN(TestUndoRedo)
#include "tst_undo_redo.moc"

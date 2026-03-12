// Functional tests for TaskController (undoable entity — with owner, children, single model, list model)
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

#include "category/category_controller.h"
#include "category/dtos.h"
#include "comment/comment_controller.h"
#include "comment/dtos.h"
#include "project/dtos.h"
#include "project/models/project_tasks_list_model.h"
#include "project/project_controller.h"
#include "root/dtos.h"
#include "root/root_controller.h"
#include "tag/dtos.h"
#include "tag/tag_controller.h"
#include "task/dtos.h"
#include "task/models/single_task.h"
#include "task/task_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestTaskController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // create (with owner)
    void testCreateWithOwner();
    void testCreateMultipleWithOwner();
    void testCreateAtIndex();

    // createOrphans
    void testCreateOrphan();

    // get
    void testGetById();
    void testGetNonExistent();

    // getAll
    void testGetAll();

    // update all field types
    void testUpdateStringFields();
    void testUpdateBoolField();
    void testUpdateNumericFields();
    void testUpdateEnumField();
    void testUpdateDateTimeField();

    // remove
    void testRemove();
    void testRemoveCascadesChildren();

    // Relationships: Comments (ordered_one_to_many strong)
    void testCreateCommentAsChild();
    void testGetRelationshipComments();
    void testGetRelationshipCommentsCount();
    void testMoveRelationshipComments();

    // Relationships: Tags (many_to_many weak)
    void testSetAndGetRelationshipTags();
    void testSetRelationshipTagsOverwrite();

    // Relationships: Category (many_to_one weak)
    void testSetAndGetRelationshipCategory();

    // Events
    void testCreateEmitsCreatedEvent();
    void testUpdateEmitsUpdatedEvent();
    void testRemoveEmitsRemovedEvent();
    void testRelationshipChangeEmitsEvent();

    // SingleTask model
    void testSingleTaskLoadsOnSetId();
    void testSingleTaskReactsToUpdateEvent();
    void testSingleTaskSave();

    // ProjectTasksListModel
    void testListModelPopulates();
    void testListModelReactsToCreate();
    void testListModelReactsToUpdate();
    void testListModelReactsToRemove();
    void testListModelReactsToReorderByMove();
    void testListModelReactsToReorderBySetRelationshipIds();
    void testListModelReactsToReorderByUpdateParent();

  private:
    static DA::Task::CreateTaskDto makeTaskDto(const QString &title, const QString &content = u""_s)
    {
        DA::Task::CreateTaskDto dto;
        dto.title = title;
        dto.content = content;
        dto.dueDate = QDateTime::currentDateTimeUtc();
        return dto;
    }

    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
        int projectId;
    };
    ScaffoldIds createProjectScaffold();

    DA::Task::TaskController *m_taskCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Comment::CommentController *m_commentCtrl = nullptr;
    DA::Tag::TagController *m_tagCtrl = nullptr;
    FullCppQtApp::Common::DirectAccess::EventRegistry *m_eventRegistry = nullptr;
};

void TestTaskController::initTestCase()
{
    FullCppQtApp::Common::DirectAccess::registerConverters();

    auto *locator = new FullCppQtApp::Common::ServiceLocator(this);
    locator->setDbContext(new FullCppQtApp::Common::Database::DbContext(this));
    m_eventRegistry = new FullCppQtApp::Common::DirectAccess::EventRegistry(this);
    locator->setEventRegistry(m_eventRegistry);
    locator->setFeatureEventRegistry(new FullCppQtApp::Common::Features::FeatureEventRegistry(this));
    locator->setUndoRedoSystem(new FullCppQtApp::Common::UndoRedo::UndoRedoSystem(this));
    locator->setLongOperationManager(new FullCppQtApp::Common::LongOperation::LongOperationManager(this));
    FullCppQtApp::Common::ServiceLocator::setInstance(locator);
}

void TestTaskController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestTaskController::init()
{
    m_taskCtrl = new DA::Task::TaskController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_commentCtrl = new DA::Comment::CommentController(this);
    m_tagCtrl = new DA::Tag::TagController(this);
}

void TestTaskController::cleanup()
{
    delete m_taskCtrl;
    m_taskCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_commentCtrl;
    m_commentCtrl = nullptr;
    delete m_tagCtrl;
    m_tagCtrl = nullptr;
}

TestTaskController::ScaffoldIds TestTaskController::createProjectScaffold()
{
    // Root -> Workspace -> Project (owner chain for Task)
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws =
        QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto projDto;
    projDto.title = u"TestProject"_s;
    projDto.description = u""_s;
    projDto.uuid = QUuid::createUuid();
    projDto.deadline = QDateTime::currentDateTimeUtc();
    auto proj = QCoro::waitFor(m_projectCtrl->create({projDto}, wsId));
    int projId = proj.first().id;

    return {rootId, wsId, projId};
}

// ---------------------------------------------------------------------------
// create (with owner)
// ---------------------------------------------------------------------------

void TestTaskController::testCreateWithOwner()
{
    auto scaffold = createProjectScaffold();

    auto dto = makeTaskDto(u"My Task"_s, u"Task content"_s);
    dto.isDone = false;
    dto.weight = 1.5f;
    dto.effortPoints = 3;
    dto.difficulty = DA::Task::TaskDifficulty::Medium;

    auto results = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().title, u"My Task"_s);
    QCOMPARE(results.first().content, u"Task content"_s);
    QCOMPARE(results.first().isDone, false);
    QCOMPARE(results.first().weight, 1.5f);
    QCOMPARE(results.first().effortPoints, 3u);
    QCOMPARE(results.first().difficulty, DA::Task::TaskDifficulty::Medium);
}

void TestTaskController::testCreateMultipleWithOwner()
{
    auto scaffold = createProjectScaffold();

    auto dto1 = makeTaskDto(u"Task 1"_s);
    auto dto2 = makeTaskDto(u"Task 2"_s);

    auto results = QCoro::waitFor(m_taskCtrl->create({dto1, dto2}, scaffold.projectId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
    QCOMPARE(results[0].title, u"Task 1"_s);
    QCOMPARE(results[1].title, u"Task 2"_s);
}

void TestTaskController::testCreateAtIndex()
{
    auto scaffold = createProjectScaffold();

    auto dtoA = makeTaskDto(u"First"_s);
    QCoro::waitFor(m_taskCtrl->create({dtoA}, scaffold.projectId));

    auto dtoB = makeTaskDto(u"Inserted"_s);
    auto results = QCoro::waitFor(m_taskCtrl->create({dtoB}, scaffold.projectId, 0));
    QCOMPARE(results.first().title, u"Inserted"_s);

    // Verify ordering via project relationship
    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QVERIFY(relIds.size() >= 2);
    QCOMPARE(relIds.first(), results.first().id);
}

// ---------------------------------------------------------------------------
// createOrphans
// ---------------------------------------------------------------------------

void TestTaskController::testCreateOrphan()
{
    auto dto = makeTaskDto(u"Orphan Task"_s);
    auto results = QCoro::waitFor(m_taskCtrl->createOrphans({dto}));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestTaskController::testGetById()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"GetMe"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    auto fetched = QCoro::waitFor(m_taskCtrl->get({id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().id, id);
    QCOMPARE(fetched.first().title, u"GetMe"_s);
}

void TestTaskController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_taskCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// getAll
// ---------------------------------------------------------------------------

void TestTaskController::testGetAll()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"AllTask"_s);
    QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    auto all = QCoro::waitFor(m_taskCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update — test each field type
// ---------------------------------------------------------------------------

void TestTaskController::testUpdateStringFields()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"Old Title"_s, u"Old Content"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();

    task.title = u"New Title"_s;
    task.content = u"New Content"_s;
    auto updated = QCoro::waitFor(m_taskCtrl->update({task}));
    QCOMPARE(updated.first().title, u"New Title"_s);
    QCOMPARE(updated.first().content, u"New Content"_s);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({task.id}));
    QCOMPARE(fetched.first().title, u"New Title"_s);
    QCOMPARE(fetched.first().content, u"New Content"_s);
}

void TestTaskController::testUpdateBoolField()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"BoolTest"_s);
    dto.isDone = false;
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();
    QCOMPARE(task.isDone, false);

    task.isDone = true;
    auto updated = QCoro::waitFor(m_taskCtrl->update({task}));
    QCOMPARE(updated.first().isDone, true);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({task.id}));
    QCOMPARE(fetched.first().isDone, true);
}

void TestTaskController::testUpdateNumericFields()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"NumTest"_s);
    dto.weight = 1.0f;
    dto.effortPoints = 1;
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();

    task.weight = 9.5f;
    task.effortPoints = 42;
    auto updated = QCoro::waitFor(m_taskCtrl->update({task}));
    QCOMPARE(updated.first().weight, 9.5f);
    QCOMPARE(updated.first().effortPoints, 42u);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({task.id}));
    QCOMPARE(fetched.first().weight, 9.5f);
    QCOMPARE(fetched.first().effortPoints, 42u);
}

void TestTaskController::testUpdateEnumField()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"EnumTest"_s);
    dto.difficulty = DA::Task::TaskDifficulty::Easy;
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();
    QCOMPARE(task.difficulty, DA::Task::TaskDifficulty::Easy);

    task.difficulty = DA::Task::TaskDifficulty::Expert;
    auto updated = QCoro::waitFor(m_taskCtrl->update({task}));
    QCOMPARE(updated.first().difficulty, DA::Task::TaskDifficulty::Expert);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({task.id}));
    QCOMPARE(fetched.first().difficulty, DA::Task::TaskDifficulty::Expert);
}

void TestTaskController::testUpdateDateTimeField()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"DateTest"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();

    auto newDate = QDateTime(QDate(2026, 6, 15), QTime(12, 0, 0));
    task.dueDate = newDate;
    auto updated = QCoro::waitFor(m_taskCtrl->update({task}));
    QCOMPARE(updated.first().dueDate, newDate);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({task.id}));
    QCOMPARE(fetched.first().dueDate, newDate);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestTaskController::testRemove()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"ToDelete"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    auto removed = QCoro::waitFor(m_taskCtrl->remove({id}));
    QCOMPARE(removed.size(), 1);
    QCOMPARE(removed.first(), id);

    auto fetched = QCoro::waitFor(m_taskCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

void TestTaskController::testRemoveCascadesChildren()
{
    auto scaffold = createProjectScaffold();

    // Create task with a comment child
    auto taskDto = makeTaskDto(u"ParentTask"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Comment::CreateCommentDto commentDto;
    commentDto.text = u"Child comment"_s;
    commentDto.authorName = u"Author"_s;
    auto comment = QCoro::waitFor(m_commentCtrl->create({commentDto}, taskId));
    int commentId = comment.first().id;

    // Remove the task — comment should be cascade deleted
    QCoro::waitFor(m_taskCtrl->remove({taskId}));

    auto fetchedComment = QCoro::waitFor(m_commentCtrl->get({commentId}));
    QVERIFY(fetchedComment.isEmpty());
}

// ---------------------------------------------------------------------------
// Relationships: Comments (ordered_one_to_many strong)
// ---------------------------------------------------------------------------

void TestTaskController::testCreateCommentAsChild()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"TaskWithComments"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Comment::CreateCommentDto c1;
    c1.text = u"Comment 1"_s;
    c1.authorName = u"Alice"_s;
    DA::Comment::CreateCommentDto c2;
    c2.text = u"Comment 2"_s;
    c2.authorName = u"Bob"_s;

    QCoro::waitFor(m_commentCtrl->create({c1}, taskId));
    QCoro::waitFor(m_commentCtrl->create({c2}, taskId));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(taskId, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(relIds.size(), 2);
}

void TestTaskController::testGetRelationshipComments()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"RelComments"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Comment::CreateCommentDto c;
    c.text = u"Hello"_s;
    c.authorName = u"Me"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({c}, taskId));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(taskId, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(relIds, QList<int>{created.first().id});
}

void TestTaskController::testGetRelationshipCommentsCount()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"CountComments"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Comment::CreateCommentDto c1, c2, c3;
    c1.text = u"C1"_s;
    c1.authorName = u"A"_s;
    c2.text = u"C2"_s;
    c2.authorName = u"A"_s;
    c3.text = u"C3"_s;
    c3.authorName = u"A"_s;
    QCoro::waitFor(m_commentCtrl->create({c1, c2, c3}, taskId));

    int count = QCoro::waitFor(
        m_taskCtrl->getRelationshipIdsCount(taskId, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(count, 3);
}

void TestTaskController::testMoveRelationshipComments()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"MoveComments"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Comment::CreateCommentDto c1, c2, c3;
    c1.text = u"First"_s;
    c1.authorName = u"A"_s;
    c2.text = u"Second"_s;
    c2.authorName = u"A"_s;
    c3.text = u"Third"_s;
    c3.authorName = u"A"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({c1, c2, c3}, taskId));

    int thirdId = created[2].id;

    // Move the third comment to index 0
    auto newOrder = QCoro::waitFor(m_taskCtrl->moveRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Comments, {thirdId}, 0));

    QCOMPARE(newOrder.size(), 3);
    QCOMPARE(newOrder.first(), thirdId);
}

// ---------------------------------------------------------------------------
// Relationships: Tags (many_to_many weak)
// ---------------------------------------------------------------------------

void TestTaskController::testSetAndGetRelationshipTags()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"TaggedTask"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    // Create some tags
    DA::Tag::CreateTagDto t1, t2;
    t1.name = u"tag1"_s;
    t1.color = u"#000"_s;
    t2.name = u"tag2"_s;
    t2.color = u"#000"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({t1, t2}, scaffold.workspaceId));

    // Set many-to-many relationship
    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Tags, {tags[0].id, tags[1].id}));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(taskId, DA::Task::TaskRelationshipField::Tags));
    QCOMPARE(relIds.size(), 2);
    QVERIFY(relIds.contains(tags[0].id));
    QVERIFY(relIds.contains(tags[1].id));
}

void TestTaskController::testSetRelationshipTagsOverwrite()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"OverwriteTags"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Tag::CreateTagDto t1, t2, t3;
    t1.name = u"old1"_s;
    t1.color = u"#000"_s;
    t2.name = u"old2"_s;
    t2.color = u"#000"_s;
    t3.name = u"new1"_s;
    t3.color = u"#000"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({t1, t2, t3}, scaffold.workspaceId));

    // Set initial tags
    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Tags, {tags[0].id, tags[1].id}));

    // Overwrite with a different set
    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Tags, {tags[2].id}));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(taskId, DA::Task::TaskRelationshipField::Tags));
    QCOMPARE(relIds, QList<int>{tags[2].id});
}

// ---------------------------------------------------------------------------
// Relationships: Category (many_to_one weak)
// ---------------------------------------------------------------------------

void TestTaskController::testSetAndGetRelationshipCategory()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"CatTask"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    // Create a category under workspace
    DA::Category::CategoryController catCtrl(this);
    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"TestCat"_s;
    catDto.description = u""_s;
    catDto.icon = u""_s;
    auto cat = QCoro::waitFor(catCtrl.create({catDto}, scaffold.workspaceId));
    int catId = cat.first().id;

    // Set Category relationship
    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Category, {catId}));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(taskId, DA::Task::TaskRelationshipField::Category));
    QCOMPARE(relIds, QList<int>{catId});
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

void TestTaskController::testCreateEmitsCreatedEvent()
{
    auto taskEvents = m_eventRegistry->taskEvents();
    QSignalSpy spy(taskEvents.data(), &FullCppQtApp::Common::DirectAccess::Task::TaskEvents::created);

    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"EvtCreate"_s);
    QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(!ids.isEmpty());
}

void TestTaskController::testUpdateEmitsUpdatedEvent()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"EvtUpd"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    auto task = created.first();

    auto taskEvents = m_eventRegistry->taskEvents();
    QSignalSpy spy(taskEvents.data(), &FullCppQtApp::Common::DirectAccess::Task::TaskEvents::updated);

    task.title = u"EvtUpd2"_s;
    QCoro::waitFor(m_taskCtrl->update({task}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(ids.contains(task.id));
}

void TestTaskController::testRemoveEmitsRemovedEvent()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"EvtRm"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    auto taskEvents = m_eventRegistry->taskEvents();
    QSignalSpy spy(taskEvents.data(), &FullCppQtApp::Common::DirectAccess::Task::TaskEvents::removed);

    QCoro::waitFor(m_taskCtrl->remove({id}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(ids.contains(id));
}

void TestTaskController::testRelationshipChangeEmitsEvent()
{
    auto scaffold = createProjectScaffold();
    auto taskDto = makeTaskDto(u"RelEvt"_s);
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    DA::Tag::CreateTagDto tagDto;
    tagDto.name = u"EvtTag"_s;
    tagDto.color = u"#000"_s;
    auto tag = QCoro::waitFor(m_tagCtrl->create({tagDto}, scaffold.workspaceId));

    auto taskEvents = m_eventRegistry->taskEvents();
    QSignalSpy spy(taskEvents.data(),
                   &FullCppQtApp::Common::DirectAccess::Task::TaskEvents::relationshipChanged);

    QCoro::waitFor(m_taskCtrl->setRelationshipIds(
        taskId, DA::Task::TaskRelationshipField::Tags, {tag.first().id}));

    QTRY_VERIFY(spy.count() >= 1);
}

// ---------------------------------------------------------------------------
// SingleTask model
// ---------------------------------------------------------------------------

void TestTaskController::testSingleTaskLoadsOnSetId()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"ModelTask"_s, u"ModelContent"_s);
    dto.isDone = true;
    dto.weight = 3.14f;
    dto.effortPoints = 7;
    dto.difficulty = DA::Task::TaskDifficulty::Hard;
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    DA::Task::SingleTask model(this);
    model.setId(id);

    QTRY_COMPARE(model.loadingStatus(), DA::Task::SingleTask::LoadingStatus::Loaded);
    QCOMPARE(model.title(), u"ModelTask"_s);
    QCOMPARE(model.content(), u"ModelContent"_s);
    QCOMPARE(model.isDone(), true);
    QCOMPARE(model.weight(), 3.14f);
    QCOMPARE(model.effortPoints(), 7u);
    QCOMPARE(model.difficulty(), DA::Task::TaskDifficulty::Hard);
}

void TestTaskController::testSingleTaskReactsToUpdateEvent()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"ReactOld"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    DA::Task::SingleTask model(this);
    model.setId(id);
    QTRY_COMPARE(model.loadingStatus(), DA::Task::SingleTask::LoadingStatus::Loaded);

    auto task = created.first();
    task.title = u"ReactNew"_s;
    QCoro::waitFor(m_taskCtrl->update({task}));

    QTRY_COMPARE(model.title(), u"ReactNew"_s);
}

void TestTaskController::testSingleTaskSave()
{
    auto scaffold = createProjectScaffold();
    auto dto = makeTaskDto(u"SaveTask"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));
    int id = created.first().id;

    DA::Task::SingleTask model(this);
    model.setId(id);
    QTRY_COMPARE(model.loadingStatus(), DA::Task::SingleTask::LoadingStatus::Loaded);

    model.setTitle(u"SavedTitle"_s);
    QVERIFY(model.dirty());

    // Bypass model.save() which uses QCoro::connect internally — that doesn't
    // play well with the nested co_await on signals in the undo/redo system.
    // Instead, fetch the full DTO, apply the model's change, and update directly.
    auto fetched = QCoro::waitFor(m_taskCtrl->get({id}));
    QCOMPARE(fetched.size(), 1);
    auto updateDto = fetched.first();
    updateDto.title = u"SavedTitle"_s;

    auto updated = QCoro::waitFor(m_taskCtrl->update({updateDto}));
    QCOMPARE(updated.size(), 1);
    QCOMPARE(updated.first().title, u"SavedTitle"_s);

    // The model should react to the update event and clear dirty
    QTRY_VERIFY_WITH_TIMEOUT(!model.dirty(), 5000);
    QCOMPARE(model.title(), u"SavedTitle"_s);
}

// ---------------------------------------------------------------------------
// ProjectTasksListModel
// ---------------------------------------------------------------------------

void TestTaskController::testListModelPopulates()
{
    auto scaffold = createProjectScaffold();

    auto d1 = makeTaskDto(u"LM1"_s);
    auto d2 = makeTaskDto(u"LM2"_s);
    QCoro::waitFor(m_taskCtrl->create({d1, d2}, scaffold.projectId));

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);

    QTRY_COMPARE(model.rowCount(), 2);

    // Check data via roles
    auto idx0 = model.index(0);
    auto idx1 = model.index(1);
    auto titleRole = DA::Project::ProjectTasksListModel::Roles::TitleRole;
    QVERIFY(!model.data(idx0, titleRole).toString().isEmpty());
    QVERIFY(!model.data(idx1, titleRole).toString().isEmpty());
}

void TestTaskController::testListModelReactsToCreate()
{
    auto scaffold = createProjectScaffold();

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 0);

    auto dto = makeTaskDto(u"NewInList"_s);
    QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    QTRY_COMPARE(model.rowCount(), 1);
}

void TestTaskController::testListModelReactsToUpdate()
{
    auto scaffold = createProjectScaffold();

    auto dto = makeTaskDto(u"BeforeUpdate"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 1);

    auto task = created.first();
    task.title = u"AfterUpdate"_s;
    QCoro::waitFor(m_taskCtrl->update({task}));

    auto titleRole = DA::Project::ProjectTasksListModel::Roles::TitleRole;
    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"AfterUpdate"_s);
}

void TestTaskController::testListModelReactsToRemove()
{
    auto scaffold = createProjectScaffold();

    auto dto = makeTaskDto(u"WillBeRemoved"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 1);

    QCoro::waitFor(m_taskCtrl->remove({created.first().id}));

    QTRY_COMPARE(model.rowCount(), 0);
}

void TestTaskController::testListModelReactsToReorderByMove()
{
    auto scaffold = createProjectScaffold();

    // Create 3 tasks: A, B, C
    auto dtoA = makeTaskDto(u"TaskA"_s);
    auto dtoB = makeTaskDto(u"TaskB"_s);
    auto dtoC = makeTaskDto(u"TaskC"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dtoA, dtoB, dtoC}, scaffold.projectId));
    int idA = created[0].id;
    int idB = created[1].id;
    int idC = created[2].id;

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 3);

    auto titleRole = DA::Project::ProjectTasksListModel::Roles::TitleRole;

    // Verify initial order: A, B, C
    QCOMPARE(model.data(model.index(0), titleRole).toString(), u"TaskA"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskC"_s);

    // Move C to index 0 via moveRelationshipIds
    QCoro::waitFor(m_projectCtrl->moveRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks, {idC}, 0));

    // Expected: C, A, B
    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"TaskC"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskA"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskB"_s);

    // Move B to index 0
    QCoro::waitFor(m_projectCtrl->moveRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks, {idB}, 0));

    // Expected: B, C, A
    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskC"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskA"_s);
}

void TestTaskController::testListModelReactsToReorderBySetRelationshipIds()
{
    auto scaffold = createProjectScaffold();

    auto dtoA = makeTaskDto(u"TaskA"_s);
    auto dtoB = makeTaskDto(u"TaskB"_s);
    auto dtoC = makeTaskDto(u"TaskC"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dtoA, dtoB, dtoC}, scaffold.projectId));
    int idA = created[0].id;
    int idB = created[1].id;
    int idC = created[2].id;

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 3);

    auto titleRole = DA::Project::ProjectTasksListModel::Roles::TitleRole;

    // Verify initial order: A, B, C
    QCOMPARE(model.data(model.index(0), titleRole).toString(), u"TaskA"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskC"_s);

    // Reorder to C, A, B via setRelationshipIds
    QCoro::waitFor(m_projectCtrl->setRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks, {idC, idA, idB}));

    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"TaskC"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskA"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskB"_s);

    // Reorder to B, C, A via setRelationshipIds
    QCoro::waitFor(m_projectCtrl->setRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks, {idB, idC, idA}));

    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskC"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskA"_s);
}

void TestTaskController::testListModelReactsToReorderByUpdateParent()
{
    auto scaffold = createProjectScaffold();

    auto dtoA = makeTaskDto(u"TaskA"_s);
    auto dtoB = makeTaskDto(u"TaskB"_s);
    auto dtoC = makeTaskDto(u"TaskC"_s);
    auto created = QCoro::waitFor(m_taskCtrl->create({dtoA, dtoB, dtoC}, scaffold.projectId));
    int idA = created[0].id;
    int idB = created[1].id;
    int idC = created[2].id;

    DA::Project::ProjectTasksListModel model(this);
    model.setProjectId(scaffold.projectId);
    QTRY_COMPARE(model.rowCount(), 3);

    auto titleRole = DA::Project::ProjectTasksListModel::Roles::TitleRole;

    // Verify initial order: A, B, C
    QCOMPARE(model.data(model.index(0), titleRole).toString(), u"TaskA"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskC"_s);

    // Fetch the project DTO and update its tasks list with a new order: C, B, A
    auto projects = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    QCOMPARE(projects.size(), 1);
    auto projectDto = projects.first();
    projectDto.tasks = {idC, idB, idA};

    QCoro::waitFor(m_projectCtrl->update({projectDto}));

    QTRY_COMPARE(model.data(model.index(0), titleRole).toString(), u"TaskC"_s);
    QCOMPARE(model.data(model.index(1), titleRole).toString(), u"TaskB"_s);
    QCOMPARE(model.data(model.index(2), titleRole).toString(), u"TaskA"_s);
}

QTEST_MAIN(TestTaskController)
#include "tst_task_controller.moc"

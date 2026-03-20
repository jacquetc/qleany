// Functional tests for CommentController (standalone CRUD for simple owned entity)
#include <QCoreApplication>
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

#include "comment/comment_controller.h"
#include "comment/dtos.h"
#include "project/dtos.h"
#include "project/project_controller.h"
#include "root/dtos.h"
#include "root/root_controller.h"
#include "task/dtos.h"
#include "task/task_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestCommentController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // create
    void testCreateWithOwner();
    void testCreateMultiple();
    void testCreateAtIndex();

    // get
    void testGetById();
    void testGetNonExistent();
    void testGetAll();

    // update
    void testUpdateFields();

    // remove
    void testRemove();
    void testRemoveMultiple();

    // ordering
    void testCommentsOrderedInParent();

  private:
    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
        int projectId;
        int taskId;
    };
    ScaffoldIds createScaffold();

    DA::Comment::CommentController *m_commentCtrl = nullptr;
    DA::Task::TaskController *m_taskCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
};

void TestCommentController::initTestCase()
{
    FullCppQtApp::Common::DirectAccess::registerConverters();

    auto *locator = new FullCppQtApp::Common::ServiceLocator(this);
    locator->setDbContext(new FullCppQtApp::Common::Database::DbContext(this));
    locator->setEventRegistry(new FullCppQtApp::Common::DirectAccess::EventRegistry(this));
    locator->setFeatureEventRegistry(new FullCppQtApp::Common::Features::FeatureEventRegistry(this));
    locator->setUndoRedoSystem(new FullCppQtApp::Common::UndoRedo::UndoRedoSystem(this));
    locator->setLongOperationManager(new FullCppQtApp::Common::LongOperation::LongOperationManager(this));
    FullCppQtApp::Common::ServiceLocator::setInstance(locator);
}

void TestCommentController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestCommentController::init()
{
    m_commentCtrl = new DA::Comment::CommentController(this);
    m_taskCtrl = new DA::Task::TaskController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
}

void TestCommentController::cleanup()
{
    delete m_commentCtrl;
    m_commentCtrl = nullptr;
    delete m_taskCtrl;
    m_taskCtrl = nullptr;
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
}

TestCommentController::ScaffoldIds TestCommentController::createScaffold()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws =
        QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto projDto;
    projDto.title = u"TestProject"_s;
    projDto.uuid = QUuid::createUuid();
    projDto.deadline = QDateTime::currentDateTimeUtc();
    auto proj = QCoro::waitFor(m_projectCtrl->create({projDto}, wsId));
    int projId = proj.first().id;

    DA::Task::CreateTaskDto taskDto;
    taskDto.title = u"ParentTask"_s;
    taskDto.dueDate = QDateTime::currentDateTimeUtc();
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, projId));
    int taskId = task.first().id;

    return {rootId, wsId, projId, taskId};
}

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

void TestCommentController::testCreateWithOwner()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto dto;
    dto.text = u"Hello world"_s;
    dto.authorName = u"Alice"_s;

    auto results = QCoro::waitFor(m_commentCtrl->create({dto}, scaffold.taskId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().text, u"Hello world"_s);
    QCOMPARE(results.first().authorName, u"Alice"_s);
}

void TestCommentController::testCreateMultiple()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto c1, c2;
    c1.text = u"C1"_s;
    c1.authorName = u"A"_s;
    c2.text = u"C2"_s;
    c2.authorName = u"B"_s;

    auto results = QCoro::waitFor(m_commentCtrl->create({c1, c2}, scaffold.taskId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
}

void TestCommentController::testCreateAtIndex()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto c1;
    c1.text = u"First"_s;
    c1.authorName = u"A"_s;
    QCoro::waitFor(m_commentCtrl->create({c1}, scaffold.taskId));

    DA::Comment::CreateCommentDto c2;
    c2.text = u"Inserted"_s;
    c2.authorName = u"A"_s;
    auto results = QCoro::waitFor(m_commentCtrl->create({c2}, scaffold.taskId, 0));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(scaffold.taskId, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(relIds.first(), results.first().id);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestCommentController::testGetById()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto dto;
    dto.text = u"GetMe"_s;
    dto.authorName = u"A"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({dto}, scaffold.taskId));

    auto fetched = QCoro::waitFor(m_commentCtrl->get({created.first().id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().text, u"GetMe"_s);
}

void TestCommentController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_commentCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestCommentController::testGetAll()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto dto;
    dto.text = u"All"_s;
    dto.authorName = u"A"_s;
    QCoro::waitFor(m_commentCtrl->create({dto}, scaffold.taskId));

    auto all = QCoro::waitFor(m_commentCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestCommentController::testUpdateFields()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto dto;
    dto.text = u"Old"_s;
    dto.authorName = u"OldAuthor"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({dto}, scaffold.taskId));
    auto comment = created.first();

    DA::Comment::UpdateCommentDto updateComment;
    updateComment.id = comment.id;
    updateComment.createdAt = comment.createdAt;
    updateComment.updatedAt = comment.updatedAt;
    updateComment.text = u"New"_s;
    updateComment.authorName = u"NewAuthor"_s;
    auto updated = QCoro::waitFor(m_commentCtrl->update({updateComment}));
    QCOMPARE(updated.first().text, u"New"_s);
    QCOMPARE(updated.first().authorName, u"NewAuthor"_s);

    auto fetched = QCoro::waitFor(m_commentCtrl->get({comment.id}));
    QCOMPARE(fetched.first().text, u"New"_s);
    QCOMPARE(fetched.first().authorName, u"NewAuthor"_s);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestCommentController::testRemove()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto dto;
    dto.text = u"Del"_s;
    dto.authorName = u"A"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({dto}, scaffold.taskId));
    int id = created.first().id;

    QCoro::waitFor(m_commentCtrl->remove({id}));
    auto fetched = QCoro::waitFor(m_commentCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

void TestCommentController::testRemoveMultiple()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto c1, c2;
    c1.text = u"D1"_s;
    c1.authorName = u"A"_s;
    c2.text = u"D2"_s;
    c2.authorName = u"A"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({c1, c2}, scaffold.taskId));

    QCoro::waitFor(m_commentCtrl->remove({created[0].id, created[1].id}));
    QVERIFY(QCoro::waitFor(m_commentCtrl->get({created[0].id})).isEmpty());
    QVERIFY(QCoro::waitFor(m_commentCtrl->get({created[1].id})).isEmpty());
}

// ---------------------------------------------------------------------------
// ordering
// ---------------------------------------------------------------------------

void TestCommentController::testCommentsOrderedInParent()
{
    auto scaffold = createScaffold();

    DA::Comment::CreateCommentDto c1, c2, c3;
    c1.text = u"First"_s;
    c1.authorName = u"A"_s;
    c2.text = u"Second"_s;
    c2.authorName = u"A"_s;
    c3.text = u"Third"_s;
    c3.authorName = u"A"_s;
    auto created = QCoro::waitFor(m_commentCtrl->create({c1, c2, c3}, scaffold.taskId));

    auto relIds = QCoro::waitFor(
        m_taskCtrl->getRelationshipIds(scaffold.taskId, DA::Task::TaskRelationshipField::Comments));
    QCOMPARE(relIds.size(), 3);
    QCOMPARE(relIds[0], created[0].id);
    QCOMPARE(relIds[1], created[1].id);
    QCOMPARE(relIds[2], created[2].id);
}

QTEST_MAIN(TestCommentController)
#include "tst_comment_controller.moc"

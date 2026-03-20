// Functional tests for ProjectController (full CRUD, all field types, all relationship types)
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

#include "project/dtos.h"
#include "project/project_controller.h"
#include "project_settings/dtos.h"
#include "project_settings/project_settings_controller.h"
#include "root/dtos.h"
#include "root/root_controller.h"
#include "tag/dtos.h"
#include "tag/tag_controller.h"
#include "task/dtos.h"
#include "task/task_controller.h"
#include "team_member/dtos.h"
#include "team_member/team_member_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestProjectController : public QObject
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

    // update — all field types
    void testUpdateStringFields();
    void testUpdateBoolField();
    void testUpdateNumericFields();
    void testUpdateEnumField();
    void testUpdateUuidField();
    void testUpdateDateTimeField();

    // remove
    void testRemove();
    void testRemoveCascadesTasks();

    // Relationships: Tasks (ordered_one_to_many strong)
    void testGetRelationshipTasks();
    void testGetRelationshipTasksCount();

    // Relationships: Tags (many_to_many)
    void testSetAndGetRelationshipTags();

    // Relationships: Settings (one_to_one strong required)
    void testProjectHasSettingsAfterCreate();

    // Relationships: Lead (one_to_one weak optional)
    void testSetAndGetRelationshipLead();

    // Events
    void testCreateEmitsCreatedEvent();
    void testUpdateEmitsUpdatedEvent();
    void testRemoveEmitsRemovedEvent();

  private:
    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
        int projectId;
    };
    ScaffoldIds createScaffold();

    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Task::TaskController *m_taskCtrl = nullptr;
    DA::Tag::TagController *m_tagCtrl = nullptr;
    DA::ProjectSettings::ProjectSettingsController *m_settingsCtrl = nullptr;
    DA::TeamMember::TeamMemberController *m_teamMemberCtrl = nullptr;
    FullCppQtApp::Common::DirectAccess::EventRegistry *m_eventRegistry = nullptr;
};

void TestProjectController::initTestCase()
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

void TestProjectController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestProjectController::init()
{
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_taskCtrl = new DA::Task::TaskController(this);
    m_tagCtrl = new DA::Tag::TagController(this);
    m_settingsCtrl = new DA::ProjectSettings::ProjectSettingsController(this);
    m_teamMemberCtrl = new DA::TeamMember::TeamMemberController(this);
}

void TestProjectController::cleanup()
{
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_taskCtrl;
    m_taskCtrl = nullptr;
    delete m_tagCtrl;
    m_tagCtrl = nullptr;
    delete m_settingsCtrl;
    m_settingsCtrl = nullptr;
    delete m_teamMemberCtrl;
    m_teamMemberCtrl = nullptr;
}

TestProjectController::ScaffoldIds TestProjectController::createScaffold()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws =
        QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto projDto;
    projDto.title = u"TestProject"_s;
    projDto.description = u"Desc"_s;
    projDto.uuid = QUuid::createUuid();
    projDto.isActive = true;
    projDto.priority = 5;
    projDto.budget = 1000.5f;
    projDto.deadline = QDateTime::currentDateTimeUtc();
    auto proj = QCoro::waitFor(m_projectCtrl->create({projDto}, wsId));
    int projId = proj.first().id;

    return {rootId, wsId, projId};
}

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

void TestProjectController::testCreateWithOwner()
{
    auto scaffold = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"MyProject"_s;
    dto.description = u"Desc"_s;
    dto.uuid = QUuid::createUuid();
    dto.isActive = true;
    dto.priority = 5;
    dto.budget = 1000.5f;
    dto.deadline = QDateTime::currentDateTimeUtc();

    auto results = QCoro::waitFor(m_projectCtrl->create({dto}, scaffold.workspaceId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().title, u"MyProject"_s);
    QCOMPARE(results.first().isActive, true);
    QCOMPARE(results.first().priority, 5);
}

void TestProjectController::testCreateMultiple()
{
    auto scaffold = createScaffold();

    DA::Project::CreateProjectDto d1, d2;
    d1.title = u"P1"_s;
    d1.uuid = QUuid::createUuid();
    d1.deadline = QDateTime::currentDateTimeUtc();
    d2.title = u"P2"_s;
    d2.uuid = QUuid::createUuid();
    d2.deadline = QDateTime::currentDateTimeUtc();

    auto results = QCoro::waitFor(m_projectCtrl->create({d1, d2}, scaffold.workspaceId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
}

void TestProjectController::testCreateAtIndex()
{
    auto scaffold = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Inserted"_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();

    auto results = QCoro::waitFor(m_projectCtrl->create({dto}, scaffold.workspaceId, 0));
    QCOMPARE(results.first().title, u"Inserted"_s);

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(scaffold.workspaceId, DA::Workspace::WorkspaceRelationshipField::Projects));
    QCOMPARE(relIds.first(), results.first().id);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestProjectController::testGetById()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().title, u"TestProject"_s);
}

void TestProjectController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_projectCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestProjectController::testGetAll()
{
    auto scaffold = createScaffold();
    Q_UNUSED(scaffold);
    auto all = QCoro::waitFor(m_projectCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update — all field types
// ---------------------------------------------------------------------------

void TestProjectController::testUpdateStringFields()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    proj.title = u"Updated Title"_s;
    proj.description = u"Updated Desc"_s;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().title, u"Updated Title"_s);
    QCOMPARE(updated.first().description, u"Updated Desc"_s);
}

void TestProjectController::testUpdateBoolField()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    proj.isActive = false;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().isActive, false);
}

void TestProjectController::testUpdateNumericFields()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    proj.priority = 99;
    proj.budget = 42.5f;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().priority, 99);
    QCOMPARE(updated.first().budget, 42.5f);
}

void TestProjectController::testUpdateEnumField()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    proj.status = DA::Project::ProjectStatus::Archived;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().status, DA::Project::ProjectStatus::Archived);
}

void TestProjectController::testUpdateUuidField()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    auto newUuid = QUuid::createUuid();
    proj.uuid = newUuid;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().uuid, newUuid);

    auto refetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    QCOMPARE(refetched.first().uuid, newUuid);
}

void TestProjectController::testUpdateDateTimeField()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    auto newDate = QDateTime(QDate(2027, 1, 15), QTime(12, 0, 0));
    proj.deadline = newDate;
    auto updated = QCoro::waitFor(m_projectCtrl->update({proj}));
    QCOMPARE(updated.first().deadline, newDate);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestProjectController::testRemove()
{
    auto scaffold = createScaffold();
    auto removed = QCoro::waitFor(m_projectCtrl->remove({scaffold.projectId}));
    QCOMPARE(removed.size(), 1);

    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    QVERIFY(fetched.isEmpty());
}

void TestProjectController::testRemoveCascadesTasks()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto taskDto;
    taskDto.title = u"ChildTask"_s;
    taskDto.dueDate = QDateTime::currentDateTimeUtc();
    auto task = QCoro::waitFor(m_taskCtrl->create({taskDto}, scaffold.projectId));
    int taskId = task.first().id;

    QCoro::waitFor(m_projectCtrl->remove({scaffold.projectId}));

    auto fetchedTask = QCoro::waitFor(m_taskCtrl->get({taskId}));
    QVERIFY(fetchedTask.isEmpty());
}

// ---------------------------------------------------------------------------
// Relationships: Tasks
// ---------------------------------------------------------------------------

void TestProjectController::testGetRelationshipTasks()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto d1, d2;
    d1.title = u"T1"_s;
    d1.dueDate = QDateTime::currentDateTimeUtc();
    d2.title = u"T2"_s;
    d2.dueDate = QDateTime::currentDateTimeUtc();
    auto tasks = QCoro::waitFor(m_taskCtrl->create({d1, d2}, scaffold.projectId));

    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(relIds.size(), 2);
    QCOMPARE(relIds[0], tasks[0].id);
    QCOMPARE(relIds[1], tasks[1].id);
}

void TestProjectController::testGetRelationshipTasksCount()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto d1, d2, d3;
    d1.title = u"A"_s;
    d1.dueDate = QDateTime::currentDateTimeUtc();
    d2.title = u"B"_s;
    d2.dueDate = QDateTime::currentDateTimeUtc();
    d3.title = u"C"_s;
    d3.dueDate = QDateTime::currentDateTimeUtc();
    QCoro::waitFor(m_taskCtrl->create({d1, d2, d3}, scaffold.projectId));

    int count = QCoro::waitFor(
        m_projectCtrl->getRelationshipIdsCount(scaffold.projectId, DA::Project::ProjectRelationshipField::Tasks));
    QCOMPARE(count, 3);
}

// ---------------------------------------------------------------------------
// Relationships: Tags (many_to_many)
// ---------------------------------------------------------------------------

void TestProjectController::testSetAndGetRelationshipTags()
{
    auto scaffold = createScaffold();

    DA::Tag::CreateTagDto t1, t2;
    t1.name = u"TagA"_s;
    t1.color = u"#F00"_s;
    t2.name = u"TagB"_s;
    t2.color = u"#0F0"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({t1, t2}, scaffold.workspaceId));

    QCoro::waitFor(m_projectCtrl->setRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Tags, {tags[0].id, tags[1].id}));

    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(scaffold.projectId, DA::Project::ProjectRelationshipField::Tags));
    QCOMPARE(relIds.size(), 2);
    QVERIFY(relIds.contains(tags[0].id));
    QVERIFY(relIds.contains(tags[1].id));
}

// ---------------------------------------------------------------------------
// Relationships: Settings (one_to_one strong required)
// ---------------------------------------------------------------------------

void TestProjectController::testProjectHasSettingsAfterCreate()
{
    auto scaffold = createScaffold();

    // Create settings for the project
    DA::ProjectSettings::CreateProjectSettingsDto settingsDto;
    auto settings = QCoro::waitFor(m_settingsCtrl->create({settingsDto}, scaffold.projectId));
    QCOMPARE(settings.size(), 1);
    int settingsId = settings.first().id;

    // Verify relationship
    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(scaffold.projectId, DA::Project::ProjectRelationshipField::Settings));
    QCOMPARE(relIds.size(), 1);
    QCOMPARE(relIds.first(), settingsId);
}

// ---------------------------------------------------------------------------
// Relationships: Lead (one_to_one weak optional)
// ---------------------------------------------------------------------------

void TestProjectController::testSetAndGetRelationshipLead()
{
    auto scaffold = createScaffold();

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Alice"_s;
    memberDto.email = u"alice@test.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, scaffold.workspaceId));
    int memberId = member.first().id;

    QCoro::waitFor(m_projectCtrl->setRelationshipIds(
        scaffold.projectId, DA::Project::ProjectRelationshipField::Lead, {memberId}));

    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(scaffold.projectId, DA::Project::ProjectRelationshipField::Lead));
    QCOMPARE(relIds, QList<int>{memberId});
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

void TestProjectController::testCreateEmitsCreatedEvent()
{
    auto projEvents = m_eventRegistry->projectEvents();
    QSignalSpy spy(projEvents.data(), &FullCppQtApp::Common::DirectAccess::Project::ProjectEvents::created);

    auto scaffold = createScaffold();
    Q_UNUSED(scaffold);

    QTRY_VERIFY(spy.count() >= 1);
}

void TestProjectController::testUpdateEmitsUpdatedEvent()
{
    auto scaffold = createScaffold();
    auto fetched = QCoro::waitFor(m_projectCtrl->get({scaffold.projectId}));
    auto proj = fetched.first();

    auto projEvents = m_eventRegistry->projectEvents();
    QSignalSpy spy(projEvents.data(), &FullCppQtApp::Common::DirectAccess::Project::ProjectEvents::updated);

    proj.title = u"EvtUpdated"_s;
    QCoro::waitFor(m_projectCtrl->update({proj}));

    QTRY_VERIFY(spy.count() >= 1);
}

void TestProjectController::testRemoveEmitsRemovedEvent()
{
    auto scaffold = createScaffold();

    auto projEvents = m_eventRegistry->projectEvents();
    QSignalSpy spy(projEvents.data(), &FullCppQtApp::Common::DirectAccess::Project::ProjectEvents::removed);

    QCoro::waitFor(m_projectCtrl->remove({scaffold.projectId}));

    QTRY_VERIFY(spy.count() >= 1);
}

QTEST_MAIN(TestProjectController)
#include "tst_project_controller.moc"

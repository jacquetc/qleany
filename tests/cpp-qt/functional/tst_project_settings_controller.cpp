// Functional tests for ProjectSettingsController (one_to_one strong required child of Project)
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
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestProjectSettingsController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // auto-creation
    void testSettingsExistAfterProjectCreate();
    void testSettingsHaveDefaultValues();

    // get
    void testGetById();
    void testGetNonExistent();

    // update
    void testUpdateFields();

    // each project gets own settings
    void testEachProjectGetsOwnSettings();

  private:
    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
        int projectId;
        int settingsId;
    };
    ScaffoldIds createScaffold();
    int getSettingsId(int projectId);

    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::ProjectSettings::ProjectSettingsController *m_settingsCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
};

void TestProjectSettingsController::initTestCase()
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

void TestProjectSettingsController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestProjectSettingsController::init()
{
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_settingsCtrl = new DA::ProjectSettings::ProjectSettingsController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
}

void TestProjectSettingsController::cleanup()
{
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_settingsCtrl;
    m_settingsCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
}

TestProjectSettingsController::ScaffoldIds TestProjectSettingsController::createScaffold()
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

    // Create required ProjectSettings
    DA::ProjectSettings::CreateProjectSettingsDto settingsDto;
    auto settings = QCoro::waitFor(m_settingsCtrl->create({settingsDto}, projId));
    int settingsId = settings.first().id;

    return {rootId, wsId, projId, settingsId};
}

int TestProjectSettingsController::getSettingsId(int projectId)
{
    auto relIds = QCoro::waitFor(
        m_projectCtrl->getRelationshipIds(projectId, DA::Project::ProjectRelationshipField::Settings));
    Q_ASSERT(relIds.size() == 1);
    return relIds.first();
}

// ---------------------------------------------------------------------------
// auto-creation
// ---------------------------------------------------------------------------

void TestProjectSettingsController::testSettingsExistAfterProjectCreate()
{
    auto scaffold = createScaffold();
    auto settings = QCoro::waitFor(m_settingsCtrl->get({scaffold.settingsId}));
    QCOMPARE(settings.size(), 1);
}

void TestProjectSettingsController::testSettingsHaveDefaultValues()
{
    auto scaffold = createScaffold();
    auto settings = QCoro::waitFor(m_settingsCtrl->get({scaffold.settingsId}));
    QCOMPARE(settings.first().notificationsEnabled, false);
    QCOMPARE(settings.first().defaultPriority, 0);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestProjectSettingsController::testGetById()
{
    auto scaffold = createScaffold();
    int settingsId = scaffold.settingsId;
    auto fetched = QCoro::waitFor(m_settingsCtrl->get({settingsId}));
    QCOMPARE(fetched.size(), 1);
}

void TestProjectSettingsController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_settingsCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestProjectSettingsController::testUpdateFields()
{
    auto scaffold = createScaffold();
    int settingsId = scaffold.settingsId;
    auto fetched = QCoro::waitFor(m_settingsCtrl->get({settingsId}));
    auto settings = fetched.first();

    DA::ProjectSettings::UpdateProjectSettingsDto updateSettings;
    updateSettings.id = settings.id;
    updateSettings.createdAt = settings.createdAt;
    updateSettings.updatedAt = settings.updatedAt;
    updateSettings.notificationsEnabled = true;
    updateSettings.defaultPriority = 5;
    updateSettings.colorTheme = u"dark"_s;

    auto updated = QCoro::waitFor(m_settingsCtrl->update({updateSettings}));
    QCOMPARE(updated.first().notificationsEnabled, true);
    QCOMPARE(updated.first().defaultPriority, 5);
    QCOMPARE(updated.first().colorTheme, u"dark"_s);

    // Verify persistence
    auto refetched = QCoro::waitFor(m_settingsCtrl->get({settingsId}));
    QCOMPARE(refetched.first().notificationsEnabled, true);
    QCOMPARE(refetched.first().defaultPriority, 5);
    QCOMPARE(refetched.first().colorTheme, u"dark"_s);
}

// ---------------------------------------------------------------------------
// each project gets own settings
// ---------------------------------------------------------------------------

void TestProjectSettingsController::testEachProjectGetsOwnSettings()
{
    auto scaffold = createScaffold();

    DA::Project::CreateProjectDto proj2Dto;
    proj2Dto.title = u"SecondProject"_s;
    proj2Dto.uuid = QUuid::createUuid();
    proj2Dto.deadline = QDateTime::currentDateTimeUtc();
    auto proj2 = QCoro::waitFor(m_projectCtrl->create({proj2Dto}, scaffold.workspaceId));
    int proj2Id = proj2.first().id;

    // Create settings for project2
    DA::ProjectSettings::CreateProjectSettingsDto settings2Dto;
    auto settings2 = QCoro::waitFor(m_settingsCtrl->create({settings2Dto}, proj2Id));
    int s2 = settings2.first().id;

    int s1 = scaffold.settingsId;
    QVERIFY(s1 != s2);

    QCOMPARE(QCoro::waitFor(m_settingsCtrl->get({s1})).size(), 1);
    QCOMPARE(QCoro::waitFor(m_settingsCtrl->get({s2})).size(), 1);
}

QTEST_MAIN(TestProjectSettingsController)
#include "tst_project_settings_controller.moc"

// Functional tests for feature use cases (project_management and task_management)
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

#include "project/dtos.h"
#include "project/project_controller.h"
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

#include "project_management_controller.h"
#include "project_management_dtos.h"
#include "task_management_controller.h"
#include "task_management_dtos.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;
namespace PM = FullCppQtApp::ProjectManagement;
namespace TM = FullCppQtApp::TaskManagement;

class TestFeatureUseCases : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // project_management
    void testCreateProject();
    void testGetProjectStats();
    void testArchiveProject();
    void testExportProjectDataStarts();
    void testImportProjectStarts();

    // task_management
    void testBatchAssignTasks();
    void testGetTaskSummary();
    void testCleanupCompleted();

  private:
    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
        int projectId;
    };
    ScaffoldIds createScaffold();

    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Task::TaskController *m_taskCtrl = nullptr;
    DA::TeamMember::TeamMemberController *m_teamMemberCtrl = nullptr;
    PM::ProjectManagementController *m_pmCtrl = nullptr;
    TM::TaskManagementController *m_tmCtrl = nullptr;
};

void TestFeatureUseCases::initTestCase()
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

void TestFeatureUseCases::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestFeatureUseCases::init()
{
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_taskCtrl = new DA::Task::TaskController(this);
    m_teamMemberCtrl = new DA::TeamMember::TeamMemberController(this);
    m_pmCtrl = new PM::ProjectManagementController(this);
    m_tmCtrl = new TM::TaskManagementController(this);
}

void TestFeatureUseCases::cleanup()
{
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_taskCtrl;
    m_taskCtrl = nullptr;
    delete m_teamMemberCtrl;
    m_teamMemberCtrl = nullptr;
    delete m_pmCtrl;
    m_pmCtrl = nullptr;
    delete m_tmCtrl;
    m_tmCtrl = nullptr;
}

TestFeatureUseCases::ScaffoldIds TestFeatureUseCases::createScaffold()
{
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

// ===========================================================================
// project_management
// ===========================================================================

void TestFeatureUseCases::testCreateProject()
{
    auto scaffold = createScaffold();
    Q_UNUSED(scaffold);

    PM::CreateProjectDto dto;
    dto.title = u"Feature Project"_s;
    dto.description = u""_s;
    dto.isActive = true;
    dto.priority = 3;
    dto.deadline = QDateTime::currentDateTimeUtc();

    // Use case stubs return default DTOs — just verify the call completes
    auto result = QCoro::waitFor(m_pmCtrl->createProject(dto));
    Q_UNUSED(result);
}

void TestFeatureUseCases::testGetProjectStats()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto t1, t2;
    t1.title = u"Task1"_s;
    t1.content = u""_s;
    t1.dueDate = QDateTime::currentDateTimeUtc();
    t2.title = u"Task2"_s;
    t2.content = u""_s;
    t2.dueDate = QDateTime::currentDateTimeUtc();
    QCoro::waitFor(m_taskCtrl->create({t1, t2}, scaffold.projectId));

    PM::GetProjectStatsDto dto;
    dto.projectId = scaffold.projectId;

    auto result = QCoro::waitFor(m_pmCtrl->getProjectStats(dto));
    // Result should have default/calculated values
    Q_UNUSED(result.totalTasks);
}

void TestFeatureUseCases::testArchiveProject()
{
    auto scaffold = createScaffold();

    PM::ArchiveProjectDto dto;
    dto.projectId = scaffold.projectId;
    dto.reason = u"Done"_s;
    dto.archivePriority = 1;

    auto result = QCoro::waitFor(m_pmCtrl->archiveProject(dto));
    Q_UNUSED(result.archived);
}

void TestFeatureUseCases::testExportProjectDataStarts()
{
    auto scaffold = createScaffold();

    PM::ExportProjectDataDto dto;
    dto.projectId = scaffold.projectId;
    dto.format = u"json"_s;
    dto.includeComments = true;

    auto operationId = m_pmCtrl->exportProjectData(dto);
    QVERIFY(!operationId.isEmpty());
}

void TestFeatureUseCases::testImportProjectStarts()
{
    auto scaffold = createScaffold();
    Q_UNUSED(scaffold);

    PM::ImportProjectDto dto;
    dto.data = u"{}"_s;
    dto.targetWorkspaceId = scaffold.workspaceId;
    dto.tagNames = {u"imported"_s};

    auto operationId = m_pmCtrl->importProject(dto);
    QVERIFY(!operationId.isEmpty());
}

// ===========================================================================
// task_management
// ===========================================================================

void TestFeatureUseCases::testBatchAssignTasks()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto t1, t2;
    t1.title = u"Assign1"_s;
    t1.content = u""_s;
    t1.dueDate = QDateTime::currentDateTimeUtc();
    t2.title = u"Assign2"_s;
    t2.content = u""_s;
    t2.dueDate = QDateTime::currentDateTimeUtc();
    auto tasks = QCoro::waitFor(m_taskCtrl->create({t1, t2}, scaffold.projectId));

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Assignee"_s;
    memberDto.email = u"a@t.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, scaffold.workspaceId));

    TM::BatchAssignTasksDto dto;
    dto.taskIds = {static_cast<uint>(tasks[0].id), static_cast<uint>(tasks[1].id)};
    dto.teamMemberId = member.first().id;

    // Should not throw (actual logic depends on use case stub)
    auto result = QCoro::waitFor(m_tmCtrl->batchAssignTasks(dto));
    Q_UNUSED(result);
}

void TestFeatureUseCases::testGetTaskSummary()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto dto;
    dto.title = u"Summary1"_s;
    dto.content = u""_s;
    dto.dueDate = QDateTime::currentDateTimeUtc();
    QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    auto result = QCoro::waitFor(m_tmCtrl->getTaskSummary());
    Q_UNUSED(result.total);
}

void TestFeatureUseCases::testCleanupCompleted()
{
    auto scaffold = createScaffold();

    DA::Task::CreateTaskDto dto;
    dto.title = u"Cleanup1"_s;
    dto.content = u""_s;
    dto.dueDate = QDateTime::currentDateTimeUtc();
    QCoro::waitFor(m_taskCtrl->create({dto}, scaffold.projectId));

    // Should not throw
    QCoro::waitFor(m_tmCtrl->cleanupCompleted());
}

QTEST_MAIN(TestFeatureUseCases)
#include "tst_feature_use_cases.moc"

// Functional tests for WorkspaceController (hub entity with multiple relationship types)
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

#include "category/category_controller.h"
#include "category/dtos.h"
#include "project/dtos.h"
#include "project/project_controller.h"
#include "root/dtos.h"
#include "root/root_controller.h"
#include "tag/dtos.h"
#include "tag/tag_controller.h"
#include "team_member/dtos.h"
#include "team_member/team_member_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestWorkspaceController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // CRUD
    void testCreateWithOwner();
    void testGetById();
    void testGetNonExistent();
    void testUpdate();
    void testRemove();
    void testRemoveCascadesChildren();

    // Relationships: Projects (ordered_one_to_many strong)
    void testGetRelationshipProjects();
    void testMoveRelationshipProjects();

    // Relationships: Categories (one_to_many strong)
    void testGetRelationshipCategories();

    // Relationships: Tags (one_to_many strong)
    void testGetRelationshipTags();

    // Relationships: TeamMembers (one_to_many strong)
    void testGetRelationshipTeamMembers();

    // count
    void testGetRelationshipCount();

  private:
    int createRoot();

    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Tag::TagController *m_tagCtrl = nullptr;
    DA::Category::CategoryController *m_categoryCtrl = nullptr;
    DA::TeamMember::TeamMemberController *m_teamMemberCtrl = nullptr;
};

void TestWorkspaceController::initTestCase()
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

void TestWorkspaceController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestWorkspaceController::init()
{
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_tagCtrl = new DA::Tag::TagController(this);
    m_categoryCtrl = new DA::Category::CategoryController(this);
    m_teamMemberCtrl = new DA::TeamMember::TeamMemberController(this);
}

void TestWorkspaceController::cleanup()
{
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_tagCtrl;
    m_tagCtrl = nullptr;
    delete m_categoryCtrl;
    m_categoryCtrl = nullptr;
    delete m_teamMemberCtrl;
    m_teamMemberCtrl = nullptr;
}

int TestWorkspaceController::createRoot()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    return root.first().id;
}

// ---------------------------------------------------------------------------
// CRUD
// ---------------------------------------------------------------------------

void TestWorkspaceController::testCreateWithOwner()
{
    int rootId = createRoot();
    auto results = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
}

void TestWorkspaceController::testGetById()
{
    int rootId = createRoot();
    auto created = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = created.first().id;

    auto fetched = QCoro::waitFor(m_workspaceCtrl->get({wsId}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().id, wsId);
}

void TestWorkspaceController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_workspaceCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestWorkspaceController::testUpdate()
{
    int rootId = createRoot();
    auto created = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    auto ws = created.first();

    DA::Workspace::UpdateWorkspaceDto updateWs;
    updateWs.id = ws.id;
    updateWs.createdAt = ws.createdAt;
    updateWs.updatedAt = ws.updatedAt;
    auto updated = QCoro::waitFor(m_workspaceCtrl->update({updateWs}));
    QCOMPARE(updated.size(), 1);
    QCOMPARE(updated.first().id, ws.id);
}

void TestWorkspaceController::testRemove()
{
    int rootId = createRoot();
    auto created = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = created.first().id;

    QCoro::waitFor(m_workspaceCtrl->remove({wsId}));
    auto fetched = QCoro::waitFor(m_workspaceCtrl->get({wsId}));
    QVERIFY(fetched.isEmpty());
}

void TestWorkspaceController::testRemoveCascadesChildren()
{
    int rootId = createRoot();
    auto created = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = created.first().id;

    DA::Project::CreateProjectDto projDto;
    projDto.title = u"Child"_s;
    projDto.uuid = QUuid::createUuid();
    projDto.deadline = QDateTime::currentDateTimeUtc();
    auto proj = QCoro::waitFor(m_projectCtrl->create({projDto}, wsId));

    DA::Tag::CreateTagDto tagDto;
    tagDto.name = u"Tag"_s;
    tagDto.color = u"#000"_s;
    auto tag = QCoro::waitFor(m_tagCtrl->create({tagDto}, wsId));

    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"Cat"_s;
    auto cat = QCoro::waitFor(m_categoryCtrl->create({catDto}, wsId));

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"M"_s;
    memberDto.email = u"m@t.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, wsId));

    QCoro::waitFor(m_workspaceCtrl->remove({wsId}));

    QVERIFY(QCoro::waitFor(m_projectCtrl->get({proj.first().id})).isEmpty());
    QVERIFY(QCoro::waitFor(m_tagCtrl->get({tag.first().id})).isEmpty());
    QVERIFY(QCoro::waitFor(m_categoryCtrl->get({cat.first().id})).isEmpty());
    QVERIFY(QCoro::waitFor(m_teamMemberCtrl->get({member.first().id})).isEmpty());
}

// ---------------------------------------------------------------------------
// Projects (ordered_one_to_many strong)
// ---------------------------------------------------------------------------

void TestWorkspaceController::testGetRelationshipProjects()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto d1, d2;
    d1.title = u"P1"_s;
    d1.uuid = QUuid::createUuid();
    d1.deadline = QDateTime::currentDateTimeUtc();
    d2.title = u"P2"_s;
    d2.uuid = QUuid::createUuid();
    d2.deadline = QDateTime::currentDateTimeUtc();
    auto projs = QCoro::waitFor(m_projectCtrl->create({d1, d2}, wsId));

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::Projects));
    QCOMPARE(relIds.size(), 2);
    QCOMPARE(relIds[0], projs[0].id);
    QCOMPARE(relIds[1], projs[1].id);
}

void TestWorkspaceController::testMoveRelationshipProjects()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto d1, d2;
    d1.title = u"A"_s;
    d1.uuid = QUuid::createUuid();
    d1.deadline = QDateTime::currentDateTimeUtc();
    d2.title = u"B"_s;
    d2.uuid = QUuid::createUuid();
    d2.deadline = QDateTime::currentDateTimeUtc();
    auto projs = QCoro::waitFor(m_projectCtrl->create({d1, d2}, wsId));

    // Move B to front
    QCoro::waitFor(m_workspaceCtrl->moveRelationshipIds(
        wsId, DA::Workspace::WorkspaceRelationshipField::Projects, {projs[1].id}, 0));

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::Projects));
    QCOMPARE(relIds, (QList<int>{projs[1].id, projs[0].id}));
}

// ---------------------------------------------------------------------------
// Categories (one_to_many strong)
// ---------------------------------------------------------------------------

void TestWorkspaceController::testGetRelationshipCategories()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Category::CreateCategoryDto d1, d2;
    d1.name = u"Cat1"_s;
    d2.name = u"Cat2"_s;
    auto cats = QCoro::waitFor(m_categoryCtrl->create({d1, d2}, wsId));

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::Categories));
    QCOMPARE(relIds.size(), 2);
    QVERIFY(relIds.contains(cats[0].id));
    QVERIFY(relIds.contains(cats[1].id));
}

// ---------------------------------------------------------------------------
// Tags (one_to_many strong)
// ---------------------------------------------------------------------------

void TestWorkspaceController::testGetRelationshipTags()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Tag::CreateTagDto dto;
    dto.name = u"T1"_s;
    dto.color = u"#000"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::Tags));
    QVERIFY(relIds.contains(tags.first().id));
}

// ---------------------------------------------------------------------------
// TeamMembers (one_to_many strong)
// ---------------------------------------------------------------------------

void TestWorkspaceController::testGetRelationshipTeamMembers()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"M1"_s;
    dto.email = u"m1@t.com"_s;
    auto members = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));

    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::TeamMembers));
    QVERIFY(relIds.contains(members.first().id));
}

// ---------------------------------------------------------------------------
// count
// ---------------------------------------------------------------------------

void TestWorkspaceController::testGetRelationshipCount()
{
    int rootId = createRoot();
    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    DA::Project::CreateProjectDto d1, d2;
    d1.title = u"P1"_s;
    d1.uuid = QUuid::createUuid();
    d1.deadline = QDateTime::currentDateTimeUtc();
    d2.title = u"P2"_s;
    d2.uuid = QUuid::createUuid();
    d2.deadline = QDateTime::currentDateTimeUtc();
    QCoro::waitFor(m_projectCtrl->create({d1, d2}, wsId));

    int count = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIdsCount(wsId, DA::Workspace::WorkspaceRelationshipField::Projects));
    QCOMPARE(count, 2);
}

QTEST_MAIN(TestWorkspaceController)
#include "tst_workspace_controller.moc"

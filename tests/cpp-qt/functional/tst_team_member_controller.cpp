// Functional tests for TeamMemberController (many_to_one relationship, enum field)
#include <QCoreApplication>
#include <QTest>
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
#include "root/dtos.h"
#include "root/root_controller.h"
#include "team_member/dtos.h"
#include "team_member/team_member_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestTeamMemberController : public QObject
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

    // get
    void testGetById();
    void testGetNonExistent();
    void testGetAll();

    // update
    void testUpdateFields();
    void testUpdateEnumRole();

    // remove
    void testRemove();

    // Relationships: Department (many_to_one weak)
    void testSetAndGetDepartment();
    void testChangeDepartment();
    void testMultipleMembersSameDepartment();

  private:
    int createWorkspace();

    DA::TeamMember::TeamMemberController *m_teamMemberCtrl = nullptr;
    DA::Category::CategoryController *m_categoryCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
};

void TestTeamMemberController::initTestCase()
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

void TestTeamMemberController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestTeamMemberController::init()
{
    m_teamMemberCtrl = new DA::TeamMember::TeamMemberController(this);
    m_categoryCtrl = new DA::Category::CategoryController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
}

void TestTeamMemberController::cleanup()
{
    delete m_teamMemberCtrl;
    m_teamMemberCtrl = nullptr;
    delete m_categoryCtrl;
    m_categoryCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
}

int TestTeamMemberController::createWorkspace()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    auto ws =
        QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, root.first().id));
    return ws.first().id;
}

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

void TestTeamMemberController::testCreateWithOwner()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"Alice"_s;
    dto.email = u"alice@test.com"_s;
    dto.role = DA::TeamMember::MemberRole::Developer;

    auto results = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().name, u"Alice"_s);
    QCOMPARE(results.first().email, u"alice@test.com"_s);
    QCOMPARE(results.first().role, DA::TeamMember::MemberRole::Developer);
}

void TestTeamMemberController::testCreateMultiple()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto d1, d2;
    d1.name = u"Bob"_s;
    d1.email = u"bob@test.com"_s;
    d1.role = DA::TeamMember::MemberRole::Designer;
    d2.name = u"Carol"_s;
    d2.email = u"carol@test.com"_s;
    d2.role = DA::TeamMember::MemberRole::Tester;

    auto results = QCoro::waitFor(m_teamMemberCtrl->create({d1, d2}, wsId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestTeamMemberController::testGetById()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"GetMe"_s;
    dto.email = u"get@test.com"_s;
    auto created = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));

    auto fetched = QCoro::waitFor(m_teamMemberCtrl->get({created.first().id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().name, u"GetMe"_s);
}

void TestTeamMemberController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_teamMemberCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestTeamMemberController::testGetAll()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"All"_s;
    dto.email = u"all@test.com"_s;
    QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));

    auto all = QCoro::waitFor(m_teamMemberCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestTeamMemberController::testUpdateFields()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"Old"_s;
    dto.email = u"old@test.com"_s;
    auto created = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));
    auto member = created.first();

    member.name = u"New"_s;
    member.email = u"new@test.com"_s;
    auto updated = QCoro::waitFor(m_teamMemberCtrl->update({member}));
    QCOMPARE(updated.first().name, u"New"_s);
    QCOMPARE(updated.first().email, u"new@test.com"_s);
}

void TestTeamMemberController::testUpdateEnumRole()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"Enum"_s;
    dto.email = u"enum@test.com"_s;
    dto.role = DA::TeamMember::MemberRole::Developer;
    auto created = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));
    auto member = created.first();

    member.role = DA::TeamMember::MemberRole::Manager;
    auto updated = QCoro::waitFor(m_teamMemberCtrl->update({member}));
    QCOMPARE(updated.first().role, DA::TeamMember::MemberRole::Manager);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestTeamMemberController::testRemove()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto dto;
    dto.name = u"Del"_s;
    dto.email = u"del@test.com"_s;
    auto created = QCoro::waitFor(m_teamMemberCtrl->create({dto}, wsId));
    int id = created.first().id;

    QCoro::waitFor(m_teamMemberCtrl->remove({id}));
    auto fetched = QCoro::waitFor(m_teamMemberCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// Department (many_to_one weak)
// ---------------------------------------------------------------------------

void TestTeamMemberController::testSetAndGetDepartment()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Dept"_s;
    memberDto.email = u"dept@test.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, wsId));
    int memberId = member.first().id;

    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"Engineering"_s;
    auto cat = QCoro::waitFor(m_categoryCtrl->create({catDto}, wsId));
    int catId = cat.first().id;

    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        memberId, DA::TeamMember::TeamMemberRelationshipField::Department, {catId}));

    auto relIds = QCoro::waitFor(
        m_teamMemberCtrl->getRelationshipIds(memberId, DA::TeamMember::TeamMemberRelationshipField::Department));
    QCOMPARE(relIds, QList<int>{catId});
}

void TestTeamMemberController::testChangeDepartment()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Switch"_s;
    memberDto.email = u"switch@test.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, wsId));
    int memberId = member.first().id;

    DA::Category::CreateCategoryDto c1Dto, c2Dto;
    c1Dto.name = u"Dept1"_s;
    c2Dto.name = u"Dept2"_s;
    auto cat1 = QCoro::waitFor(m_categoryCtrl->create({c1Dto}, wsId));
    auto cat2 = QCoro::waitFor(m_categoryCtrl->create({c2Dto}, wsId));

    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        memberId, DA::TeamMember::TeamMemberRelationshipField::Department, {cat1.first().id}));

    // Change department
    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        memberId, DA::TeamMember::TeamMemberRelationshipField::Department, {cat2.first().id}));

    auto relIds = QCoro::waitFor(
        m_teamMemberCtrl->getRelationshipIds(memberId, DA::TeamMember::TeamMemberRelationshipField::Department));
    QCOMPARE(relIds, QList<int>{cat2.first().id});
}

void TestTeamMemberController::testMultipleMembersSameDepartment()
{
    int wsId = createWorkspace();

    DA::TeamMember::CreateTeamMemberDto d1, d2;
    d1.name = u"M1"_s;
    d1.email = u"m1@test.com"_s;
    d2.name = u"M2"_s;
    d2.email = u"m2@test.com"_s;
    auto members = QCoro::waitFor(m_teamMemberCtrl->create({d1, d2}, wsId));

    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"Shared"_s;
    auto cat = QCoro::waitFor(m_categoryCtrl->create({catDto}, wsId));
    int catId = cat.first().id;

    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        members[0].id, DA::TeamMember::TeamMemberRelationshipField::Department, {catId}));
    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        members[1].id, DA::TeamMember::TeamMemberRelationshipField::Department, {catId}));

    auto rel1 = QCoro::waitFor(
        m_teamMemberCtrl->getRelationshipIds(members[0].id, DA::TeamMember::TeamMemberRelationshipField::Department));
    auto rel2 = QCoro::waitFor(
        m_teamMemberCtrl->getRelationshipIds(members[1].id, DA::TeamMember::TeamMemberRelationshipField::Department));
    QCOMPARE(rel1, QList<int>{catId});
    QCOMPARE(rel2, QList<int>{catId});
}

QTEST_MAIN(TestTeamMemberController)
#include "tst_team_member_controller.moc"

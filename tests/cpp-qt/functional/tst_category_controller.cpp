// Functional tests for CategoryController (leaf entity, target of many_to_one relationships)
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

class TestCategoryController : public QObject
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

    // remove
    void testRemove();

    // referential integrity
    void testDeleteCategoryLeavesTeamMemberIntact();
    void testDeleteCategoryDoesNotDeleteTeamMember();

  private:
    int createWorkspace();

    DA::Category::CategoryController *m_categoryCtrl = nullptr;
    DA::TeamMember::TeamMemberController *m_teamMemberCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
};

void TestCategoryController::initTestCase()
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

void TestCategoryController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestCategoryController::init()
{
    m_categoryCtrl = new DA::Category::CategoryController(this);
    m_teamMemberCtrl = new DA::TeamMember::TeamMemberController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
}

void TestCategoryController::cleanup()
{
    delete m_categoryCtrl;
    m_categoryCtrl = nullptr;
    delete m_teamMemberCtrl;
    m_teamMemberCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
}

int TestCategoryController::createWorkspace()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    auto ws =
        QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, root.first().id));
    return ws.first().id;
}

// ---------------------------------------------------------------------------
// create
// ---------------------------------------------------------------------------

void TestCategoryController::testCreateWithOwner()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto dto;
    dto.name = u"Backend"_s;
    dto.description = u"Backend team"_s;
    dto.icon = u"server"_s;

    auto results = QCoro::waitFor(m_categoryCtrl->create({dto}, wsId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().name, u"Backend"_s);
    QCOMPARE(results.first().description, u"Backend team"_s);
    QCOMPARE(results.first().icon, u"server"_s);
}

void TestCategoryController::testCreateMultiple()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto d1, d2;
    d1.name = u"A"_s;
    d2.name = u"B"_s;

    auto results = QCoro::waitFor(m_categoryCtrl->create({d1, d2}, wsId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestCategoryController::testGetById()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto dto;
    dto.name = u"GetCat"_s;
    auto created = QCoro::waitFor(m_categoryCtrl->create({dto}, wsId));

    auto fetched = QCoro::waitFor(m_categoryCtrl->get({created.first().id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().name, u"GetCat"_s);
}

void TestCategoryController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_categoryCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestCategoryController::testGetAll()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto dto;
    dto.name = u"AllCat"_s;
    QCoro::waitFor(m_categoryCtrl->create({dto}, wsId));

    auto all = QCoro::waitFor(m_categoryCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestCategoryController::testUpdateFields()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto dto;
    dto.name = u"OldCat"_s;
    auto created = QCoro::waitFor(m_categoryCtrl->create({dto}, wsId));
    auto cat = created.first();

    cat.name = u"NewCat"_s;
    cat.description = u"Updated"_s;
    cat.icon = u"folder"_s;
    auto updated = QCoro::waitFor(m_categoryCtrl->update({cat}));
    QCOMPARE(updated.first().name, u"NewCat"_s);
    QCOMPARE(updated.first().description, u"Updated"_s);
    QCOMPARE(updated.first().icon, u"folder"_s);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestCategoryController::testRemove()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto dto;
    dto.name = u"DelCat"_s;
    auto created = QCoro::waitFor(m_categoryCtrl->create({dto}, wsId));
    int id = created.first().id;

    QCoro::waitFor(m_categoryCtrl->remove({id}));
    auto fetched = QCoro::waitFor(m_categoryCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// referential integrity
// ---------------------------------------------------------------------------

void TestCategoryController::testDeleteCategoryLeavesTeamMemberIntact()
{
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"ToDelete"_s;
    auto cat = QCoro::waitFor(m_categoryCtrl->create({catDto}, wsId));
    int catId = cat.first().id;

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Member"_s;
    memberDto.email = u"m@test.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, wsId));
    int memberId = member.first().id;

    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        memberId, DA::TeamMember::TeamMemberRelationshipField::Department, {catId}));

    // Delete the category
    QCoro::waitFor(m_categoryCtrl->remove({catId}));

    // Team member still exists
    auto fetchedMember = QCoro::waitFor(m_teamMemberCtrl->get({memberId}));
    QCOMPARE(fetchedMember.size(), 1);
    QCOMPARE(fetchedMember.first().name, u"Member"_s);
}

void TestCategoryController::testDeleteCategoryDoesNotDeleteTeamMember()
{
    // Deleting a weak reference target does not affect the referencing entity
    int wsId = createWorkspace();

    DA::Category::CreateCategoryDto catDto;
    catDto.name = u"WillGo"_s;
    auto cat = QCoro::waitFor(m_categoryCtrl->create({catDto}, wsId));
    int catId = cat.first().id;

    DA::TeamMember::CreateTeamMemberDto memberDto;
    memberDto.name = u"Ref"_s;
    memberDto.email = u"ref@test.com"_s;
    auto member = QCoro::waitFor(m_teamMemberCtrl->create({memberDto}, wsId));
    int memberId = member.first().id;

    QCoro::waitFor(m_teamMemberCtrl->setRelationshipIds(
        memberId, DA::TeamMember::TeamMemberRelationshipField::Department, {catId}));

    // Delete category
    QCoro::waitFor(m_categoryCtrl->remove({catId}));

    // Team member still exists
    auto fetchedMember = QCoro::waitFor(m_teamMemberCtrl->get({memberId}));
    QCOMPARE(fetchedMember.size(), 1);
}

QTEST_MAIN(TestCategoryController)
#include "tst_category_controller.moc"

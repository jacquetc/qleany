// Functional tests for RootController (orphan entity — no owner, has relationships)
#include <QCoreApplication>
#include <QSignalSpy>
#include <QTest>
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
#include "system/dtos.h"
#include "system/system_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestRootController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // createOrphans
    void testCreateOrphan();
    void testCreateMultipleOrphans();

    // get
    void testGetById();
    void testGetNonExistent();

    // getAll
    void testGetAll();
    void testGetAllDoesNotCrash();

    // update
    void testUpdate();

    // remove
    void testRemove();
    void testRemoveNonExistent();

    // Relationships
    void testSetAndGetRelationshipWorkspace();
    void testSetAndGetRelationshipSystem();
    void testGetRelationshipIdsCount();
    void testGetRelationshipIdsInRange();

    // Events
    void testCreateEmitsCreatedEvent();
    void testUpdateEmitsUpdatedEvent();
    void testRemoveEmitsRemovedEvent();

  private:
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::System::SystemController *m_systemCtrl = nullptr;
    FullCppQtApp::Common::DirectAccess::EventRegistry *m_eventRegistry = nullptr;
};

void TestRootController::initTestCase()
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

void TestRootController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestRootController::init()
{
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_systemCtrl = new DA::System::SystemController(this);
}

void TestRootController::cleanup()
{
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_systemCtrl;
    m_systemCtrl = nullptr;
}

// ---------------------------------------------------------------------------
// createOrphans
// ---------------------------------------------------------------------------

void TestRootController::testCreateOrphan()
{
    auto createDto = DA::Root::RootController::getCreateDto();
    auto results = QCoro::waitFor(m_rootCtrl->createOrphans({createDto}));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
}

void TestRootController::testCreateMultipleOrphans()
{
    auto dto1 = DA::Root::RootController::getCreateDto();
    auto dto2 = DA::Root::RootController::getCreateDto();
    auto results = QCoro::waitFor(m_rootCtrl->createOrphans({dto1, dto2}));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestRootController::testGetById()
{
    auto created = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int id = created.first().id;

    auto fetched = QCoro::waitFor(m_rootCtrl->get({id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().id, id);
}

void TestRootController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_rootCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// getAll
// ---------------------------------------------------------------------------

void TestRootController::testGetAll()
{
    QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    auto all = QCoro::waitFor(m_rootCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

void TestRootController::testGetAllDoesNotCrash()
{
    // getAll should never crash, even when previous tests have populated the DB
    auto all = QCoro::waitFor(m_rootCtrl->getAll());
    Q_UNUSED(all);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestRootController::testUpdate()
{
    auto created = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    auto dto = created.first();

    DA::Root::UpdateRootDto updateDto;
    updateDto.id = dto.id;
    updateDto.createdAt = dto.createdAt;
    updateDto.updatedAt = dto.updatedAt;
    auto updated = QCoro::waitFor(m_rootCtrl->update({updateDto}));
    QCOMPARE(updated.size(), 1);
    QCOMPARE(updated.first().id, dto.id);

    // Root has no user-settable fields — verify the entity is still fetchable
    // and updatedAt was set (the update use case auto-sets it)
    auto fetched = QCoro::waitFor(m_rootCtrl->get({dto.id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().id, dto.id);
    QVERIFY(fetched.first().updatedAt.isValid());
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestRootController::testRemove()
{
    auto created = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int id = created.first().id;

    auto removed = QCoro::waitFor(m_rootCtrl->remove({id}));
    QCOMPARE(removed.size(), 1);
    QCOMPARE(removed.first(), id);

    auto fetched = QCoro::waitFor(m_rootCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

void TestRootController::testRemoveNonExistent()
{
    // Removing a non-existent ID should not crash
    auto removed = QCoro::waitFor(m_rootCtrl->remove({999999}));
    Q_UNUSED(removed);
}

// ---------------------------------------------------------------------------
// Relationships
// ---------------------------------------------------------------------------

void TestRootController::testSetAndGetRelationshipWorkspace()
{
    // Create a root and a workspace
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws = QCoro::waitFor(m_workspaceCtrl->createOrphans({DA::Workspace::WorkspaceController::getCreateDto()}));
    int wsId = ws.first().id;

    // Set the Workspace relationship
    QCoro::waitFor(
        m_rootCtrl->setRelationshipIds(rootId, DA::Root::RootRelationshipField::Workspace, {wsId}));

    // Read it back
    auto relIds =
        QCoro::waitFor(m_rootCtrl->getRelationshipIds(rootId, DA::Root::RootRelationshipField::Workspace));
    QCOMPARE(relIds, QList<int>{wsId});
}

void TestRootController::testSetAndGetRelationshipSystem()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    DA::System::CreateSystemDto sysDto;
    sysDto.name = u"TestSystem"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->createOrphans({sysDto}));
    int sysId = sys.first().id;

    QCoro::waitFor(
        m_rootCtrl->setRelationshipIds(rootId, DA::Root::RootRelationshipField::System, {sysId}));

    auto relIds =
        QCoro::waitFor(m_rootCtrl->getRelationshipIds(rootId, DA::Root::RootRelationshipField::System));
    QCOMPARE(relIds, QList<int>{sysId});
}

void TestRootController::testGetRelationshipIdsCount()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws = QCoro::waitFor(m_workspaceCtrl->createOrphans({DA::Workspace::WorkspaceController::getCreateDto()}));
    QCoro::waitFor(
        m_rootCtrl->setRelationshipIds(rootId, DA::Root::RootRelationshipField::Workspace, {ws.first().id}));

    int count = QCoro::waitFor(
        m_rootCtrl->getRelationshipIdsCount(rootId, DA::Root::RootRelationshipField::Workspace));
    QCOMPARE(count, 1);
}

void TestRootController::testGetRelationshipIdsInRange()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws = QCoro::waitFor(m_workspaceCtrl->createOrphans({DA::Workspace::WorkspaceController::getCreateDto()}));
    QCoro::waitFor(
        m_rootCtrl->setRelationshipIds(rootId, DA::Root::RootRelationshipField::Workspace, {ws.first().id}));

    auto rangeIds = QCoro::waitFor(
        m_rootCtrl->getRelationshipIdsInRange(rootId, DA::Root::RootRelationshipField::Workspace, 0, 10));
    QCOMPARE(rangeIds.size(), 1);
    QCOMPARE(rangeIds.first(), ws.first().id);
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

void TestRootController::testCreateEmitsCreatedEvent()
{
    auto rootEvents = m_eventRegistry->rootEvents();
    QSignalSpy spy(rootEvents.data(), &FullCppQtApp::Common::DirectAccess::Root::RootEvents::created);

    QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.first().first().value<QList<int>>();
    QVERIFY(!ids.isEmpty());
}

void TestRootController::testUpdateEmitsUpdatedEvent()
{
    auto created = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    auto dto = created.first();

    auto rootEvents = m_eventRegistry->rootEvents();
    QSignalSpy spy(rootEvents.data(), &FullCppQtApp::Common::DirectAccess::Root::RootEvents::updated);

    DA::Root::UpdateRootDto updateDto;
    updateDto.id = dto.id;
    updateDto.createdAt = dto.createdAt;
    updateDto.updatedAt = QDateTime::currentDateTime();
    QCoro::waitFor(m_rootCtrl->update({updateDto}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.first().first().value<QList<int>>();
    QVERIFY(ids.contains(dto.id));
}

void TestRootController::testRemoveEmitsRemovedEvent()
{
    auto created = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int id = created.first().id;

    auto rootEvents = m_eventRegistry->rootEvents();
    QSignalSpy spy(rootEvents.data(), &FullCppQtApp::Common::DirectAccess::Root::RootEvents::removed);

    QCoro::waitFor(m_rootCtrl->remove({id}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.first().first().value<QList<int>>();
    QVERIFY(ids.contains(id));
}

QTEST_MAIN(TestRootController)
#include "tst_root_controller.moc"

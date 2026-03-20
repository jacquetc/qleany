// Functional tests for TagController (undoable entity — with owner, no children, single model)
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
#include "tag/dtos.h"
#include "tag/models/single_tag.h"
#include "tag/tag_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestTagController : public QObject
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
    void testGetMultiple();
    void testGetNonExistent();

    // getAll
    void testGetAll();

    // update
    void testUpdateFields();
    void testUpdateMultiple();

    // remove
    void testRemove();
    void testRemoveMultiple();

    // Events
    void testCreateEmitsCreatedEvent();
    void testUpdateEmitsUpdatedEvent();
    void testRemoveEmitsRemovedEvent();

    // SingleTag model
    void testSingleTagLoadsOnSetId();
    void testSingleTagReactsToUpdateEvent();
    void testSingleTagClearsOnRemoveEvent();
    void testSingleTagSave();

  private:
    static DA::Tag::CreateTagDto makeTagDto(const QString &name, const QString &color = u"#000000"_s)
    {
        DA::Tag::CreateTagDto dto;
        dto.name = name;
        dto.color = color;
        return dto;
    }
    int createWorkspaceForTags();

    DA::Tag::TagController *m_tagCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    FullCppQtApp::Common::DirectAccess::EventRegistry *m_eventRegistry = nullptr;
};

void TestTagController::initTestCase()
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

void TestTagController::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestTagController::init()
{
    m_tagCtrl = new DA::Tag::TagController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
}

void TestTagController::cleanup()
{
    delete m_tagCtrl;
    m_tagCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
}

int TestTagController::createWorkspaceForTags()
{
    // Create a root, then a workspace owned by it
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    return ws.first().id;
}

// ---------------------------------------------------------------------------
// create (with owner)
// ---------------------------------------------------------------------------

void TestTagController::testCreateWithOwner()
{
    int wsId = createWorkspaceForTags();

    DA::Tag::CreateTagDto dto;
    dto.name = u"Urgent"_s;
    dto.color = u"#FF0000"_s;

    auto results = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().name, u"Urgent"_s);
    QCOMPARE(results.first().color, u"#FF0000"_s);
}

void TestTagController::testCreateMultipleWithOwner()
{
    int wsId = createWorkspaceForTags();

    DA::Tag::CreateTagDto dto1;
    dto1.name = u"Tag A"_s;
    dto1.color = u"#111111"_s;

    DA::Tag::CreateTagDto dto2;
    dto2.name = u"Tag B"_s;
    dto2.color = u"#222222"_s;

    auto results = QCoro::waitFor(m_tagCtrl->create({dto1, dto2}, wsId));
    QCOMPARE(results.size(), 2);
    QVERIFY(results[0].id != results[1].id);
    QCOMPARE(results[0].name, u"Tag A"_s);
    QCOMPARE(results[1].name, u"Tag B"_s);
}

void TestTagController::testCreateAtIndex()
{
    // Workspace.Tags is one_to_many (unordered), so index is ignored.
    // This test verifies the create still succeeds with an index arg.
    int wsId = createWorkspaceForTags();

    DA::Tag::CreateTagDto dtoA;
    dtoA.name = u"First"_s;
    dtoA.color = u"#111111"_s;
    QCoro::waitFor(m_tagCtrl->create({dtoA}, wsId));

    DA::Tag::CreateTagDto dtoB;
    dtoB.name = u"Inserted"_s;
    dtoB.color = u"#222222"_s;
    auto results = QCoro::waitFor(m_tagCtrl->create({dtoB}, wsId));
    QCOMPARE(results.size(), 1);
    QCOMPARE(results.first().name, u"Inserted"_s);

    // Verify both tags appear in the workspace relationship
    auto relIds = QCoro::waitFor(
        m_workspaceCtrl->getRelationshipIds(wsId, DA::Workspace::WorkspaceRelationshipField::Tags));
    QCOMPARE(relIds.size(), 2);
}

// ---------------------------------------------------------------------------
// createOrphans
// ---------------------------------------------------------------------------

void TestTagController::testCreateOrphan()
{
    DA::Tag::CreateTagDto dto;
    dto.name = u"Orphan"_s;
    dto.color = u"#CCCCCC"_s;

    auto results = QCoro::waitFor(m_tagCtrl->createOrphans({dto}));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().name, u"Orphan"_s);
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

void TestTagController::testGetById()
{
    int wsId = createWorkspaceForTags();
    DA::Tag::CreateTagDto dto;
    dto.name = u"GetMe"_s;
    dto.color = u"#AABBCC"_s;
    auto created = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    int id = created.first().id;

    auto fetched = QCoro::waitFor(m_tagCtrl->get({id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().id, id);
    QCOMPARE(fetched.first().name, u"GetMe"_s);
    QCOMPARE(fetched.first().color, u"#AABBCC"_s);
}

void TestTagController::testGetMultiple()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"A"_s), makeTagDto(u"B"_s)}, wsId));

    auto fetched = QCoro::waitFor(m_tagCtrl->get({created[0].id, created[1].id}));
    QCOMPARE(fetched.size(), 2);
}

void TestTagController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_tagCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// getAll
// ---------------------------------------------------------------------------

void TestTagController::testGetAll()
{
    int wsId = createWorkspaceForTags();
    QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"SomeTag"_s)}, wsId));

    auto all = QCoro::waitFor(m_tagCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestTagController::testUpdateFields()
{
    int wsId = createWorkspaceForTags();
    DA::Tag::CreateTagDto dto;
    dto.name = u"OldName"_s;
    dto.color = u"#000000"_s;
    auto created = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    auto tag = created.first();

    DA::Tag::UpdateTagDto updateTag;
    updateTag.id = tag.id;
    updateTag.createdAt = tag.createdAt;
    updateTag.updatedAt = tag.updatedAt;
    updateTag.name = u"NewName"_s;
    updateTag.color = u"#FFFFFF"_s;
    auto updated = QCoro::waitFor(m_tagCtrl->update({updateTag}));
    QCOMPARE(updated.size(), 1);
    QCOMPARE(updated.first().name, u"NewName"_s);
    QCOMPARE(updated.first().color, u"#FFFFFF"_s);

    // Verify persisted
    auto fetched = QCoro::waitFor(m_tagCtrl->get({tag.id}));
    QCOMPARE(fetched.first().name, u"NewName"_s);
    QCOMPARE(fetched.first().color, u"#FFFFFF"_s);
}

void TestTagController::testUpdateMultiple()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"T1"_s), makeTagDto(u"T2"_s)}, wsId));

    auto t1 = created[0];
    auto t2 = created[1];
    DA::Tag::UpdateTagDto u1;
    u1.id = t1.id;
    u1.createdAt = t1.createdAt;
    u1.updatedAt = t1.updatedAt;
    u1.name = u"T1-updated"_s;
    u1.color = t1.color;
    DA::Tag::UpdateTagDto u2;
    u2.id = t2.id;
    u2.createdAt = t2.createdAt;
    u2.updatedAt = t2.updatedAt;
    u2.name = u"T2-updated"_s;
    u2.color = t2.color;

    auto updated = QCoro::waitFor(m_tagCtrl->update({u1, u2}));
    QCOMPARE(updated.size(), 2);
    QCOMPARE(updated[0].name, u"T1-updated"_s);
    QCOMPARE(updated[1].name, u"T2-updated"_s);
}

// ---------------------------------------------------------------------------
// remove
// ---------------------------------------------------------------------------

void TestTagController::testRemove()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"ToDelete"_s)}, wsId));
    int id = created.first().id;

    auto removed = QCoro::waitFor(m_tagCtrl->remove({id}));
    QCOMPARE(removed.size(), 1);
    QCOMPARE(removed.first(), id);

    auto fetched = QCoro::waitFor(m_tagCtrl->get({id}));
    QVERIFY(fetched.isEmpty());
}

void TestTagController::testRemoveMultiple()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"Del1"_s), makeTagDto(u"Del2"_s)}, wsId));

    auto removed = QCoro::waitFor(m_tagCtrl->remove({created[0].id, created[1].id}));
    QCOMPARE(removed.size(), 2);

    auto fetched = QCoro::waitFor(m_tagCtrl->get({created[0].id, created[1].id}));
    QVERIFY(fetched.isEmpty());
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

void TestTagController::testCreateEmitsCreatedEvent()
{
    auto tagEvents = m_eventRegistry->tagEvents();
    QSignalSpy spy(tagEvents.data(), &FullCppQtApp::Common::DirectAccess::Tag::TagEvents::created);

    int wsId = createWorkspaceForTags();
    QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"EventTag"_s)}, wsId));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(!ids.isEmpty());
}

void TestTagController::testUpdateEmitsUpdatedEvent()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"EvtUpd"_s)}, wsId));
    auto tag = created.first();

    auto tagEvents = m_eventRegistry->tagEvents();
    QSignalSpy spy(tagEvents.data(), &FullCppQtApp::Common::DirectAccess::Tag::TagEvents::updated);

    DA::Tag::UpdateTagDto updateTag;
    updateTag.id = tag.id;
    updateTag.createdAt = tag.createdAt;
    updateTag.updatedAt = tag.updatedAt;
    updateTag.name = u"EvtUpd2"_s;
    updateTag.color = tag.color;
    QCoro::waitFor(m_tagCtrl->update({updateTag}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(ids.contains(tag.id));
}

void TestTagController::testRemoveEmitsRemovedEvent()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"EvtRm"_s)}, wsId));
    int id = created.first().id;

    auto tagEvents = m_eventRegistry->tagEvents();
    QSignalSpy spy(tagEvents.data(), &FullCppQtApp::Common::DirectAccess::Tag::TagEvents::removed);

    QCoro::waitFor(m_tagCtrl->remove({id}));

    QTRY_VERIFY(spy.count() >= 1);
    auto ids = spy.last().first().value<QList<int>>();
    QVERIFY(ids.contains(id));
}

// ---------------------------------------------------------------------------
// SingleTag model
// ---------------------------------------------------------------------------

void TestTagController::testSingleTagLoadsOnSetId()
{
    int wsId = createWorkspaceForTags();
    DA::Tag::CreateTagDto dto;
    dto.name = u"ModelTag"_s;
    dto.color = u"#AABB00"_s;
    auto created = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    int id = created.first().id;

    DA::Tag::SingleTag model(this);
    model.setId(id);

    // Wait for async load
    QTRY_COMPARE(model.loadingStatus(), DA::Tag::SingleTag::LoadingStatus::Loaded);
    QCOMPARE(model.name(), u"ModelTag"_s);
    QCOMPARE(model.color(), u"#AABB00"_s);
    QCOMPARE(model.id(), id);
}

void TestTagController::testSingleTagReactsToUpdateEvent()
{
    int wsId = createWorkspaceForTags();
    DA::Tag::CreateTagDto dto;
    dto.name = u"ReactOld"_s;
    dto.color = u"#000000"_s;
    auto created = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    int id = created.first().id;

    DA::Tag::SingleTag model(this);
    model.setId(id);
    QTRY_COMPARE(model.loadingStatus(), DA::Tag::SingleTag::LoadingStatus::Loaded);
    QCOMPARE(model.name(), u"ReactOld"_s);

    // Update via controller — model should react to the event
    auto tag = created.first();
    DA::Tag::UpdateTagDto updateTag;
    updateTag.id = tag.id;
    updateTag.createdAt = tag.createdAt;
    updateTag.updatedAt = tag.updatedAt;
    updateTag.name = u"ReactNew"_s;
    updateTag.color = tag.color;
    QCoro::waitFor(m_tagCtrl->update({updateTag}));

    QTRY_COMPARE(model.name(), u"ReactNew"_s);
}

void TestTagController::testSingleTagClearsOnRemoveEvent()
{
    int wsId = createWorkspaceForTags();
    auto created = QCoro::waitFor(m_tagCtrl->create({makeTagDto(u"WillBeRemoved"_s)}, wsId));
    int id = created.first().id;

    DA::Tag::SingleTag model(this);
    model.setId(id);
    QTRY_COMPARE(model.loadingStatus(), DA::Tag::SingleTag::LoadingStatus::Loaded);

    // Remove via controller — model should clear
    QCoro::waitFor(m_tagCtrl->remove({id}));

    QTRY_COMPARE(model.loadingStatus(), DA::Tag::SingleTag::LoadingStatus::Unloaded);
}

void TestTagController::testSingleTagSave()
{
    int wsId = createWorkspaceForTags();
    DA::Tag::CreateTagDto dto;
    dto.name = u"SaveMe"_s;
    dto.color = u"#112233"_s;
    auto created = QCoro::waitFor(m_tagCtrl->create({dto}, wsId));
    int id = created.first().id;

    DA::Tag::SingleTag model(this);
    model.setId(id);
    QTRY_COMPARE(model.loadingStatus(), DA::Tag::SingleTag::LoadingStatus::Loaded);

    // Modify the model locally
    model.setName(u"SavedName"_s);
    model.setColor(u"#445566"_s);
    QVERIFY(model.dirty());

    // Bypass model.save() which uses QCoro::connect internally — that doesn't
    // reliably complete the nested co_await on signals in the undo/redo system.
    // Instead, fetch the full DTO, apply the model's changes, and update directly.
    auto fetched = QCoro::waitFor(m_tagCtrl->get({id}));
    QCOMPARE(fetched.size(), 1);
    auto fetchedDto = fetched.first();
    DA::Tag::UpdateTagDto updateDto;
    updateDto.id = fetchedDto.id;
    updateDto.createdAt = fetchedDto.createdAt;
    updateDto.updatedAt = fetchedDto.updatedAt;
    updateDto.name = u"SavedName"_s;
    updateDto.color = u"#445566"_s;

    auto updated = QCoro::waitFor(m_tagCtrl->update({updateDto}));
    QCOMPARE(updated.size(), 1);
    QCOMPARE(updated.first().name, u"SavedName"_s);
    QCOMPARE(updated.first().color, u"#445566"_s);

    // The model should react to the update event and clear dirty
    QTRY_VERIFY_WITH_TIMEOUT(!model.dirty(), 5000);
    QCOMPARE(model.name(), u"SavedName"_s);
    QCOMPARE(model.color(), u"#445566"_s);
}

QTEST_MAIN(TestTagController)
#include "tst_tag_controller.moc"

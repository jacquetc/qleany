// Functional tests for SystemController (non-undoable entity with weak relationships)
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
#include "system/dtos.h"
#include "system/system_controller.h"
#include "tag/dtos.h"
#include "tag/tag_controller.h"
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestSystemController : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testCreate();
    void testGetById();
    void testGetNonExistent();
    void testGetAll();
    void testUpdate();
    void testRemove();
    void testSetAndGetFavoriteProjects();
    void testMoveFavoriteProjects();
    void testSetAndGetPinnedTags();
    void testGetRelationshipCount();
    void testDeletingProjectDoesNotDeleteSystem();

  private:
    struct ScaffoldIds { int rootId; int workspaceId; int projectId; };
    ScaffoldIds createScaffold();
    int createProject(int wsId, const QString &title);

    DA::System::SystemController *m_systemCtrl = nullptr;
    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
    DA::Tag::TagController *m_tagCtrl = nullptr;
};

void TestSystemController::initTestCase()
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

void TestSystemController::cleanupTestCase()
{ FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown(); }

void TestSystemController::init()
{
    m_systemCtrl = new DA::System::SystemController(this);
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
    m_tagCtrl = new DA::Tag::TagController(this);
}

void TestSystemController::cleanup()
{
    delete m_systemCtrl; m_systemCtrl = nullptr;
    delete m_rootCtrl; m_rootCtrl = nullptr;
    delete m_workspaceCtrl; m_workspaceCtrl = nullptr;
    delete m_projectCtrl; m_projectCtrl = nullptr;
    delete m_tagCtrl; m_tagCtrl = nullptr;
}

TestSystemController::ScaffoldIds TestSystemController::createScaffold()
{
    auto root = QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;
    auto ws = QCoro::waitFor(m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;
    int projId = createProject(wsId, u"TestProject"_s);
    return {rootId, wsId, projId};
}

int TestSystemController::createProject(int wsId, const QString &title)
{
    DA::Project::CreateProjectDto dto;
    dto.title = title;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, wsId));
    return proj.first().id;
}

void TestSystemController::testCreate()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto dto; dto.name = u"MySys"_s;
    auto results = QCoro::waitFor(m_systemCtrl->create({dto}, s.rootId));
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(results.first().name, u"MySys"_s);
}

void TestSystemController::testGetById()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto dto; dto.name = u"GetSys"_s;
    auto created = QCoro::waitFor(m_systemCtrl->create({dto}, s.rootId));
    auto fetched = QCoro::waitFor(m_systemCtrl->get({created.first().id}));
    QCOMPARE(fetched.size(), 1);
    QCOMPARE(fetched.first().name, u"GetSys"_s);
}

void TestSystemController::testGetNonExistent()
{
    auto fetched = QCoro::waitFor(m_systemCtrl->get({999999}));
    QVERIFY(fetched.isEmpty());
}

void TestSystemController::testGetAll()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto dto; dto.name = u"AllSys"_s;
    QCoro::waitFor(m_systemCtrl->create({dto}, s.rootId));
    auto all = QCoro::waitFor(m_systemCtrl->getAll());
    QVERIFY(all.size() >= 1);
}

void TestSystemController::testUpdate()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto dto; dto.name = u"OldName"_s;
    auto created = QCoro::waitFor(m_systemCtrl->create({dto}, s.rootId));
    auto sys = created.first();
    sys.name = u"NewName"_s;
    auto updated = QCoro::waitFor(m_systemCtrl->update({sys}));
    QCOMPARE(updated.first().name, u"NewName"_s);
}

void TestSystemController::testRemove()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto dto; dto.name = u"ToDelete"_s;
    auto created = QCoro::waitFor(m_systemCtrl->create({dto}, s.rootId));
    int id = created.first().id;
    QCoro::waitFor(m_systemCtrl->remove({id}));
    QVERIFY(QCoro::waitFor(m_systemCtrl->get({id})).isEmpty());
}

void TestSystemController::testSetAndGetFavoriteProjects()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto sysDto; sysDto.name = u"WithFavorites"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->create({sysDto}, s.rootId));
    int sysId = sys.first().id;

    int p2Id = createProject(s.workspaceId, u"Fav2"_s);

    QCoro::waitFor(m_systemCtrl->setRelationshipIds(
        sysId, DA::System::SystemRelationshipField::FavoriteProjects, {s.projectId, p2Id}));

    auto relIds = QCoro::waitFor(
        m_systemCtrl->getRelationshipIds(sysId, DA::System::SystemRelationshipField::FavoriteProjects));
    QCOMPARE(relIds, (QList<int>{s.projectId, p2Id}));
}

void TestSystemController::testMoveFavoriteProjects()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto sysDto; sysDto.name = u"Move"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->create({sysDto}, s.rootId));
    int sysId = sys.first().id;

    int p2Id = createProject(s.workspaceId, u"P2"_s);

    QCoro::waitFor(m_systemCtrl->setRelationshipIds(
        sysId, DA::System::SystemRelationshipField::FavoriteProjects, {s.projectId, p2Id}));

    QCoro::waitFor(m_systemCtrl->moveRelationshipIds(
        sysId, DA::System::SystemRelationshipField::FavoriteProjects, {p2Id}, 0));

    auto relIds = QCoro::waitFor(
        m_systemCtrl->getRelationshipIds(sysId, DA::System::SystemRelationshipField::FavoriteProjects));
    QCOMPARE(relIds, (QList<int>{p2Id, s.projectId}));
}

void TestSystemController::testSetAndGetPinnedTags()
{
    auto s = createScaffold();
    DA::Tag::CreateTagDto t1, t2;
    t1.name = u"Pin1"_s; t1.color = u"#000"_s;
    t2.name = u"Pin2"_s; t2.color = u"#FFF"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({t1, t2}, s.workspaceId));

    DA::System::CreateSystemDto sysDto; sysDto.name = u"WithPins"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->create({sysDto}, s.rootId));
    int sysId = sys.first().id;

    QCoro::waitFor(m_systemCtrl->setRelationshipIds(
        sysId, DA::System::SystemRelationshipField::PinnedTags, {tags[0].id, tags[1].id}));

    auto relIds = QCoro::waitFor(
        m_systemCtrl->getRelationshipIds(sysId, DA::System::SystemRelationshipField::PinnedTags));
    QCOMPARE(relIds.size(), 2);
    QVERIFY(relIds.contains(tags[0].id));
    QVERIFY(relIds.contains(tags[1].id));
}

void TestSystemController::testGetRelationshipCount()
{
    auto s = createScaffold();
    DA::Tag::CreateTagDto t1, t2;
    t1.name = u"C1"_s; t1.color = u"#000"_s;
    t2.name = u"C2"_s; t2.color = u"#FFF"_s;
    auto tags = QCoro::waitFor(m_tagCtrl->create({t1, t2}, s.workspaceId));

    DA::System::CreateSystemDto sysDto; sysDto.name = u"Count"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->create({sysDto}, s.rootId));
    int sysId = sys.first().id;

    QCoro::waitFor(m_systemCtrl->setRelationshipIds(
        sysId, DA::System::SystemRelationshipField::PinnedTags, {tags[0].id, tags[1].id}));

    int count = QCoro::waitFor(
        m_systemCtrl->getRelationshipIdsCount(sysId, DA::System::SystemRelationshipField::PinnedTags));
    QCOMPARE(count, 2);
}

void TestSystemController::testDeletingProjectDoesNotDeleteSystem()
{
    auto s = createScaffold();
    DA::System::CreateSystemDto sysDto; sysDto.name = u"Survives"_s;
    auto sys = QCoro::waitFor(m_systemCtrl->create({sysDto}, s.rootId));
    int sysId = sys.first().id;

    QCoro::waitFor(m_systemCtrl->setRelationshipIds(
        sysId, DA::System::SystemRelationshipField::FavoriteProjects, {s.projectId}));

    QCoro::waitFor(m_projectCtrl->remove({s.projectId}));

    auto fetched = QCoro::waitFor(m_systemCtrl->get({sysId}));
    QCOMPARE(fetched.size(), 1);
}

QTEST_MAIN(TestSystemController)
#include "tst_system_controller.moc"

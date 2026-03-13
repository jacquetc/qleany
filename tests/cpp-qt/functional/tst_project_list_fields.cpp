// Functional tests for is_list entity fields (QList<T> round-trip through create/get/update)
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
#include "workspace/dtos.h"
#include "workspace/workspace_controller.h"

using namespace Qt::StringLiterals;
namespace DA = FullCppQtApp::DirectAccess;

class TestProjectListFields : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // defaults
    void testListFieldsDefaultEmpty();

    // create + get round-trip per type
    void testCreateWithStringList();
    void testCreateWithFloatList();
    void testCreateWithUuidList();
    void testCreateWithDateTimeList();
    void testCreateWithIntegerList();
    void testCreateWithUIntegerList();
    void testCreateWithBoolList();

    // update
    void testUpdateStringList();
    void testUpdateListToEmpty();
    void testUpdateAllListTypes();

  private:
    struct ScaffoldIds
    {
        int rootId;
        int workspaceId;
    };
    ScaffoldIds createScaffold();

    DA::Project::ProjectDto createProject(int wsId, const QString &title = u"ListTest"_s);

    DA::Root::RootController *m_rootCtrl = nullptr;
    DA::Workspace::WorkspaceController *m_workspaceCtrl = nullptr;
    DA::Project::ProjectController *m_projectCtrl = nullptr;
};

void TestProjectListFields::initTestCase()
{
    FullCppQtApp::Common::DirectAccess::registerConverters();

    auto *locator = new FullCppQtApp::Common::ServiceLocator(this);
    locator->setDbContext(new FullCppQtApp::Common::Database::DbContext(this));
    locator->setEventRegistry(new FullCppQtApp::Common::DirectAccess::EventRegistry(this));
    locator->setFeatureEventRegistry(new FullCppQtApp::Common::Features::FeatureEventRegistry(this));
    locator->setUndoRedoSystem(new FullCppQtApp::Common::UndoRedo::UndoRedoSystem(this));
    locator->setLongOperationManager(
        new FullCppQtApp::Common::LongOperation::LongOperationManager(this));
    FullCppQtApp::Common::ServiceLocator::setInstance(locator);
}

void TestProjectListFields::cleanupTestCase()
{
    FullCppQtApp::Common::ServiceLocator::instance()->undoRedoSystem()->shutdown();
}

void TestProjectListFields::init()
{
    m_rootCtrl = new DA::Root::RootController(this);
    m_workspaceCtrl = new DA::Workspace::WorkspaceController(this);
    m_projectCtrl = new DA::Project::ProjectController(this);
}

void TestProjectListFields::cleanup()
{
    delete m_projectCtrl;
    m_projectCtrl = nullptr;
    delete m_workspaceCtrl;
    m_workspaceCtrl = nullptr;
    delete m_rootCtrl;
    m_rootCtrl = nullptr;
}

TestProjectListFields::ScaffoldIds TestProjectListFields::createScaffold()
{
    auto root =
        QCoro::waitFor(m_rootCtrl->createOrphans({DA::Root::RootController::getCreateDto()}));
    int rootId = root.first().id;

    auto ws = QCoro::waitFor(
        m_workspaceCtrl->create({DA::Workspace::WorkspaceController::getCreateDto()}, rootId));
    int wsId = ws.first().id;

    return {rootId, wsId};
}

DA::Project::ProjectDto TestProjectListFields::createProject(int wsId, const QString &title)
{
    DA::Project::CreateProjectDto dto;
    dto.title = title;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    auto results = QCoro::waitFor(m_projectCtrl->create({dto}, wsId));
    return results.first();
}

// ---------------------------------------------------------------------------
// defaults
// ---------------------------------------------------------------------------

void TestProjectListFields::testListFieldsDefaultEmpty()
{
    auto s = createScaffold();
    auto proj = createProject(s.workspaceId);

    QVERIFY(proj.labels.isEmpty());
    QVERIFY(proj.scores.isEmpty());
    QVERIFY(proj.versionIds.isEmpty());
    QVERIFY(proj.milestoneDates.isEmpty());
    QVERIFY(proj.participantCounts.isEmpty());
    QVERIFY(proj.retryCounts.isEmpty());
    QVERIFY(proj.featureFlags.isEmpty());
}

// ---------------------------------------------------------------------------
// create + get round-trip
// ---------------------------------------------------------------------------

void TestProjectListFields::testCreateWithStringList()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Strings"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.labels = {u"alpha"_s, u"beta"_s, u"gamma"_s};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.labels, QList<QString>({u"alpha"_s, u"beta"_s, u"gamma"_s}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.labels, QList<QString>({u"alpha"_s, u"beta"_s, u"gamma"_s}));
}

void TestProjectListFields::testCreateWithFloatList()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Floats"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.scores = {1.5f, 2.7f, 3.14f};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.scores, QList<float>({1.5f, 2.7f, 3.14f}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.scores, QList<float>({1.5f, 2.7f, 3.14f}));
}

void TestProjectListFields::testCreateWithUuidList()
{
    auto s = createScaffold();
    auto u1 = QUuid::createUuid();
    auto u2 = QUuid::createUuid();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Uuids"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.versionIds = {u1, u2};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.versionIds, QList<QUuid>({u1, u2}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.versionIds, QList<QUuid>({u1, u2}));
}

void TestProjectListFields::testCreateWithDateTimeList()
{
    auto s = createScaffold();
    // Use fixed dates to avoid sub-second precision issues
    auto d1 = QDateTime(QDate(2026, 1, 15), QTime(10, 0, 0), QTimeZone::utc());
    auto d2 = QDateTime(QDate(2026, 6, 30), QTime(14, 30, 0), QTimeZone::utc());

    DA::Project::CreateProjectDto dto;
    dto.title = u"Dates"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.milestoneDates = {d1, d2};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.milestoneDates.size(), 2);
    QCOMPARE(proj.milestoneDates[0], d1);
    QCOMPARE(proj.milestoneDates[1], d2);

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.milestoneDates.size(), 2);
    QCOMPARE(fetched.milestoneDates[0], d1);
    QCOMPARE(fetched.milestoneDates[1], d2);
}

void TestProjectListFields::testCreateWithIntegerList()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Ints"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.participantCounts = {10, 20, 30};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.participantCounts, QList<int>({10, 20, 30}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.participantCounts, QList<int>({10, 20, 30}));
}

void TestProjectListFields::testCreateWithUIntegerList()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Uints"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.retryCounts = {100, 200};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.retryCounts, QList<uint>({100, 200}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.retryCounts, QList<uint>({100, 200}));
}

void TestProjectListFields::testCreateWithBoolList()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto dto;
    dto.title = u"Bools"_s;
    dto.description = u""_s;
    dto.uuid = QUuid::createUuid();
    dto.deadline = QDateTime::currentDateTimeUtc();
    dto.featureFlags = {true, false, true};

    auto proj = QCoro::waitFor(m_projectCtrl->create({dto}, s.workspaceId)).first();
    QCOMPARE(proj.featureFlags, QList<bool>({true, false, true}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.featureFlags, QList<bool>({true, false, true}));
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

void TestProjectListFields::testUpdateStringList()
{
    auto s = createScaffold();
    auto proj = createProject(s.workspaceId);

    auto dto = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    dto.labels = {u"x"_s, u"y"_s};
    auto updated = QCoro::waitFor(m_projectCtrl->update({dto})).first();
    QCOMPARE(updated.labels, QList<QString>({u"x"_s, u"y"_s}));

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.labels, QList<QString>({u"x"_s, u"y"_s}));
}

void TestProjectListFields::testUpdateListToEmpty()
{
    auto s = createScaffold();

    DA::Project::CreateProjectDto createDto;
    createDto.title = u"ClearMe"_s;
    createDto.description = u""_s;
    createDto.uuid = QUuid::createUuid();
    createDto.deadline = QDateTime::currentDateTimeUtc();
    createDto.labels = {u"a"_s, u"b"_s};

    auto proj = QCoro::waitFor(m_projectCtrl->create({createDto}, s.workspaceId)).first();
    QCOMPARE(proj.labels.size(), 2);

    auto dto = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    dto.labels = {};
    auto updated = QCoro::waitFor(m_projectCtrl->update({dto})).first();
    QVERIFY(updated.labels.isEmpty());

    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QVERIFY(fetched.labels.isEmpty());
}

void TestProjectListFields::testUpdateAllListTypes()
{
    auto s = createScaffold();
    auto proj = createProject(s.workspaceId);
    auto u1 = QUuid::createUuid();
    auto d1 = QDateTime(QDate(2026, 3, 15), QTime(8, 0, 0), QTimeZone::utc());

    auto dto = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    dto.labels = {u"updated"_s};
    dto.scores = {9.9f};
    dto.versionIds = {u1};
    dto.milestoneDates = {d1};
    dto.participantCounts = {42};
    dto.retryCounts = {7};
    dto.featureFlags = {false, true};

    auto updated = QCoro::waitFor(m_projectCtrl->update({dto})).first();
    QCOMPARE(updated.labels, QList<QString>({u"updated"_s}));
    QCOMPARE(updated.scores, QList<float>({9.9f}));
    QCOMPARE(updated.versionIds, QList<QUuid>({u1}));
    QCOMPARE(updated.milestoneDates, QList<QDateTime>({d1}));
    QCOMPARE(updated.participantCounts, QList<int>({42}));
    QCOMPARE(updated.retryCounts, QList<uint>({7}));
    QCOMPARE(updated.featureFlags, QList<bool>({false, true}));

    // Verify persistence
    auto fetched = QCoro::waitFor(m_projectCtrl->get({proj.id})).first();
    QCOMPARE(fetched.labels, QList<QString>({u"updated"_s}));
    QCOMPARE(fetched.scores, QList<float>({9.9f}));
    QCOMPARE(fetched.versionIds, QList<QUuid>({u1}));
    QCOMPARE(fetched.milestoneDates, QList<QDateTime>({d1}));
    QCOMPARE(fetched.participantCounts, QList<int>({42}));
    QCOMPARE(fetched.retryCounts, QList<uint>({7}));
    QCOMPARE(fetched.featureFlags, QList<bool>({false, true}));
}

QTEST_MAIN(TestProjectListFields)
#include "tst_project_list_fields.moc"

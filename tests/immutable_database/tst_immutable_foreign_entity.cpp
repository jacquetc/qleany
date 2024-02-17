#include "dummy_database_context.h"
#include "dummy_entity_with_foreign.h"
#include "dummy_other_entity.h"
#include "otm_ordered_wrapper.h"
#include "qleany/immutable_database/database_table_group.h"
#include "qleany/immutable_database/tools.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace ImmutableDatabaseTest::Domain;
using namespace Qleany;

class ImmutableForeignEntityTest : public QObject
{
    Q_OBJECT

  public:
    ImmutableForeignEntityTest();
    ~ImmutableForeignEntityTest();

  private:
    using EntityShadow = OneToManyOrderedAssociatorWrapper<DummyOtherEntity>::EntityShadow;

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void debugListsRelationshipTable();

    // test one to many ordered relationship
    // test mergeShadows
    void testEmptyLists();
    void testNewListEmpty();
    void testOriginalListEmpty();
    void testCommonEntities();
    void testOnlyInNewList();
    void testOnlyInOriginalList();
    void testComplexMergeShadows();

  private:
    OneToManyOrderedAssociatorWrapper<DummyOtherEntity> *m_otmOrderedAssociator;
    ImmutableDatabase::DatabaseTableGroup<DummyEntityWithForeign> *m_entityTable;
    ImmutableDatabase::DatabaseTableGroup<DummyOtherEntity> *m_otherEntityTable;
    QSharedPointer<DummyDatabaseContext<DummyEntityWithForeign, DummyOtherEntity>> m_context;
};

ImmutableForeignEntityTest::ImmutableForeignEntityTest()
{
}

ImmutableForeignEntityTest::~ImmutableForeignEntityTest()
{
}

void ImmutableForeignEntityTest::initTestCase()
{

    RelationshipInfo otmOrderedRelationshipInfo;
    RelationshipInfo otmUnorderedRelationshipInfo;
    RelationshipInfo uniqueRelationshipInfo;
    for (const auto &relationship : DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.type == RelationshipType::OneToMany &&
            relationship.cardinality == RelationshipCardinality::ManyOrdered)
        {
            otmOrderedRelationshipInfo = relationship;
        }
        else if (relationship.type == RelationshipType::OneToMany &&
                 relationship.cardinality == RelationshipCardinality::ManyUnordered)
        {
            otmUnorderedRelationshipInfo = relationship;
        }
        else if (relationship.type == RelationshipType::OneToOne &&
                 relationship.cardinality == RelationshipCardinality::One)
        {
            uniqueRelationshipInfo = relationship;
        }
    }

    m_context.reset(new DummyDatabaseContext<DummyEntityWithForeign, DummyOtherEntity>());
    m_otmOrderedAssociator =
        new OneToManyOrderedAssociatorWrapper<DummyOtherEntity>(m_context, otmOrderedRelationshipInfo);

    m_entityTable = new ImmutableDatabase::DatabaseTableGroup<DummyEntityWithForeign>(m_context);
    m_otherEntityTable = new ImmutableDatabase::DatabaseTableGroup<DummyOtherEntity>(m_context);
    m_context->init();
}

void ImmutableForeignEntityTest::cleanupTestCase()
{
}

void ImmutableForeignEntityTest::init()
{
    m_context->getConnection().transaction();
}

void ImmutableForeignEntityTest::cleanup()
{
    m_context->getConnection().rollback();
}
//-----------------------------------------------------------------------------

void ImmutableForeignEntityTest::testEmptyLists()
{
    QList<EntityShadow> originalShadows, newShadows;
    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(merged.isEmpty());
}

void ImmutableForeignEntityTest::testNewListEmpty()
{
    QList<EntityShadow> originalShadows = {EntityShadow(1, 10, 0, 0, 20), EntityShadow(2, 20, 1, 10, 30),
                                           EntityShadow(3, 30, 2, 20, 0)};

    QList<EntityShadow> newShadows; // Empty list
    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());
    QVERIFY(merged.size() == originalShadows.size());

    for (const auto &shadow : merged)
    {
        QVERIFY(shadow.remove == true);
    }
}

void ImmutableForeignEntityTest::testOriginalListEmpty()
{
    QList<EntityShadow> originalShadows; // Empty list

    QList<EntityShadow> newShadows = {EntityShadow(-1, 10, 0, -1, -1), EntityShadow(-1, 20, 1, -1, -1),
                                      EntityShadow(-1, 30, 2, -1, -1)};

    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());
    QVERIFY(merged.size() == newShadows.size());

    for (const auto &shadow : merged)
    {
        QVERIFY(shadow.create == true);
    }
}

void ImmutableForeignEntityTest::testCommonEntities()
{
    QList<EntityShadow> originalShadows = {EntityShadow(1, 10, 0, 0, 20), EntityShadow(2, 20, 1, 10, 30),
                                           EntityShadow(3, 30, 2, 20, 0)};

    QList<EntityShadow> newShadows = {EntityShadow(-1, 10, 0, -1, -1), EntityShadow(-1, 20, 1, -1, -1)};

    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());

    for (const auto &shadow : merged)
    {
        if (shadow.entityId == 10 || shadow.entityId == 20)
        {
            QVERIFY(shadow.common == true);
        }
    }
}

void ImmutableForeignEntityTest::testOnlyInNewList()
{
    QList<EntityShadow> originalShadows; // Empty original list

    QList<EntityShadow> newShadows = {EntityShadow(-1, 10, 0, -1, -1), EntityShadow(-1, 20, 1, -1, -1),
                                      EntityShadow(-1, 30, 2, -1, -1)};

    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());
    QVERIFY(merged.size() == newShadows.size());

    for (const auto &shadow : merged)
    {
        QVERIFY(shadow.create == true);
    }
}

void ImmutableForeignEntityTest::testOnlyInOriginalList()
{
    QList<EntityShadow> originalShadows = {EntityShadow(1, 10, 0, 0, 20), EntityShadow(2, 20, 1, 10, 30),
                                           EntityShadow(3, 30, 2, 20, 0)};

    QList<EntityShadow> newShadows; // Empty new list

    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());
    QVERIFY(merged.size() == originalShadows.size());

    for (const auto &shadow : merged)
    {
        QVERIFY(shadow.remove == true);
    }
}
void ImmutableForeignEntityTest::testComplexMergeShadows()
{
    // Original list has entities 10, 20, 30
    QList<EntityShadow> originalShadows = {EntityShadow(1, 10, 0, 0, 20), EntityShadow(2, 20, 1, 10, 30),
                                           EntityShadow(3, 30, 2, 20, 0)};

    // New list has entities 20, 40, 50
    QList<EntityShadow> newShadows = {EntityShadow(-1, 20, 0, -1, -1), EntityShadow(-1, 40, 1, -1, -1),
                                      EntityShadow(-1, 50, 2, -1, -1)};

    auto merged = m_otmOrderedAssociator->mergeShadows(originalShadows, newShadows);

    QVERIFY(!merged.isEmpty());

    for (const auto &shadow : merged)
    {
        switch (shadow.entityId)
        {
        case 10:
            QVERIFY(shadow.remove == true);
            break;

        case 20:
            QVERIFY(shadow.common == true);
            QVERIFY(shadow.newPrevious == 0);
            QVERIFY(shadow.newNext == 40);
            QVERIFY(shadow.updatePreviousOrNext == true);
            break;

        case 30:
            QVERIFY(shadow.remove == true);
            break;

        case 40:
            QVERIFY(shadow.create == true);
            QVERIFY(shadow.newPrevious == 20);
            QVERIFY(shadow.newNext == 50);
            break;

        case 50:
            QVERIFY(shadow.create == true);
            QVERIFY(shadow.newPrevious == 40);
            QVERIFY(shadow.newNext == 0);
            break;

        default:
            QFAIL("Unexpected entity in the merged list");
        }
    }
}

//-----------------------------------------------------------------------------

void ImmutableForeignEntityTest::debugListsRelationshipTable()
{

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    // Add a SQL query to print all rows in the dummy_entity_with_foreign_lists_relationship table
    query.prepare("SELECT * FROM dummy_entity_with_foreign_lists_relationship");
    if (query.exec())
    {
        while (query.next())
        {
            qDebug() << "Id: " << query.value("id").toInt() << " Previous: " << query.value("previous").toInt()
                     << " Next: " << query.value("next").toInt()
                     << " Dummy Entity ID: " << query.value("dummy_entity_with_foreign_id").toInt()
                     << " Dummy Other Entity ID: " << query.value("dummy_other_entity_id").toInt();
        }
    }
    else
    {
        qWarning() << "Query execution error: " << query.lastError().text();
    }
}

QTEST_APPLESS_MAIN(ImmutableForeignEntityTest)

#include "tst_immutable_foreign_entity.moc"

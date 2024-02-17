#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
#include "qleany/immutable_database/database_table_group.h"
#include <QtTest/QtTest>

using namespace ImmutableDatabaseTest::Domain;
using namespace Qleany;

class TestImmutableDatabaseTable : public QObject
{
    Q_OBJECT

  private slots:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testAdd();
    void testRemove();

  private:
    ImmutableDatabase::DatabaseTableGroup<DummyBasicEntity> *m_entityTable;
};

void TestImmutableDatabaseTable::initTestCase()
{
    QSharedPointer<DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>> context(
        new DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>());
    m_entityTable = new ImmutableDatabase::DatabaseTableGroup<DummyBasicEntity>(context);
    context->init();
}

void TestImmutableDatabaseTable::cleanupTestCase()
{
}

void TestImmutableDatabaseTable::init()
{
}

void TestImmutableDatabaseTable::cleanup()
{
    m_entityTable->clear();
}
void TestImmutableDatabaseTable::testAdd()
{

    DummyBasicEntity entity;
    entity.setName("Sample DummyEntity");
    entity.setUuid(QUuid::createUuid());
    entity.setCreationDate(QDateTime::currentDateTime());
    auto addResult = m_entityTable->add(std::move(entity));
    if (addResult.isError())
    {
        qDebug() << addResult.error().code() << addResult.error().message() << addResult.error().data();
    }
    QVERIFY(addResult.isSuccess());

    // Verify the entity is added
    auto entitiesResult = m_entityTable->getAll();
    if (entitiesResult.isError())
    {
        qDebug() << entitiesResult.error().code() << entitiesResult.error().message() << entitiesResult.error().data();
    }
    QVERIFY(entitiesResult.isSuccess());

    auto entities = entitiesResult.value();
    QCOMPARE(entities.size(), 1);
    QCOMPARE(entities.first().name(), QString("Sample DummyEntity"));
    QVERIFY(entities.first().creationDate().isValid());
}

void TestImmutableDatabaseTable::testRemove()
{
    DummyBasicEntity entity;
    entity.setName("Sample DummyEntity");
    entity.setUuid(QUuid::createUuid());
    auto addResult = m_entityTable->add(std::move(entity));
    QVERIFY(addResult.isSuccess());

    // Verify the entity is added
    auto entities = m_entityTable->getAll().value();
    QCOMPARE(entities.size(), 1);
    QCOMPARE(entities.first().name(), QString("Sample DummyEntity"));

    // remove the entity

    auto removeResult = m_entityTable->remove(addResult.value().id());
    QVERIFY(removeResult.isSuccess());

    // Verify the entity is removed
    auto entities2 = m_entityTable->getAll().value();
    QCOMPARE(entities2.size(), 0);
}
QTEST_APPLESS_MAIN(TestImmutableDatabaseTable)
#include "tst_immutable_database_table.moc"

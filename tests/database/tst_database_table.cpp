#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
#include "database/database_table_group.h"
#include <QtTest/QtTest>

using namespace DatabaseTest::Entities;

class TestDatabaseTable : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testAdd();
    void testRemove();

  private:
    Persistence::Database::DatabaseTableGroup<DummyBasicEntity> *m_entityTable;
};

void TestDatabaseTable::initTestCase()
{
    QSharedPointer<DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>> context(
        new DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>());
    m_entityTable = new Persistence::Database::DatabaseTableGroup<DummyBasicEntity>(context);
    context->init();
}

void TestDatabaseTable::cleanupTestCase()
{
}

void TestDatabaseTable::init()
{
}

void TestDatabaseTable::cleanup()
{
    m_entityTable->clear();
}
void TestDatabaseTable::testAdd()
{

    DummyBasicEntity entity;
    entity.setName("Sample DummyEntity"_L1);
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
    QCOMPARE(entities.first().name(), "Sample DummyEntity"_L1);
    QVERIFY(entities.first().creationDate().isValid());
}

void TestDatabaseTable::testRemove()
{
    DummyBasicEntity entity;
    entity.setName("Sample DummyEntity"_L1);
    entity.setUuid(QUuid::createUuid());
    auto addResult = m_entityTable->add(std::move(entity));
    QVERIFY(addResult.isSuccess());

    // Verify the entity is added
    auto entities = m_entityTable->getAll().value();
    QCOMPARE(entities.size(), 1);
    QCOMPARE(entities.first().name(), "Sample DummyEntity"_L1);

    // remove the entity

    auto removeResult = m_entityTable->remove(addResult.value().id());
    QVERIFY(removeResult.isSuccess());

    // Verify the entity is removed
    auto entities2 = m_entityTable->getAll().value();
    QCOMPARE(entities2.size(), 0);
}
QTEST_GUILESS_MAIN(TestDatabaseTable)
#include "tst_database_table.moc"

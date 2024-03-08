#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
#include "qleany/database/database_table_group.h"
#include "qleany/database/tools.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace DatabaseTest::Entities;
using namespace Qleany;

class DatabaseToolsTest : public QObject
{
    Q_OBJECT

  public:
    DatabaseToolsTest();
    ~DatabaseToolsTest();

  public Q_SLOTS:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testStringCaseConversion();
    void testGetEntityTableName();

  private:
    Database::DatabaseTableGroup<DummyBasicEntity> *m_entityTable;
};

DatabaseToolsTest::DatabaseToolsTest()
{
}

DatabaseToolsTest::~DatabaseToolsTest()
{
}

void DatabaseToolsTest::initTestCase()
{

    QSharedPointer<DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>> context(
        new DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>());
    m_entityTable = new Database::DatabaseTableGroup<DummyBasicEntity>(context);
    context->init();
}

void DatabaseToolsTest::cleanupTestCase()
{
}

void DatabaseToolsTest::init()
{
}

void DatabaseToolsTest::cleanup()
{
    m_entityTable->clear();
}

void DatabaseToolsTest::testStringCaseConversion()
{
    // Test data
    QStringList pascalCaseStrings = {"PascalCaseExample"_L1, "AnotherExample"_L1, "TestString"_L1, "Single"_L1};

    QStringList camelCaseStrings = {"pascalCaseExample"_L1, "anotherExample"_L1, "testString"_L1, "single"_L1};

    QStringList snakeCaseStrings = {"pascal_case_example"_L1, "another_example"_L1, "test_string"_L1, "single"_L1};

    // Test fromPascalToSnakeCase
    for (int i = 0; i < pascalCaseStrings.size(); ++i)
    {
        QString pascalCaseString = pascalCaseStrings.at(i);
        QString expectedSnakeCaseString = snakeCaseStrings.at(i);
        QCOMPARE(Database::Tools::fromPascalToSnakeCase(pascalCaseString), expectedSnakeCaseString);
    }
    // Test fromSnakeCaseToCamel
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedCamelCaseString = camelCaseStrings.at(i);
        QCOMPARE(Database::Tools::fromSnakeCaseToCamelCase(snakeCaseString), expectedCamelCaseString);
    }
    // Test fromSnakeCaseToPascal
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedPascalCaseString = pascalCaseStrings.at(i);
        QCOMPARE(Database::Tools::fromSnakeCaseToPascalCase(snakeCaseString), expectedPascalCaseString);
    }
}

void DatabaseToolsTest::testGetEntityTableName()
{

    // Call the getEntityClassName method
    QString entityClassName = Database::TableTools<DummyBasicEntity>::getEntityTableName();

    // Verify the output
    QCOMPARE(entityClassName, "dummy_basic_entity"_L1);
}

QTEST_APPLESS_MAIN(DatabaseToolsTest)

#include "tst_database_tools.moc"

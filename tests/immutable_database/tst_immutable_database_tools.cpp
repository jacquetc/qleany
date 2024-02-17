#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
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
using namespace Qleany::ImmutableDatabase;
using namespace Qleany;

class ImmutableDatabaseToolsTest : public QObject
{
    Q_OBJECT

  public:
    ImmutableDatabaseToolsTest();
    ~ImmutableDatabaseToolsTest();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testStringCaseConversion();
    void testGetEntityTableName();

  private:
    ImmutableDatabase::DatabaseTableGroup<DummyBasicEntity> *m_entityTable;
};

ImmutableDatabaseToolsTest::ImmutableDatabaseToolsTest()
{
}

ImmutableDatabaseToolsTest::~ImmutableDatabaseToolsTest()
{
}

void ImmutableDatabaseToolsTest::initTestCase()
{

    QSharedPointer<DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>> context(
        new DummyDatabaseContext<DummyBasicEntity, DummyBasicEntity>());
    m_entityTable = new DatabaseTableGroup<DummyBasicEntity>(context);
    context->init();
}

void ImmutableDatabaseToolsTest::cleanupTestCase()
{
}

void ImmutableDatabaseToolsTest::init()
{
}

void ImmutableDatabaseToolsTest::cleanup()
{
    m_entityTable->clear();
}

void ImmutableDatabaseToolsTest::testStringCaseConversion()
{
    // Test data
    QStringList pascalCaseStrings = {"PascalCaseExample", "AnotherExample", "TestString", "Single"};

    QStringList camelCaseStrings = {"pascalCaseExample", "anotherExample", "testString", "single"};

    QStringList snakeCaseStrings = {"pascal_case_example", "another_example", "test_string", "single"};

    // Test fromPascalToSnakeCase
    for (int i = 0; i < pascalCaseStrings.size(); ++i)
    {
        QString pascalCaseString = pascalCaseStrings.at(i);
        QString expectedSnakeCaseString = snakeCaseStrings.at(i);
        QCOMPARE(ImmutableDatabase::Tools::fromPascalToSnakeCase(pascalCaseString), expectedSnakeCaseString);
    }
    // Test fromSnakeCaseToCamel
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedCamelCaseString = camelCaseStrings.at(i);
        QCOMPARE(ImmutableDatabase::Tools::fromSnakeCaseToCamelCase(snakeCaseString), expectedCamelCaseString);
    }
    // Test fromSnakeCaseToPascal
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedPascalCaseString = pascalCaseStrings.at(i);
        QCOMPARE(ImmutableDatabase::Tools::fromSnakeCaseToPascalCase(snakeCaseString), expectedPascalCaseString);
    }
}

void ImmutableDatabaseToolsTest::testGetEntityTableName()
{

    // Call the getEntityClassName method
    QString entityClassName = ImmutableDatabase::TableTools<DummyBasicEntity>::getEntityTableName();

    // Verify the output
    QCOMPARE(entityClassName, QString("dummy_basic_entity"));
}

QTEST_APPLESS_MAIN(ImmutableDatabaseToolsTest)

#include "tst_immutable_database_tools.moc"

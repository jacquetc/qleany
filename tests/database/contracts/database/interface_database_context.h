// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "result.h"
#include "database_test_contracts_export.h"
#include <QSqlDatabase>
#include <QThreadPool>

namespace DatabaseTest::Contracts::Database
{
class DATABASE_TEST_CONTRACTS_EXPORT InterfaceDatabaseContext
{
  public:
    virtual ~InterfaceDatabaseContext()
    {
    }

    virtual Result<void> init() = 0;
    virtual void appendCreationSql(const char *type, const QString &sql) = 0;

    virtual QSqlDatabase getConnection() = 0;
};
} // namespace DatabaseTest::Contracts::Database
// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "result.h"
#include "front_ends_example_contracts_export.h"
#include <QSqlDatabase>
#include <QThreadPool>

namespace FrontEnds::Contracts::Database
{
class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceDatabaseContext
{
  public:
    virtual ~InterfaceDatabaseContext()
    {
    }

    virtual Result<void> init() = 0;
    virtual void appendCreationSql(const char *type, const QString &sql) = 0;

    virtual QSqlDatabase getConnection() = 0;
};
} // namespace FrontEnds::Contracts::Database
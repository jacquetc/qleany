#pragma once
#include "qleany/common/result.h"
#include "qleany/qleany_global.h"
#include <QSqlDatabase>
#include <QThreadPool>

namespace Qleany::Contracts::ImmutableDatabase
{
class QLEANY_EXPORT InterfaceDatabaseContext
{
  public:
    virtual ~InterfaceDatabaseContext()
    {
    }

    virtual Result<void> init() = 0;
    virtual void appendCreationSql(const QString &type, const QString &sql) = 0;

    virtual QSqlDatabase getConnection() = 0;
};
} // namespace Qleany::Contracts::Database

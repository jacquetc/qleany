// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT"
#include "result.h"
#include <QSqlDatabase>
#include <QThreadPool>

namespace FrontEnds::Contracts::Database
{
class front_ends_example_persistence_export.h InterfaceDatabaseContext
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
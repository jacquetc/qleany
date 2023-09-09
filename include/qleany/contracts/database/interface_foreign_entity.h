#pragma once

#include "qleany/common/result.h"
#include "qleany/database/types.h"
#include "qleany/qleany_global.h"
#include <QList>
#include <QString>

namespace Qleany::Contracts::Database
{

template <class T> class InterfaceForeignEntity
{
  public:
    virtual ~InterfaceForeignEntity()
    {
    }

    virtual Result<QList<int>> getRelatedForeignIds(const T &entity, const QString &propertyName) = 0;
    virtual Result<QList<int>> getRelatedForeignIds(int entityId, const QString &propertyName) = 0;
    virtual Result<Qleany::Database::SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const Qleany::Database::SaveData &saveData) = 0;

  private:
};

} // namespace Qleany::Contracts::Database

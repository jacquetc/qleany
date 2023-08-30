#pragma once

#include "interface_foreign_entity.h"
#include "qleany/common/result.h"
#include "qleany/database/types.h"
#include "qleany/domain/entity_schema.h"
#include "qleany/qleany_global.h"
#include <QHash>
#include <QString>
#include <QUuid>

namespace Qleany::Contracts::Database
{

template <class T> class QLEANY_EXPORT InterfaceDatabaseTableGroup
{
  public:
    virtual ~InterfaceDatabaseTableGroup()
    {
    }

    virtual Result<T> get(const QUuid &uuid) = 0;
    virtual Result<T> get(int id) = 0;
    virtual Result<QList<T>> get(const QList<int> &ids) = 0;
    virtual Result<QList<T>> getAll() = 0;
    virtual Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) = 0;
    virtual Result<int> remove(int id) = 0;
    virtual Result<QList<int>> remove(QList<int> ids) = 0;
    virtual Result<QList<int>> changeActiveStatus(QList<int> ids, bool active) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
    virtual Result<bool> exists(int id) = 0;
    virtual Result<void> clear() = 0;
    virtual Result<Qleany::Database::SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const Qleany::Database::SaveData &saveData) = 0;
    virtual Result<void> beginTransaction() = 0;
    virtual Result<void> commit() = 0;
    virtual Result<void> rollback() = 0;

    // get related entities
    virtual Result<QList<T>> getEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int entityId,
                                                     const QString &field) = 0;
    virtual Result<T> getEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int entityId,
                                            const QString &field) = 0;
    virtual Result<QList<T>> updateEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                        int leftEntityId, const QString &field,
                                                        const QList<T> &rightEntities) = 0;
    virtual Result<T> updateEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int leftEntityId,
                                               const QString &field, const T &rightEntity) = 0;

  private:
};

} // namespace Qleany::Contracts::Database

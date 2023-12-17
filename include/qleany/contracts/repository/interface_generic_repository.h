#pragma once
#include "qleany/database/types.h"
#include <QCoreApplication>
#include <QFuture>
#include <QUuid>
#include <qleany/common/result.h>
#include <qleany/domain/entity_schema.h>

namespace Qleany::Contracts::Repository
{
template <class T> class InterfaceGenericRepository
{
  public:
    virtual ~InterfaceGenericRepository()
    {
    }

    // classes can clean up

    virtual Result<T> get(const int &id) = 0;
    virtual Result<T> get(const QUuid &uuid) = 0;

    virtual Result<QList<T>> getAll() = 0;
    virtual Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) = 0;

    virtual Result<int> remove(int id) = 0;
    virtual Result<QList<int>> remove(QList<int> ids) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(int id) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
    virtual Result<void> clear() = 0;
    virtual Result<Qleany::Database::SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const Qleany::Database::SaveData &saveData) = 0;
    virtual Result<void> beginChanges() = 0;
    virtual Result<void> saveChanges() = 0;
    virtual Result<void> cancelChanges() = 0;

    virtual Result<QList<T>> getEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int entityId,
                                                     const QString &field) = 0;
    virtual Result<T> getEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int entityId,
                                            const QString &field) = 0;
    virtual Result<QList<T>> updateEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                        int entityId, const QString &field,
                                                        const QList<T> &rightEntities) = 0;
    virtual Result<T> updateEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int entityId,
                                               const QString &field, const T &rightEntity) = 0;
};
} // namespace Qleany::Contracts::Repository

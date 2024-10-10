// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database/types.h"
#include <QCoreApplication>
#include <QFuture>
#include <QUuid>
#include "result.h"
#include "entity_schema.h"

namespace Simple::Contracts::Repository
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

    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(int id) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
    virtual Result<void> clear() = 0;
    virtual Result<Simple::Contracts::Database::SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const Simple::Contracts::Database::SaveData &saveData) = 0;
    virtual Result<void> beginChanges() = 0;
    virtual Result<void> saveChanges() = 0;
    virtual Result<void> cancelChanges() = 0;

    virtual Result<QList<T>> getEntitiesInRelationOf(const Simple::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                     int entityId, const QString &field) = 0;
    virtual Result<T> getEntityInRelationOf(const Simple::Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                            const QString &field) = 0;
    virtual Result<QList<T>> updateEntitiesInRelationOf(const Simple::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                        int entityId, const QString &field,
                                                        const QList<T> &rightEntities) = 0;
    virtual Result<T> updateEntityInRelationOf(const Simple::Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                               const QString &field, const T &rightEntity) = 0;
};
} // namespace Simple::Contracts::Repository
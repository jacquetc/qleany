// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "result.h"
#include "database/types.h"
#include "entity_schema.h"
#include "types.h"
#include <QHash>
#include <QString>
#include <QUuid>

namespace DatabaseTest::Contracts::Database
{

template <class T> class InterfaceDatabaseTableGroup
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
    virtual Result<DatabaseTest::Contracts::Database::SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const DatabaseTest::Contracts::Database::SaveData &saveData) = 0;
    virtual Result<void> beginTransaction() = 0;
    virtual Result<void> commit() = 0;
    virtual Result<void> rollback() = 0;

    // get related entities
    virtual Result<QList<T>> getEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                     int entityId, const QString &field) = 0;
    virtual Result<T> getEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                            const QString &field) = 0;
    virtual Result<QList<T>> updateEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                        int leftEntityId, const QString &field,
                                                        const QList<T> &rightEntities) = 0;
    virtual Result<T> updateEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int leftEntityId,
                                               const QString &field, const T &rightEntity) = 0;
    virtual Result<void> removeAssociationsWith(QList<int> rightEntityIds) = 0;

  private:
};

} // namespace DatabaseTest::Contracts::Database
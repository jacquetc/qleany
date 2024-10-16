// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "result.h"
#include "database/interface_database_table_group.h"
#include "repository/interface_generic_repository.h"

#include <QFuture>
#include <QObject>
#include <QReadWriteLock>
#include <QUuid>

using namespace DatabaseTest::Contracts::Database;

namespace DatabaseTest::Persistence::Repository
{
// -------------------------------------------------

template <class T> class GenericRepository : public virtual DatabaseTest::Contracts::Repository::InterfaceGenericRepository<T>
{

  public:
    // InterfaceGenericRepository interface

  public:
    GenericRepository(InterfaceDatabaseTableGroup<T> *databaseTable)
    {
        m_databaseTable.reset(databaseTable);
    }

    virtual Result<T> get(const QUuid &uuid) override;
    virtual Result<T> get(const int &id) override;

    virtual Result<QList<T>> getAll() override;
    Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) override;

    virtual Result<T> add(T &&entity) override;

    virtual Result<T> update(T &&entity) override;

    virtual Result<bool> exists(const QUuid &uuid) override;
    virtual Result<bool> exists(int id) override;
    Result<void> clear() override;

    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &saveData) override;
    virtual Result<void> beginChanges() override;
    virtual Result<void> saveChanges() override;
    virtual Result<void> cancelChanges() override;
    InterfaceDatabaseTableGroup<T> *databaseTable() const;

    virtual Result<QList<T>> getEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                     int entityId, const QString &field) override;
    virtual Result<T> getEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                            const QString &field) override;
    virtual Result<QList<T>> updateEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                        int entityId, const QString &field,
                                                        const QList<T> &rightEntities) override;
    virtual Result<T> updateEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                               const QString &field, const T &rightEntity) override;

  private:
    QScopedPointer<InterfaceDatabaseTableGroup<T>> m_databaseTable;
    QReadWriteLock m_lock;

  public:
};

template <class T> Result<T> GenericRepository<T>::get(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return databaseTable()->get(uuid);
}

template <class T> Result<T> GenericRepository<T>::get(const int &id)
{
    QReadLocker locker(&m_lock);
    return databaseTable()->get(id);
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll()
{
    QReadLocker locker(&m_lock);
    return databaseTable()->getAll();
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll(const QHash<QString, QVariant> &filters)
{
    QReadLocker locker(&m_lock);
    return databaseTable()->getAll(filters);
}

template <class T> Result<T> GenericRepository<T>::add(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return databaseTable()->add(std::move(entity));
}

template <class T> Result<T> GenericRepository<T>::update(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return databaseTable()->update(std::move(entity));
}

template <class T> Result<bool> GenericRepository<T>::exists(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return databaseTable()->exists(uuid);
}

template <class T> Result<bool> GenericRepository<T>::exists(int id)
{

    QReadLocker locker(&m_lock);
    return databaseTable()->exists(id);
}

template <class T> Result<void> GenericRepository<T>::clear()
{
    QReadLocker locker(&m_lock);

    return databaseTable()->clear();
}

template <class T> Result<SaveData> GenericRepository<T>::save(const QList<int> &idList)
{
    QWriteLocker locker(&m_lock);
    return databaseTable()->save(idList);
}

template <class T> Result<void> GenericRepository<T>::restore(const SaveData &saveData)

{
    QWriteLocker locker(&m_lock);
    return databaseTable()->restore(saveData);
}

template <class T> Result<void> GenericRepository<T>::beginChanges()
{
    QWriteLocker locker(&m_lock);
    return databaseTable()->beginTransaction();
}

template <class T> Result<void> GenericRepository<T>::saveChanges()
{
    QWriteLocker locker(&m_lock);
    return databaseTable()->commit();
}

template <class T> Result<void> GenericRepository<T>::cancelChanges()
{
    QWriteLocker locker(&m_lock);
    return databaseTable()->rollback();
}

template <class T> InterfaceDatabaseTableGroup<T> *GenericRepository<T>::databaseTable() const
{
    return m_databaseTable.get();
}

template <class T>
Result<QList<T>> GenericRepository<T>::getEntitiesInRelationOf(const Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                               int entityId, const QString &field)
{
    return m_databaseTable->getEntitiesInRelationOf(leftEntitySchema, entityId, field);
}

template <class T>
Result<T> GenericRepository<T>::getEntityInRelationOf(const Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                                      const QString &field)
{
    return m_databaseTable->getEntityInRelationOf(leftEntitySchema, entityId, field);
}

template <class T>
Result<QList<T>> GenericRepository<T>::updateEntitiesInRelationOf(const Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                                  int entityId, const QString &field,
                                                                  const QList<T> &rightEntities)
{
    return m_databaseTable->updateEntitiesInRelationOf(leftEntitySchema, entityId, field, rightEntities);
}

template <class T>
Result<T> GenericRepository<T>::updateEntityInRelationOf(const Entities::EntitySchema::EntitySchema &leftEntitySchema, int entityId,
                                                         const QString &field, const T &rightEntity)
{
    return m_databaseTable->updateEntityInRelationOf(leftEntitySchema, entityId, field, rightEntity);
}

} // namespace DatabaseTest::Persistence::Repository
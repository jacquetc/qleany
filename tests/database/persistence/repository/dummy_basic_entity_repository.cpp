// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "dummy_basic_entity_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace DatabaseTest;
using namespace DatabaseTest::Persistence::Repository;
using namespace DatabaseTest::Contracts::Repository;

DummyBasicEntityRepository::DummyBasicEntityRepository(
    InterfaceDatabaseTableGroup<DatabaseTest::Entities::DummyBasicEntity> *dummyBasicEntityDatabase)
    : DatabaseTest::Persistence::Repository::GenericRepository<DatabaseTest::Entities::DummyBasicEntity>(
          dummyBasicEntityDatabase)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *DummyBasicEntityRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> DummyBasicEntityRepository::remove(
    QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>,
                        associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(DatabaseTest::Entities::Entities::EntityEnum::DummyBasicEntity,
                                              removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> DummyBasicEntityRepository::
    changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(DatabaseTest::Entities::Entities::EntityEnum::DummyBasicEntity,
                                                    changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>>(
        returnedHashOfEntityWithActiveChangedIds);
}
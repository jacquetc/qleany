// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "passenger_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace FrontEnds::Persistence::Repository;
using namespace FrontEnds::Contracts::Repository;

PassengerRepository::PassengerRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Passenger> *passengerDatabase)
    : Qleany::Repository::GenericRepository<FrontEnds::Entities::Passenger>(passengerDatabase)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *PassengerRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> PassengerRepository::remove(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<FrontEnds::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<FrontEnds::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(FrontEnds::Entities::Entities::EntityEnum::Passenger, removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> PassengerRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<FrontEnds::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(FrontEnds::Entities::Entities::EntityEnum::Passenger, changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
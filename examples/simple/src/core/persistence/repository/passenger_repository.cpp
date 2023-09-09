// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "passenger_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Persistence::Repository;
using namespace Simple::Contracts::Repository;

PassengerRepository::PassengerRepository(InterfaceDatabaseTableGroup<Simple::Domain::Passenger> *passengerDatabase)
    : Qleany::Repository::GenericRepository<Simple::Domain::Passenger>(passengerDatabase)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *PassengerRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<QHash<int, QList<int>>> PassengerRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    if (removedIdsResult.isError())
    {
        return Result<QHash<int, QList<int>>>(removedIdsResult.error());
    }

    returnedHashOfEntityWithRemovedIds.insert(Simple::Domain::Entities::Passenger, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> PassengerRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);
    if (changedIdsResult.isError())
    {
        return Result<QHash<int, QList<int>>>(changedIdsResult.error());
    }
    returnedHashOfEntityWithActiveChangedIds.insert(Simple::Domain::Entities::Passenger, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "brand_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Persistence::Repository;
using namespace Simple::Contracts::Repository;

BrandRepository::BrandRepository(InterfaceDatabaseTableGroup<Simple::Entities::Brand> *brandDatabase)
    : Qleany::Repository::GenericRepository<Simple::Entities::Brand>(brandDatabase)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *BrandRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> BrandRepository::remove(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<Simple::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Simple::Entities::Entities::EntityEnum::Brand, removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> BrandRepository::changeActiveStatusInCascade(
    QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<Simple::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Simple::Entities::Entities::EntityEnum::Brand,
                                                    changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
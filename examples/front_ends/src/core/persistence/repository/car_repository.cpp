// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "car_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace FrontEnds::Persistence::Repository;
using namespace FrontEnds::Contracts::Repository;

CarRepository::CarRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Car> *carDatabase,
                             InterfaceBrandRepository *brandRepository,
                             InterfacePassengerRepository *passengerRepository)
    : Qleany::Repository::GenericRepository<FrontEnds::Entities::Car>(carDatabase), m_brandRepository(brandRepository),
      m_passengerRepository(passengerRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *CarRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<FrontEnds::Entities::Car> CarRepository::update(Entities::Car &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().brandSet)
    {

        Result<Entities::Brand> brandResult =
            m_brandRepository->updateEntityInRelationOf(Entities::Car::schema, entity.id(), "brand", entity.brand());

#ifdef QT_DEBUG
        if (brandResult.isError())
        {
            qCritical() << brandResult.error().code() << brandResult.error().message() << brandResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::Car, brandResult)
    }

    if (entity.metaData().passengersSet)
    {

        Result<QList<Entities::Passenger>> passengersResult = m_passengerRepository->updateEntitiesInRelationOf(
            Entities::Car::schema, entity.id(), "passengers", entity.passengers());

#ifdef QT_DEBUG
        if (passengersResult.isError())
        {
            qCritical() << passengersResult.error().code() << passengersResult.error().message()
                        << passengersResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::Car, passengersResult)
    }

    return Qleany::Repository::GenericRepository<Entities::Car>::update(std::move(entity));
}

Result<FrontEnds::Entities::Car> CarRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Entities::Car>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Entities::Car entity = getResult.value();

    Result<Entities::Brand> brandResult =
        m_brandRepository->getEntityInRelationOf(Entities::Car::schema, entity.id(), "brand");

#ifdef QT_DEBUG
    if (brandResult.isError())
    {
        qCritical() << brandResult.error().code() << brandResult.error().message() << brandResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::Car, brandResult)

    entity.setBrand(brandResult.value());

    Result<QList<Entities::Passenger>> passengersResult =
        m_passengerRepository->getEntitiesInRelationOf(Entities::Car::schema, entity.id(), "passengers");

#ifdef QT_DEBUG
    if (passengersResult.isError())
    {
        qCritical() << passengersResult.error().code() << passengersResult.error().message()
                    << passengersResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::Car, passengersResult)

    entity.setPassengers(passengersResult.value());

    return Result<Entities::Car>(entity);
}

FrontEnds::Entities::Car::BrandLoader CarRepository::fetchBrandLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "brand" property in the entity Car using staticMetaObject
    int propertyIndex = FrontEnds::Entities::Car::staticMetaObject.indexOfProperty("brand");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Car doesn't have a property named brand";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult =
            m_brandRepository->getEntityInRelationOf(FrontEnds::Entities::Car::schema, entityId, "brand");

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return FrontEnds::Entities::Brand();
        }

        return foreignEntityResult.value();
    };
}

FrontEnds::Entities::Car::PassengersLoader CarRepository::fetchPassengersLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "passengers" property in the entity Car using staticMetaObject
    int propertyIndex = FrontEnds::Entities::Car::staticMetaObject.indexOfProperty("passengers");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Car doesn't have a property named passengers";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult =
            m_passengerRepository->getEntitiesInRelationOf(FrontEnds::Entities::Car::schema, entityId, "passengers");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<FrontEnds::Entities::Passenger>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> CarRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the brand in cascade

    Qleany::Entities::RelationshipInfo brandBrandRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Car::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Brand && relationship.fieldName == "brand")
        {
            brandBrandRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (brandBrandRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            FrontEnds::Entities::Brand foreignBrand = this->fetchBrandLoader().operator()(entityId);

            if (!foreignBrand.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignBrand.id());

            auto removalResult = m_brandRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the passengers in cascade

    Qleany::Entities::RelationshipInfo passengerPassengersRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Car::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == "passengers")
        {
            passengerPassengersRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerPassengersRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<FrontEnds::Entities::Passenger> foreignPassengers =
                this->fetchPassengersLoader().operator()(entityId);

            if (foreignPassengers.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &passenger : foreignPassengers)
            {
                foreignIds.append(passenger.id());
            }

            auto removalResult = m_passengerRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(FrontEnds::Entities::Entities::Car, removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> CarRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the brand in cascade

    Qleany::Entities::RelationshipInfo brandBrandRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Car::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Brand && relationship.fieldName == "brand")
        {
            brandBrandRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (brandBrandRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            FrontEnds::Entities::Brand foreignBrand = this->fetchBrandLoader().operator()(entityId);

            if (!foreignBrand.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignBrand.id());

            auto changeResult = m_brandRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the passengers in cascade

    Qleany::Entities::RelationshipInfo passengerPassengersRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Car::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == "passengers")
        {
            passengerPassengersRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerPassengersRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<FrontEnds::Entities::Passenger> foreignPassengers =
                this->fetchPassengersLoader().operator()(entityId);

            if (foreignPassengers.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &passenger : foreignPassengers)
            {
                foreignIds.append(passenger.id());
            }

            auto changeResult = m_passengerRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(FrontEnds::Entities::Entities::Car, changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
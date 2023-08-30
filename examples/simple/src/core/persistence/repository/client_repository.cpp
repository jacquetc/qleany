// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "client_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Persistence::Repository;
using namespace Simple::Contracts::Repository;

ClientRepository::ClientRepository(InterfaceDatabaseTableGroup<Simple::Domain::Client> *clientDatabase,
                                   InterfacePassengerRepository *passengerRepository)
    : Qleany::Repository::GenericRepository<Simple::Domain::Client>(clientDatabase),
      m_passengerRepository(passengerRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *ClientRepository::signalHolder()
{
    return m_signalHolder.data();
}

Result<Simple::Domain::Client> ClientRepository::update(Domain::Client &&entity)
{

    if (entity.clientSet())
    {

        Result<Domain::Passenger> clientResult = m_passengerRepository->updateEntityInRelationOf(
            Domain::Client::schema, entity.id(), "client", entity.client());

        if (clientResult.isError())
        {
#ifdef QT_DEBUG
            qCritical() << clientResult.error().code() << clientResult.error().message() << clientResult.error().data();
            qFatal("Error found. The application will now exit");
#endif
            return Result<Domain::Client>(clientResult.error());
        }
    }

    return Qleany::Repository::GenericRepository<Domain::Client>::update(std::move(entity));
}

Result<Simple::Domain::Client> ClientRepository::getWithDetails(int entityId)
{
    auto getResult = Qleany::Repository::GenericRepository<Domain::Client>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Client entity = getResult.value();

    Result<Domain::Passenger> clientResult =
        m_passengerRepository->getEntityInRelationOf(Domain::Client::schema, entity.id(), "client");

    if (clientResult.isError())
    {
#ifdef QT_DEBUG
        qCritical() << clientResult.error().code() << clientResult.error().message() << clientResult.error().data();
        qFatal("Error found. The application will now exit");
#endif
        return Result<Domain::Client>(clientResult.error());
    }
    entity.setClient(clientResult.value());

    return Result<Domain::Client>(entity);
}

Simple::Domain::Client::ClientLoader ClientRepository::fetchClientLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "client" property in the entity Client using staticMetaObject
    int propertyIndex = Simple::Domain::Client::staticMetaObject.indexOfProperty("client");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Client doesn't have a property named client";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult =
            m_passengerRepository->getEntityInRelationOf(Simple::Domain::Passenger::schema, entityId, "client");

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return Simple::Domain::Passenger();
        }

        return foreignEntityResult.value();
    };
}

Result<QHash<int, QList<int>>> ClientRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the client in cascade

    Qleany::Domain::RelationshipInfo passengerClientRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Simple::Domain::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Domain::Entities::Passenger && relationship.fieldName == "client")
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            Simple::Domain::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto removalResult = m_passengerRepository->removeInCascade(foreignIds);
            if (removalResult.isError())
            {
                return Result<QHash<int, QList<int>>>(removalResult.error());
            }

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    if (removedIdsResult.isError())
    {
        return Result<QHash<int, QList<int>>>(removedIdsResult.error());
    }

    returnedHashOfEntityWithRemovedIds.insert(Simple::Domain::Entities::Client, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> ClientRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the client in cascade

    Qleany::Domain::RelationshipInfo passengerClientRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Simple::Domain::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Domain::Entities::Passenger && relationship.fieldName == "client")
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            Simple::Domain::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto changeResult = m_passengerRepository->changeActiveStatusInCascade(foreignIds, active);
            if (changeResult.isError())
            {
                return Result<QHash<int, QList<int>>>(changeResult.error());
            }

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);
    if (changedIdsResult.isError())
    {
        return Result<QHash<int, QList<int>>>(changedIdsResult.error());
    }
    returnedHashOfEntityWithActiveChangedIds.insert(Simple::Domain::Entities::Client, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}

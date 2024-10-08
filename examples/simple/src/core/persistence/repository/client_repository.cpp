// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "client_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Simple;
using namespace Simple::Persistence::Repository;
using namespace Simple::Contracts::Repository;

ClientRepository::ClientRepository(InterfaceDatabaseTableGroup<Simple::Entities::Client> *clientDatabase,
                                   InterfacePassengerRepository *passengerRepository)
    : Simple::Persistence::Repository::GenericRepository<Simple::Entities::Client>(clientDatabase),
      m_passengerRepository(passengerRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *ClientRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Simple::Entities::Client> ClientRepository::update(Entities::Client &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().clientSet)
    {

        Result<Entities::Passenger> clientResult = m_passengerRepository->updateEntityInRelationOf(
            Entities::Client::schema, entity.id(), "client"_L1, entity.client());

#ifdef QT_DEBUG
        if (clientResult.isError())
        {
            qCritical() << clientResult.error().code() << clientResult.error().message() << clientResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::Client, clientResult)
    }

    if (entity.metaData().clientFriendsSet)
    {

        Result<QList<Entities::Passenger>> clientFriendsResult = m_passengerRepository->updateEntitiesInRelationOf(
            Entities::Client::schema, entity.id(), "clientFriends"_L1, entity.clientFriends());

#ifdef QT_DEBUG
        if (clientFriendsResult.isError())
        {
            qCritical() << clientFriendsResult.error().code() << clientFriendsResult.error().message()
                        << clientFriendsResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::Client, clientFriendsResult)
    }

    return Simple::Persistence::Repository::GenericRepository<Entities::Client>::update(std::move(entity));
}

Result<Simple::Entities::Client> ClientRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Simple::Persistence::Repository::GenericRepository<Entities::Client>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Entities::Client entity = getResult.value();

    Result<Entities::Passenger> clientResult =
        m_passengerRepository->getEntityInRelationOf(Entities::Client::schema, entity.id(), "client"_L1);

#ifdef QT_DEBUG
    if (clientResult.isError())
    {
        qCritical() << clientResult.error().code() << clientResult.error().message() << clientResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::Client, clientResult)

    entity.setClient(clientResult.value());

    Result<QList<Entities::Passenger>> clientFriendsResult =
        m_passengerRepository->getEntitiesInRelationOf(Entities::Client::schema, entity.id(), "clientFriends"_L1);

#ifdef QT_DEBUG
    if (clientFriendsResult.isError())
    {
        qCritical() << clientFriendsResult.error().code() << clientFriendsResult.error().message()
                    << clientFriendsResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::Client, clientFriendsResult)

    entity.setClientFriends(clientFriendsResult.value());

    return Result<Entities::Client>(entity);
}

Simple::Entities::Client::ClientLoader ClientRepository::fetchClientLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "client" property in the entity Client using staticMetaObject
    int propertyIndex = Simple::Entities::Client::staticMetaObject.indexOfProperty("client");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Client doesn't have a property named client";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult =
            m_passengerRepository->getEntityInRelationOf(Simple::Entities::Client::schema, entityId, "client"_L1);

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return Simple::Entities::Passenger();
        }

        return foreignEntityResult.value();
    };
}

Simple::Entities::Client::ClientFriendsLoader ClientRepository::fetchClientFriendsLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "clientFriends" property in the entity Client using staticMetaObject
    int propertyIndex = Simple::Entities::Client::staticMetaObject.indexOfProperty("clientFriends");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Client doesn't have a property named clientFriends";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult = m_passengerRepository->getEntitiesInRelationOf(
            Simple::Entities::Client::schema, entityId, QString::fromLatin1("clientFriends"));

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Simple::Entities::Passenger>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> ClientRepository::remove(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<Simple::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the client in cascade

    Simple::Entities::EntitySchema::RelationshipInfo passengerClientRelationship;
    for (const Simple::Entities::EntitySchema::RelationshipInfo &relationship :
         Simple::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Entities::Entities::EntityEnum::Passenger &&
            relationship.fieldName == "client"_L1)
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Simple::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            Simple::Entities::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            if (!foreignClient.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto removalResult = m_passengerRepository->remove(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the clientFriends in cascade

    Simple::Entities::EntitySchema::RelationshipInfo passengerClientFriendsRelationship;
    for (const Simple::Entities::EntitySchema::RelationshipInfo &relationship :
         Simple::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Entities::Entities::EntityEnum::Passenger &&
            relationship.fieldName == "clientFriends"_L1)
        {
            passengerClientFriendsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientFriendsRelationship.strength == Simple::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Simple::Entities::Passenger> foreignClientFriends =
                this->fetchClientFriendsLoader().operator()(entityId);

            if (foreignClientFriends.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &passenger : foreignClientFriends)
            {
                foreignIds.append(passenger.id());
            }

            auto removalResult = m_passengerRepository->remove(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Simple::Entities::Entities::EntityEnum::Client, removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> ClientRepository::changeActiveStatusInCascade(
    QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<Simple::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the client in cascade

    Simple::Entities::EntitySchema::RelationshipInfo passengerClientRelationship;
    for (const Simple::Entities::EntitySchema::RelationshipInfo &relationship :
         Simple::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Entities::Entities::EntityEnum::Passenger &&
            relationship.fieldName == QString::fromLatin1("client"))
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Simple::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            Simple::Entities::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            if (!foreignClient.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto changeResult = m_passengerRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the clientFriends in cascade

    Simple::Entities::EntitySchema::RelationshipInfo passengerClientFriendsRelationship;
    for (const Simple::Entities::EntitySchema::RelationshipInfo &relationship :
         Simple::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == Simple::Entities::Entities::EntityEnum::Passenger &&
            relationship.fieldName == QString::fromLatin1("clientFriends"))
        {
            passengerClientFriendsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientFriendsRelationship.strength == Simple::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Simple::Entities::Passenger> foreignClientFriends =
                this->fetchClientFriendsLoader().operator()(entityId);

            if (foreignClientFriends.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &passenger : foreignClientFriends)
            {
                foreignIds.append(passenger.id());
            }

            auto changeResult = m_passengerRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<Simple::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Simple::Entities::Entities::EntityEnum::Client,
                                                    changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
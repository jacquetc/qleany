// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "client_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace FrontEnds::Persistence::Repository;
using namespace FrontEnds::Contracts::Repository;

ClientRepository::ClientRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Client> *clientDatabase,
                                   InterfacePassengerRepository *passengerRepository)
    : Qleany::Repository::GenericRepository<FrontEnds::Entities::Client>(clientDatabase),
      m_passengerRepository(passengerRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *ClientRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<FrontEnds::Entities::Client> ClientRepository::update(Entities::Client &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().clientSet)
    {

        Result<Entities::Passenger> clientResult = m_passengerRepository->updateEntityInRelationOf(
            Entities::Client::schema, entity.id(), QString::fromLatin1("client"), entity.client());

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
            Entities::Client::schema, entity.id(), QString::fromLatin1("clientFriends"), entity.clientFriends());

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

    return Qleany::Repository::GenericRepository<Entities::Client>::update(std::move(entity));
}

Result<FrontEnds::Entities::Client> ClientRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Entities::Client>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Entities::Client entity = getResult.value();

    Result<Entities::Passenger> clientResult = m_passengerRepository->getEntityInRelationOf(
        Entities::Client::schema, entity.id(), QString::fromLatin1("client"));

#ifdef QT_DEBUG
    if (clientResult.isError())
    {
        qCritical() << clientResult.error().code() << clientResult.error().message() << clientResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::Client, clientResult)

    entity.setClient(clientResult.value());

    Result<QList<Entities::Passenger>> clientFriendsResult = m_passengerRepository->getEntitiesInRelationOf(
        Entities::Client::schema, entity.id(), QString::fromLatin1("clientFriends"));

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

FrontEnds::Entities::Client::ClientLoader ClientRepository::fetchClientLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "client" property in the entity Client using staticMetaObject
    int propertyIndex = FrontEnds::Entities::Client::staticMetaObject.indexOfProperty("client");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Client doesn't have a property named client";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult = m_passengerRepository->getEntityInRelationOf(
            FrontEnds::Entities::Client::schema, entityId, QString::fromLatin1("client"));

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return FrontEnds::Entities::Passenger();
        }

        return foreignEntityResult.value();
    };
}

FrontEnds::Entities::Client::ClientFriendsLoader ClientRepository::fetchClientFriendsLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "clientFriends" property in the entity Client using staticMetaObject
    int propertyIndex = FrontEnds::Entities::Client::staticMetaObject.indexOfProperty("clientFriends");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Client doesn't have a property named clientFriends";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult = m_passengerRepository->getEntitiesInRelationOf(
            FrontEnds::Entities::Client::schema, entityId, QString::fromLatin1("clientFriends"));

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<FrontEnds::Entities::Passenger>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> ClientRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the client in cascade

    Qleany::Entities::RelationshipInfo passengerClientRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == QString::fromLatin1("client"))
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            FrontEnds::Entities::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            if (!foreignClient.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto removalResult = m_passengerRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the clientFriends in cascade

    Qleany::Entities::RelationshipInfo passengerClientFriendsRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == QString::fromLatin1("clientFriends"))
        {
            passengerClientFriendsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientFriendsRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<FrontEnds::Entities::Passenger> foreignClientFriends =
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

    returnedHashOfEntityWithRemovedIds.insert(FrontEnds::Entities::Entities::Client, removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> ClientRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the client in cascade

    Qleany::Entities::RelationshipInfo passengerClientRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == QString::fromLatin1("client"))
        {
            passengerClientRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            FrontEnds::Entities::Passenger foreignClient = this->fetchClientLoader().operator()(entityId);

            if (!foreignClient.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignClient.id());

            auto changeResult = m_passengerRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the clientFriends in cascade

    Qleany::Entities::RelationshipInfo passengerClientFriendsRelationship;
    for (const Qleany::Entities::RelationshipInfo &relationship : FrontEnds::Entities::Client::schema.relationships)
    {
        if (relationship.rightEntityId == FrontEnds::Entities::Entities::Passenger &&
            relationship.fieldName == QString::fromLatin1("clientFriends"))
        {
            passengerClientFriendsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (passengerClientFriendsRelationship.strength == Qleany::Entities::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<FrontEnds::Entities::Passenger> foreignClientFriends =
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

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(FrontEnds::Entities::Entities::Client, changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
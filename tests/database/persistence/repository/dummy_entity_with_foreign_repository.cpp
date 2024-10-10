// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "dummy_entity_with_foreign_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace DatabaseTest;
using namespace DatabaseTest::Persistence::Repository;
using namespace DatabaseTest::Contracts::Repository;

DummyEntityWithForeignRepository::DummyEntityWithForeignRepository(
    InterfaceDatabaseTableGroup<DatabaseTest::Entities::DummyEntityWithForeign> *dummyEntityWithForeignDatabase,
    InterfaceDummyOtherEntityRepository *dummyOtherEntityRepository)
    : DatabaseTest::Persistence::Repository::GenericRepository<DatabaseTest::Entities::DummyEntityWithForeign>(
          dummyEntityWithForeignDatabase),
      m_dummyOtherEntityRepository(dummyOtherEntityRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *DummyEntityWithForeignRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<DatabaseTest::Entities::DummyEntityWithForeign> DummyEntityWithForeignRepository::update(
    Entities::DummyEntityWithForeign &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().uniqueSet)
    {

        Result<Entities::DummyOtherEntity> uniqueResult = m_dummyOtherEntityRepository->updateEntityInRelationOf(
            Entities::DummyEntityWithForeign::schema, entity.id(), "unique"_L1, entity.unique());

#ifdef QT_DEBUG
        if (uniqueResult.isError())
        {
            qCritical() << uniqueResult.error().code() << uniqueResult.error().message() << uniqueResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, uniqueResult)
    }

    if (entity.metaData().orderedListSet)
    {

        Result<QList<Entities::DummyOtherEntity>> orderedListResult =
            m_dummyOtherEntityRepository->updateEntitiesInRelationOf(
                Entities::DummyEntityWithForeign::schema, entity.id(), "orderedList"_L1, entity.orderedList());

#ifdef QT_DEBUG
        if (orderedListResult.isError())
        {
            qCritical() << orderedListResult.error().code() << orderedListResult.error().message()
                        << orderedListResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, orderedListResult)
    }

    if (entity.metaData().unorderedListSet)
    {

        Result<QList<Entities::DummyOtherEntity>> unorderedListResult =
            m_dummyOtherEntityRepository->updateEntitiesInRelationOf(
                Entities::DummyEntityWithForeign::schema, entity.id(), "unorderedList"_L1, entity.unorderedList());

#ifdef QT_DEBUG
        if (unorderedListResult.isError())
        {
            qCritical() << unorderedListResult.error().code() << unorderedListResult.error().message()
                        << unorderedListResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, unorderedListResult)
    }

    return DatabaseTest::Persistence::Repository::GenericRepository<Entities::DummyEntityWithForeign>::update(
        std::move(entity));
}

Result<DatabaseTest::Entities::DummyEntityWithForeign> DummyEntityWithForeignRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult =
        DatabaseTest::Persistence::Repository::GenericRepository<Entities::DummyEntityWithForeign>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Entities::DummyEntityWithForeign entity = getResult.value();

    Result<Entities::DummyOtherEntity> uniqueResult = m_dummyOtherEntityRepository->getEntityInRelationOf(
        Entities::DummyEntityWithForeign::schema, entity.id(), "unique"_L1);

#ifdef QT_DEBUG
    if (uniqueResult.isError())
    {
        qCritical() << uniqueResult.error().code() << uniqueResult.error().message() << uniqueResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, uniqueResult)

    entity.setUnique(uniqueResult.value());

    Result<QList<Entities::DummyOtherEntity>> orderedListResult = m_dummyOtherEntityRepository->getEntitiesInRelationOf(
        Entities::DummyEntityWithForeign::schema, entity.id(), "orderedList"_L1);

#ifdef QT_DEBUG
    if (orderedListResult.isError())
    {
        qCritical() << orderedListResult.error().code() << orderedListResult.error().message()
                    << orderedListResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, orderedListResult)

    entity.setOrderedList(orderedListResult.value());

    Result<QList<Entities::DummyOtherEntity>> unorderedListResult =
        m_dummyOtherEntityRepository->getEntitiesInRelationOf(Entities::DummyEntityWithForeign::schema, entity.id(),
                                                              "unorderedList"_L1);

#ifdef QT_DEBUG
    if (unorderedListResult.isError())
    {
        qCritical() << unorderedListResult.error().code() << unorderedListResult.error().message()
                    << unorderedListResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Entities::DummyEntityWithForeign, unorderedListResult)

    entity.setUnorderedList(unorderedListResult.value());

    return Result<Entities::DummyEntityWithForeign>(entity);
}

DatabaseTest::Entities::DummyEntityWithForeign::UniqueLoader DummyEntityWithForeignRepository::fetchUniqueLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "unique" property in the entity DummyEntityWithForeign using staticMetaObject
    int propertyIndex = DatabaseTest::Entities::DummyEntityWithForeign::staticMetaObject.indexOfProperty("unique");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity DummyEntityWithForeign doesn't have a property named unique";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult = m_dummyOtherEntityRepository->getEntityInRelationOf(
            DatabaseTest::Entities::DummyEntityWithForeign::schema, entityId, "unique"_L1);

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return DatabaseTest::Entities::DummyOtherEntity();
        }

        return foreignEntityResult.value();
    };
}

DatabaseTest::Entities::DummyEntityWithForeign::OrderedListLoader DummyEntityWithForeignRepository::
    fetchOrderedListLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "orderedList" property in the entity DummyEntityWithForeign using staticMetaObject
    int propertyIndex = DatabaseTest::Entities::DummyEntityWithForeign::staticMetaObject.indexOfProperty("orderedList");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity DummyEntityWithForeign doesn't have a property named orderedList";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult = m_dummyOtherEntityRepository->getEntitiesInRelationOf(
            DatabaseTest::Entities::DummyEntityWithForeign::schema, entityId, QString::fromLatin1("orderedList"));

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<DatabaseTest::Entities::DummyOtherEntity>();
        }

        return foreignEntitiesResult.value();
    };
}

DatabaseTest::Entities::DummyEntityWithForeign::UnorderedListLoader DummyEntityWithForeignRepository::
    fetchUnorderedListLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "unorderedList" property in the entity DummyEntityWithForeign using staticMetaObject
    int propertyIndex =
        DatabaseTest::Entities::DummyEntityWithForeign::staticMetaObject.indexOfProperty("unorderedList");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity DummyEntityWithForeign doesn't have a property named unorderedList";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult = m_dummyOtherEntityRepository->getEntitiesInRelationOf(
            DatabaseTest::Entities::DummyEntityWithForeign::schema, entityId, QString::fromLatin1("unorderedList"));

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<DatabaseTest::Entities::DummyOtherEntity>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> DummyEntityWithForeignRepository::remove(
    QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the unique in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityUniqueRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == "unique"_L1)
        {
            dummyOtherEntityUniqueRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityUniqueRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            DatabaseTest::Entities::DummyOtherEntity foreignUnique = this->fetchUniqueLoader().operator()(entityId);

            if (!foreignUnique.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignUnique.id());

            auto removalResult = m_dummyOtherEntityRepository->remove(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the orderedList in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityOrderedListRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == "orderedList"_L1)
        {
            dummyOtherEntityOrderedListRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityOrderedListRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<DatabaseTest::Entities::DummyOtherEntity> foreignOrderedList =
                this->fetchOrderedListLoader().operator()(entityId);

            if (foreignOrderedList.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &dummyOtherEntity : foreignOrderedList)
            {
                foreignIds.append(dummyOtherEntity.id());
            }

            auto removalResult = m_dummyOtherEntityRepository->remove(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the unorderedList in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityUnorderedListRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == "unorderedList"_L1)
        {
            dummyOtherEntityUnorderedListRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityUnorderedListRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<DatabaseTest::Entities::DummyOtherEntity> foreignUnorderedList =
                this->fetchUnorderedListLoader().operator()(entityId);

            if (foreignUnorderedList.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &dummyOtherEntity : foreignUnorderedList)
            {
                foreignIds.append(dummyOtherEntity.id());
            }

            auto removalResult = m_dummyOtherEntityRepository->remove(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>,
                        associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign,
                                              removedIdsResult.value());

    Q_EMIT m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> DummyEntityWithForeignRepository::
    changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the unique in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityUniqueRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == QString::fromLatin1("unique"))
        {
            dummyOtherEntityUniqueRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityUniqueRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            DatabaseTest::Entities::DummyOtherEntity foreignUnique = this->fetchUniqueLoader().operator()(entityId);

            if (!foreignUnique.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignUnique.id());

            auto changeResult = m_dummyOtherEntityRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the orderedList in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityOrderedListRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == QString::fromLatin1("orderedList"))
        {
            dummyOtherEntityOrderedListRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityOrderedListRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<DatabaseTest::Entities::DummyOtherEntity> foreignOrderedList =
                this->fetchOrderedListLoader().operator()(entityId);

            if (foreignOrderedList.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &dummyOtherEntity : foreignOrderedList)
            {
                foreignIds.append(dummyOtherEntity.id());
            }

            auto changeResult = m_dummyOtherEntityRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the unorderedList in cascade

    DatabaseTest::Entities::EntitySchema::RelationshipInfo dummyOtherEntityUnorderedListRelationship;
    for (const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship :
         DatabaseTest::Entities::DummyEntityWithForeign::schema.relationships)
    {
        if (relationship.rightEntityId == DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity &&
            relationship.fieldName == QString::fromLatin1("unorderedList"))
        {
            dummyOtherEntityUnorderedListRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (dummyOtherEntityUnorderedListRelationship.strength ==
            DatabaseTest::Entities::EntitySchema::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<DatabaseTest::Entities::DummyOtherEntity> foreignUnorderedList =
                this->fetchUnorderedListLoader().operator()(entityId);

            if (foreignUnorderedList.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &dummyOtherEntity : foreignUnorderedList)
            {
                foreignIds.append(dummyOtherEntity.id());
            }

            auto changeResult = m_dummyOtherEntityRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<DatabaseTest::Entities::Entities::EntityEnum QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(
        DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, changedIdsResult.value());
    Q_EMIT m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>>(
        returnedHashOfEntityWithActiveChangedIds);
}
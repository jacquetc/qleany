#pragma once

#include "result.h"
#include "database/one_to_many_ordered_associator.h"

//---------------------------

using namespace Simple;

template <class RightEntity>
class OneToManyOrderedAssociatorWrapper : public Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>
{
  public:
    OneToManyOrderedAssociatorWrapper(QSharedPointer<InterfaceDatabaseContext> context,
                                      const Simple::Entities::RelationshipInfo &relationship)
        : Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>(context, relationship)
    {
    }

    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);

    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const Simple::Entities::EntitySchema &entitySchema) const;
    QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> mergeShadows(
        const QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
        const QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows)
        const;
};

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntities(int leftEntityId)
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::getRightEntities(leftEntityId);
}

template <class RightEntity> QString OneToManyOrderedAssociatorWrapper<RightEntity>::getTableCreationSql() const
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::getTableCreationSql();
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::updateRightEntities(leftEntityId, rightEntities);
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(rightEntityIds);
}

template <class RightEntity>
QStringList OneToManyOrderedAssociatorWrapper<RightEntity>::getTablePropertyColumns(
    const Simple::Entities::EntitySchema &entitySchema) const
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::getTablePropertyColumns(entitySchema);
}

template <class RightEntity>
QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow>
OneToManyOrderedAssociatorWrapper<RightEntity>::mergeShadows(
    const QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
    const QList<typename Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows) const
{
    return Simple::Persistence::Database::OneToManyOrderedAssociator<RightEntity>::mergeShadows(originalShadows, newShadows);
}

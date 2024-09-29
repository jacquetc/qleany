#pragma once

#include "result.h"
#include "examples/simple/src/core/persistence/database/one_to_many_ordered_associator.h"

//---------------------------

using namespace Qleany;

template <class RightEntity>
class OneToManyOrderedAssociatorWrapper : public Qleany::Database::OneToManyOrderedAssociator<RightEntity>
{
  public:
    OneToManyOrderedAssociatorWrapper(QSharedPointer<InterfaceDatabaseContext> context,
                                      const Qleany::Entities::RelationshipInfo &relationship)
        : Qleany::Database::OneToManyOrderedAssociator<RightEntity>(context, relationship)
    {
    }

    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);

    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const Qleany::Entities::EntitySchema &entitySchema) const;
    QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> mergeShadows(
        const QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
        const QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows)
        const;
};

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntities(int leftEntityId)
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::getRightEntities(leftEntityId);
}

template <class RightEntity> QString OneToManyOrderedAssociatorWrapper<RightEntity>::getTableCreationSql() const
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::getTableCreationSql();
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::updateRightEntities(leftEntityId, rightEntities);
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(rightEntityIds);
}

template <class RightEntity>
QStringList OneToManyOrderedAssociatorWrapper<RightEntity>::getTablePropertyColumns(
    const Entities::EntitySchema &entitySchema) const
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::getTablePropertyColumns(entitySchema);
}

template <class RightEntity>
QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow>
OneToManyOrderedAssociatorWrapper<RightEntity>::mergeShadows(
    const QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
    const QList<typename Qleany::Database::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows) const
{
    return Qleany::Database::OneToManyOrderedAssociator<RightEntity>::mergeShadows(originalShadows, newShadows);
}

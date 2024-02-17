#pragma once

#include "qleany/common/result.h"
#include "qleany/immutable_database/one_to_many_ordered_associator.h"

//---------------------------

using namespace Qleany;

template <class RightEntity>
class OneToManyOrderedAssociatorWrapper : public Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>
{
  public:
    OneToManyOrderedAssociatorWrapper(QSharedPointer<InterfaceDatabaseContext> context,
                                      const Qleany::Domain::RelationshipInfo &relationship)
        : Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>(context, relationship)
    {
    }

    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);

    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const Qleany::Domain::EntitySchema &entitySchema) const;
    QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow> mergeShadows(
        const QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
        const QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows)
        const;
};

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntities(int leftEntityId)
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::getRightEntities(leftEntityId);
}

template <class RightEntity> QString OneToManyOrderedAssociatorWrapper<RightEntity>::getTableCreationSql() const
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::getTableCreationSql();
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::updateRightEntities(leftEntityId, rightEntities);
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociatorWrapper<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(rightEntityIds);
}

template <class RightEntity>
QStringList OneToManyOrderedAssociatorWrapper<RightEntity>::getTablePropertyColumns(
    const Domain::EntitySchema &entitySchema) const
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::getTablePropertyColumns(entitySchema);
}

template <class RightEntity>
QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow>
OneToManyOrderedAssociatorWrapper<RightEntity>::mergeShadows(
    const QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &originalShadows,
    const QList<typename Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::EntityShadow> &newShadows) const
{
    return Qleany::ImmutableDatabase::OneToManyOrderedAssociator<RightEntity>::mergeShadows(originalShadows, newShadows);
}

#pragma once

#include "qleany/contracts/immutable_database/interface_database_context.h"
#include "tools.h"
#include <QList>
#include <QSharedPointer>
#include <QSqlError>
#include <QSqlQuery>
#include <qleany/common/result.h>
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Contracts::ImmutableDatabase;

namespace Qleany::ImmutableDatabase
{
template <class RightEntity> class OneToManyOrderedAssociator
{
  public:
    OneToManyOrderedAssociator(QSharedPointer<Qleany::Contracts::ImmutableDatabase::InterfaceDatabaseContext> context,
                               const Qleany::Domain::RelationshipInfo &relationship)
        : m_databaseContext(context), m_relationship(relationship), m_fieldName(relationship.fieldName)
    {

        QString leftEntityLastName = relationship.leftEntityName;
        QString rightEntityLastName = RightEntity::schema.name;

        m_junctionTableName =
            leftEntityLastName + "_" + relationship.fieldName + "_" + rightEntityLastName + "_junction";
        m_junctionTableLeftEntityForeignKeyName = leftEntityLastName + "_id";
        m_leftEntityForeignTableName = Qleany::ImmutableDatabase::Tools::fromPascalToSnakeCase(leftEntityLastName);
        m_junctionTableRightEntityForeignKeyName = rightEntityLastName + "_id";
        m_rightEntityForeignTableName = Qleany::ImmutableDatabase::TableTools<RightEntity>::getEntityTableName();
    }
    ~OneToManyOrderedAssociator() = default;
    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);
    Result<void> removeTheseRightIds(QList<int> rightEntityIds);

    struct EntityShadow
    {
        EntityShadow() = default;
        EntityShadow(int junctionTableId, int entityId, int order, int previous, int next)
            : junctionTableId(junctionTableId), entityId(entityId), order(order), previous(previous), next(next)
        {
        }

        bool isNull()
        {
            return entityId == 0;
        }

        // equality operator only on entityId
        bool operator==(const EntityShadow &other) const
        {
            return entityId == other.entityId;
        }

        int junctionTableId;
        int entityId = 0;
        int order;
        bool create = false;
        bool remove = false;
        bool common = false;
        int previous;
        int newPrevious = 0;
        int next;
        int newNext = 0;
        bool updatePreviousOrNext = false;
    };

  protected:
    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const Qleany::Domain::EntitySchema &entitySchema) const;
    QList<EntityShadow> mergeShadows(const QList<EntityShadow> &originalShadows,
                                     const QList<EntityShadow> &newShadows) const;

  private:
    QSharedPointer<Qleany::Contracts::ImmutableDatabase::InterfaceDatabaseContext>
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this
                            * DatabaseTableGroup.
                            */

    QString m_junctionTableName;
    QString m_junctionTableLeftEntityForeignKeyName;
    QString m_leftEntityForeignTableName;
    QString m_junctionTableRightEntityForeignKeyName;
    QString m_rightEntityForeignTableName;
    Qleany::Domain::RelationshipInfo m_relationship;
    Qleany::Domain::EntitySchema m_rightEntitySchema = RightEntity::schema;
    const QStringList m_rightEntityPropertyColumns = getTablePropertyColumns(RightEntity::schema);
    Qleany::Domain::EntitySchema m_leftEntitySchema;
    QString m_fieldName;
    QList<OneToManyOrderedAssociator<RightEntity>::EntityShadow> writePreviousAndNext(
        const QList<EntityShadow> &shadows) const;
    QList<OneToManyOrderedAssociator<RightEntity>::EntityShadow> markUpdatePreviousOrNext(
        const QList<EntityShadow> &shadows) const;
};

// Contrary to other Associators, the foreign keys are on ON DELETE RESTRICT
template <class RightEntity> QString OneToManyOrderedAssociator<RightEntity>::getTableCreationSql() const
{
    return "CREATE TABLE IF NOT EXISTS " + m_junctionTableName +
           " (id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON "
           "CONFLICT ROLLBACK, previous INTEGER, next INTEGER," +
           m_junctionTableLeftEntityForeignKeyName + " INTEGER NOT NULL, " + m_junctionTableRightEntityForeignKeyName +
           " INTEGER NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, FOREIGN KEY (" +
           m_junctionTableLeftEntityForeignKeyName + ") REFERENCES " + m_leftEntityForeignTableName +
           " (id) ON DELETE RESTRICT, FOREIGN KEY (" + m_junctionTableRightEntityForeignKeyName + ") REFERENCES " +
           m_rightEntityForeignTableName + " (id) ON DELETE RESTRICT, " + "UNIQUE (" +
           m_junctionTableLeftEntityForeignKeyName + ", " + m_junctionTableRightEntityForeignKeyName +
           ") ON CONFLICT ROLLBACK" + ");";
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociator<RightEntity>::getRightEntities(int leftEntityId)
{
    auto connection = m_databaseContext->getConnection();

    QString queryStr = QString("WITH RECURSIVE ordered_relationships(id, %3, row_number) AS ("
                               "  SELECT id, %3, 1"
                               "  FROM %1"
                               "  WHERE previous IS NULL AND %2 = :entityId"
                               "  UNION ALL"
                               "  SELECT deo.id, deo.%3, o_r.row_number + 1"
                               "  FROM %1 deo"
                               "  JOIN ordered_relationships o_r ON deo.previous = o_r.id "
                               "  AND %2 = :entityId"
                               ")"
                               "SELECT %3 FROM ordered_relationships ORDER BY row_number")
                           .arg(m_junctionTableName, m_junctionTableLeftEntityForeignKeyName,
                                m_junctionTableRightEntityForeignKeyName);

    QSqlQuery query(connection);

    if (!query.prepare(queryStr))
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error_prepare", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId", leftEntityId);

    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    QList<int> rightEntityIds;
    while (query.next())
    {
        rightEntityIds.append(query.value(0).toInt());
    }
    return getRightEntitiesFromTheirIds(rightEntityIds);
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociator<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    // find all the right entities that are already associated with the left entity, then compare them with the new
    // ones, and delete the ones that are not in the new list. Then add the new ones.
    auto connection = m_databaseContext->getConnection();
    QSqlQuery query(connection);
    QString queryStr = "SELECT id, " + m_junctionTableRightEntityForeignKeyName + ", previous, next FROM " +
                       m_junctionTableName + " WHERE " + m_junctionTableLeftEntityForeignKeyName + " = :entityId";
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId", leftEntityId);
    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    QList<int> originalRightEntityIds;
    QList<EntityShadow> originalShadows;
    int order = 0;
    while (query.next())
    {
        originalRightEntityIds.append(query.value(1).toInt());
        originalShadows.append(EntityShadow(query.value(0).toInt(), query.value(1).toInt(), order++,
                                            query.value(2).toInt(), query.value(3).toInt()));
    }

    // create new shadow list
    QList<EntityShadow> newShadows;
    for (int i = 0; i < rightEntities.size(); ++i)
    {
        newShadows.append(EntityShadow(0, rightEntities[i].id(), i, 0, 0));
    }

    // merge shadows list, setting create, remove, updatePrevious, updateNext
    QList<EntityShadow> mergedShadows = mergeShadows(originalShadows, newShadows);

    for (const EntityShadow &shadow : mergedShadows)
    {
        // create new junction table rows
        if (shadow.create)
        {
            queryStr =
                QString("INSERT INTO %1 (%2, %3, previous, next) VALUES (:entityId, :rightEntityId, :previous, :next)")
                    .arg(m_junctionTableName, m_junctionTableLeftEntityForeignKeyName,
                         m_junctionTableRightEntityForeignKeyName);
            query.prepare(queryStr);
            query.bindValue(":entityId", leftEntityId);
            query.bindValue(":rightEntityId", shadow.entityId);
            query.bindValue(":previous", shadow.previous == 0 ? QVariant() : shadow.previous);
            query.bindValue(":next", shadow.next == 0 ? QVariant() : shadow.next);
            if (!query.exec())
            {
                return Result<QList<RightEntity>>(
                    QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
        }
        // remove old junction table rows

        if (shadow.remove)
        {

            queryStr = QString("DELETE FROM %1 WHERE id = :junctionId").arg(m_junctionTableName);
            query.prepare(queryStr);
            query.bindValue(":junctionId", shadow.junctionTableId);
            if (!query.exec())
            {
                return Result<QList<RightEntity>>(
                    QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
        }
        // update junction table rows
        if (shadow.updatePreviousOrNext)
        {
            queryStr = QString("UPDATE %1 SET previous = :previous, next = :next WHERE id = :junctionId")
                           .arg(m_junctionTableName);
            query.prepare(queryStr);
            query.bindValue(":junctionId", shadow.junctionTableId);
            query.bindValue(":previous", shadow.previous == 0 ? QVariant() : shadow.previous);
            query.bindValue(":next", shadow.next == 0 ? QVariant() : shadow.next);
            if (!query.exec())
            {
                return Result<QList<RightEntity>>(
                    QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
        }
    }

    return getRightEntities(leftEntityId);
}

//--------------------------------------------

template <class RightEntity>
Result<QList<RightEntity>> OneToManyOrderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
    if (rightEntityIds.isEmpty())
    {
        return Result<QList<RightEntity>>(QList<RightEntity>());
    }

    const QStringList &columns = getTablePropertyColumns(m_rightEntitySchema);

    QSqlDatabase database = m_databaseContext->getConnection();
    QHash<QString, QVariant> columnWithValues;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    QList<RightEntity> rightEntities;

    QString queryStr = "SELECT " + fields + " FROM " + m_rightEntityForeignTableName + " WHERE " + "id IN (";
    for (int i = 0; i < rightEntityIds.count(); i++)
    {
        queryStr += ":id" + QString::number(i) + ",";
    }
    queryStr.chop(1);

    queryStr += ")";
    QSqlQuery query(database);
    if (!query.prepare(queryStr))
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    for (int i = 0; i < rightEntityIds.count(); i++)
    {
        query.bindValue(":id" + QString::number(i), QVariant(rightEntityIds.at(i)));
    }
    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    if (query.lastError().isValid())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    while (query.next())
    {
        for (int i = 0; i < columns.count(); i++)
        {
            columnWithValues.insert(columns.at(i), query.value(i));
        }
        rightEntities.append(TableTools<RightEntity>::mapToEntity(columnWithValues).value());
    }

    return Result<QList<RightEntity>>(rightEntities);
}

//--------------------------------------------

template <class RightEntity>
QStringList OneToManyOrderedAssociator<RightEntity>::getTablePropertyColumns(
    const Qleany::Domain::EntitySchema &entitySchema) const
{
    QStringList columns;

    for (const auto &field : entitySchema.fields)
    {
        if (field.isLinkedToAnotherEntity)
        {
            continue;
        }
        columns.append(Tools::fromPascalToSnakeCase(field.name));
    }

    return columns;
}

template <class RightEntity>
QList<typename OneToManyOrderedAssociator<RightEntity>::EntityShadow> OneToManyOrderedAssociator<
    RightEntity>::mergeShadows(const QList<EntityShadow> &originalShadows, const QList<EntityShadow> &newShadows) const
{
    QList<EntityShadow> originalShadowsClone = originalShadows;
    QList<EntityShadow> newShadowsClone = newShadows;

    // if newShadow is empty, return originalShadows with all the entities marked as deleted
    if (newShadowsClone.isEmpty())
    {
        QList<EntityShadow> shadows = originalShadowsClone;
        for (auto &shadow : shadows)
        {
            shadow.remove = true;
        }
        return markUpdatePreviousOrNext(writePreviousAndNext(shadows));
    }

    // if originalShadows is empty, return newShadows with all the entities marked as create
    if (originalShadowsClone.isEmpty())
    {
        QList<EntityShadow> shadows = newShadowsClone;
        for (auto &shadow : shadows)
        {
            shadow.create = true;
        }
        return markUpdatePreviousOrNext(writePreviousAndNext(shadows));
    }

    // both lists are empty, return empty list
    if (originalShadowsClone.isEmpty() && newShadowsClone.isEmpty())
    {
        return QList<EntityShadow>();
    }

    // find the first common entity in the two shadows lists, to use as position reference
    EntityShadow firstCommonShadow;
    for (const auto &shadow : originalShadowsClone)
    {
        if (newShadowsClone.contains(shadow))
        {
            firstCommonShadow = shadow;
            break;
        }
    }

    // if no common entity is found, return newShadows with all the entities marked as create and all the others marked
    // as delete
    if (firstCommonShadow.isNull())
    {
        QList<EntityShadow> shadows = newShadowsClone;
        for (auto &shadow : shadows)
        {
            shadow.create = true;
        }
        shadows.append(originalShadowsClone);
        for (auto &shadow : shadows)
        {
            if (!shadow.create)
            {
                shadow.remove = true;
            }
        }
        return markUpdatePreviousOrNext(writePreviousAndNext(shadows));
    }

    // mark as common in originalShadows and newShadows all the entities that are present in both originalShadows and
    // newShadows

    for (auto &shadow : originalShadowsClone)
    {
        if (newShadowsClone.contains(shadow))
        {
            shadow.common = true;
        }
    }

    // fill common newShadows of the information from originalShadows
    for (auto &shadow : originalShadowsClone)
    {

        if (shadow.common)
        {
            EntityShadow &tempShadow = newShadowsClone[newShadowsClone.indexOf(shadow)];
            tempShadow.junctionTableId = shadow.junctionTableId;
            tempShadow.entityId = shadow.entityId;
            tempShadow.next = shadow.next;
            tempShadow.previous = shadow.previous;
            tempShadow.common = true;
        }
    }

    // mark as deleted all the entities that are present in originalShadows but not in newShadows
    for (auto &originalShadow : originalShadowsClone)
    {
        if (!newShadowsClone.contains(originalShadow))
        {
            originalShadow.remove = true;
        }
    }

    // mark as created all the entities that are present in newShadows but not in originalShadows
    for (auto &newShadow : newShadowsClone)
    {
        if (!originalShadowsClone.contains(newShadow))
        {
            newShadow.create = true;
        }
    }
    // keept the new shadows as base
    QList<EntityShadow> mergedShadows = writePreviousAndNext(newShadowsClone);

    // add the removed entities at the end of the merged shadows
    for (auto &originalShadow : originalShadowsClone)
    {
        if (originalShadow.remove == true)
        {
            mergedShadows.append(originalShadow);
        }
    }

    return markUpdatePreviousOrNext(mergedShadows);
}

template <class RightEntity>
QList<typename OneToManyOrderedAssociator<RightEntity>::EntityShadow> OneToManyOrderedAssociator<
    RightEntity>::writePreviousAndNext(const QList<EntityShadow> &shadows) const
{
    QList<EntityShadow> writtenShadows = shadows;

    // calculate the newPrevious and newNext for all the entities in mergedShadows but removed ones
    for (int i = 0; i < writtenShadows.size(); i++)
    {
        if (writtenShadows[i].remove == false)
        {
            if (i == 0)
            {
                writtenShadows[i].newPrevious = 0;
            }
            else
            {
                writtenShadows[i].newPrevious = writtenShadows[i - 1].entityId;
            }
            if (i == writtenShadows.size() - 1)
            {
                writtenShadows[i].newNext = 0;
            }
            else
            {
                writtenShadows[i].newNext = writtenShadows[i + 1].entityId;
            }
        }
    }
    return writtenShadows;
}

template <class RightEntity>
QList<typename OneToManyOrderedAssociator<RightEntity>::EntityShadow> OneToManyOrderedAssociator<
    RightEntity>::markUpdatePreviousOrNext(const QList<EntityShadow> &shadows) const
{
    QList<EntityShadow> writtenShadows = shadows;

    // mark updatePreviousOrNext
    for (auto &shadow : writtenShadows)
    {
        if (shadow.remove == false)
        {
            if (shadow.previous != shadow.newPrevious || shadow.next != shadow.newNext)
            {
                shadow.updatePreviousOrNext = true;
                shadow.previous = shadow.newPrevious;
                shadow.next = shadow.newNext;
            }
        }
    }

    return writtenShadows;
}

template <class RightEntity>
Result<void> OneToManyOrderedAssociator<RightEntity>::removeTheseRightIds(QList<int> rightEntityIds)
{
    if (rightEntityIds.isEmpty())
    {
        return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "rigntentitty_empty", "rightEntityIds is empty"));
    }

    struct RemovalShadow
    {
        int id;
        int rightEntityId;
        int previous;
        int next;
        bool done = false;
    };

    struct RemovalShadowGroup
    {
        int rightEntityId = 0;
        QList<RemovalShadow> shadows;
        int previous = 0;
        int next = 0;
    };

    QList<RemovalShadow> shadows;
    shadows.reserve(rightEntityIds.size());

    // fetch with SQL the previous and next for each line with rightEntityId

    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);
    QString queryString = QString("SELECT id, %1, previous, next FROM %2 WHERE %1 IN (")
                              .arg(m_junctionTableRightEntityForeignKeyName, m_junctionTableName);
    for (int i = 0; i < rightEntityIds.size(); i++)
    {
        if (i != 0)
        {
            queryString += ", ";
        }
        queryString += QString::number(rightEntityIds[i]);
    }
    queryString += ")";
    if (!query.exec(queryString))
    {
        return Result<void>(
            QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error", query.lastError().text()));
    }

    // fill shadows with the fetched data
    while (query.next())
    {
        shadows.append({.id = query.value(0).toInt(),
                        .rightEntityId = query.value(1).toInt(),
                        .previous = query.value(2).toInt(),
                        .next = query.value(3).toInt()});
    }

    // divide RemovalShadow into RemovalShadowGroup by groups of adjacent RemovalShadows, an adjacent RemovalShadow is a
    // RemovalShadow with next == id of the next RemovalShadow
    QList<RemovalShadowGroup> shadowGroups;
    shadowGroups.reserve(shadows.size());

    for (int i = 0; i < shadows.size(); i++)
    {
        int previousToFind = 0;
        int nextToFind = 0;

        if (shadows[i].done == false)
        {
            shadowGroups.append({shadows[i].rightEntityId, {shadows[i]}, shadows[i].previous, shadows[i].next});
            previousToFind = shadows[i].previous;
            nextToFind = shadows[i].next;
            shadows[i].done = true;

            for (int k = 0; k > shadows.size(); k++)
            {
                if (shadows[k].done == false)
                {
                    if (shadows[k].previous == nextToFind && nextToFind != 0)
                    {
                        shadowGroups.last().shadows.append(shadows[k]);
                        shadowGroups.last().next = shadows[k].next;
                        shadows[k].done = true;
                        nextToFind = shadows[k].next;
                    }
                    else if (shadows[k].next == previousToFind && previousToFind != 0)
                    {
                        shadowGroups.last().shadows.prepend(shadows[k]);
                        shadowGroups.last().previous = shadows[k].previous;
                        shadows[k].done = true;
                        previousToFind = shadows[k].previous;
                    }
                }
            }
        }
    }

    // remove the shadows from the database

    for (auto &shadowGroup : shadowGroups)
    {
        QString queryString = QString("DELETE FROM %1 WHERE id IN (").arg(m_junctionTableName);
        for (int i = 0; i < shadowGroup.shadows.size(); i++)
        {
            if (i != 0)
            {
                queryString += ", ";
            }
            queryString += QString::number(shadowGroup.shadows[i].id);
        }
        queryString += ")";
        if (!query.exec(queryString))
        {
            return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error",
                                            query.lastError().text()));
        }
    }

    // update the previous and next of the shadows in the database. Previous entity of a group must take the "next" of
    // the group. Next entity of a group must take the "previous" of the group.
    for (auto &shadowGroup : shadowGroups)
    {

        // previous :
        if (shadowGroup.previous != 0)
        {
            queryString = QString("UPDATE %1 SET next = :next WHERE id = :id").arg(m_junctionTableName);
            if (!query.prepare(queryString))
            {
                return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error",
                                                query.lastError().text()));
            }
            query.bindValue(":next", shadowGroup.next == 0 ? QVariant(QMetaType::fromType<int>()) : shadowGroup.next);
            query.bindValue(":id", shadowGroup.previous);

            if (!query.exec())
            {
                qDebug() << query.lastQuery();
                qDebug() << query.boundValues();
                qDebug() << query.lastError();
                return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error",
                                                query.lastError().text()));
            }
        }
        if (shadowGroup.next != 0)
        {
            // next :
            queryString = QString("UPDATE %1 SET previous = :previous WHERE id = :id").arg(m_junctionTableName);
            if (!query.prepare(queryString))
            {
                return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error",
                                                query.lastError().text()));
            }
            query.bindValue(":previous",
                            shadowGroup.previous == 0 ? QVariant(QMetaType::fromType<int>()) : shadowGroup.previous);
            query.bindValue(":id", shadowGroup.next);

            if (!query.exec())
            {
                return Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Status::Fatal, "association-removal-sql-error",
                                                query.lastError().text()));
            }
        }
    }

    return Result<void>();
}

} // namespace Qleany::Database

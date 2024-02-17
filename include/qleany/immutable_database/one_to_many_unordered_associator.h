#pragma once

#include "qleany/contracts/immutable_database/interface_database_context.h"
#include "qleany/qleany_global.h"
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
template <class RightEntity> class OneToManyUnorderedAssociator
{
  public:
    OneToManyUnorderedAssociator(QSharedPointer<Qleany::Contracts::ImmutableDatabase::InterfaceDatabaseContext> context,
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
    ~OneToManyUnorderedAssociator() = default;
    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);

  private:
    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const Qleany::Domain::EntitySchema &entitySchema) const;
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
};

template <class RightEntity> QString OneToManyUnorderedAssociator<RightEntity>::getTableCreationSql() const
{
    return "CREATE TABLE IF NOT EXISTS " + m_junctionTableName +
           " (id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON "
           "CONFLICT ROLLBACK," +
           m_junctionTableLeftEntityForeignKeyName + " INTEGER NOT NULL, " + m_junctionTableRightEntityForeignKeyName +
           " INTEGER NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, FOREIGN KEY (" +
           m_junctionTableLeftEntityForeignKeyName + ") REFERENCES " + m_leftEntityForeignTableName +
           " (id) ON DELETE CASCADE, FOREIGN KEY (" + m_junctionTableRightEntityForeignKeyName + ") REFERENCES " +
           m_rightEntityForeignTableName + " (id) ON DELETE CASCADE, " + "UNIQUE (" +
           m_junctionTableLeftEntityForeignKeyName + ", " + m_junctionTableRightEntityForeignKeyName +
           ") ON CONFLICT ROLLBACK" + ");";
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyUnorderedAssociator<RightEntity>::getRightEntities(int leftEntityId)
{
    auto connection = m_databaseContext->getConnection();

    QSqlQuery query(connection);
    QString queryStr = "SELECT " + m_junctionTableRightEntityForeignKeyName + " FROM " + m_junctionTableName +
                       " WHERE " + m_junctionTableLeftEntityForeignKeyName + " = :entityId";
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
    QList<int> rightEntityIds;
    while (query.next())
    {
        rightEntityIds.append(query.value(0).toInt());
    }
    return getRightEntitiesFromTheirIds(rightEntityIds);
}

template <class RightEntity>
Result<QList<RightEntity>> OneToManyUnorderedAssociator<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    // find all the right entities that are already associated with the left entity, then compare them with the new
    // ones, and delete the ones that are not in the new list. Then add the new ones.
    auto connection = m_databaseContext->getConnection();
    QSqlQuery query(connection);
    QString queryStr = "SELECT " + m_junctionTableRightEntityForeignKeyName + " FROM " + m_junctionTableName +
                       " WHERE " + m_junctionTableLeftEntityForeignKeyName + " = :entityId";
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
    QList<int> rightEntityIds;
    while (query.next())
    {
        rightEntityIds.append(query.value(0).toInt());
    }
    // delete the ones that are not in the new list
    for (int rightEntityId : rightEntityIds)
    {
        if (!rightEntities.contains(rightEntityId))
        {
            QSqlQuery deleteQuery(connection);
            QString deleteQueryStr = "DELETE FROM " + m_junctionTableName + " WHERE " +
                                     m_junctionTableLeftEntityForeignKeyName + " = :leftEntityId AND " +
                                     m_junctionTableRightEntityForeignKeyName + " = :rightEntityId";
            deleteQuery.prepare(deleteQueryStr);
            deleteQuery.bindValue(":leftEntityId", leftEntityId);
            deleteQuery.bindValue(":rightEntityId", rightEntityId);
            if (!deleteQuery.exec())
            {
                return Result<QList<RightEntity>>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                              deleteQuery.lastError().text(), deleteQueryStr));
            }
        }
    }
    // add the new ones

    for (int rightEntityId : rightEntityIds)
    {
        QSqlQuery insertQuery(connection);
        QString insertQueryStr = "INSERT INTO " + m_junctionTableName + " (" + m_junctionTableLeftEntityForeignKeyName +
                                 ", " + m_junctionTableRightEntityForeignKeyName +
                                 ") VALUES (:leftEntityId, :rightEntityId)";
        insertQuery.prepare(insertQueryStr);
        insertQuery.bindValue(":leftEntityId", leftEntityId);
        insertQuery.bindValue(":rightEntityId", rightEntityId);
        if (!insertQuery.exec())
        {
            return Result<QList<RightEntity>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertQueryStr));
        }
    }
    return getRightEntities(leftEntityId);
}

//--------------------------------------------

template <class RightEntity>
Result<QList<RightEntity>> OneToManyUnorderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
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
QStringList OneToManyUnorderedAssociator<RightEntity>::getTablePropertyColumns(
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
} // namespace Qleany::Database

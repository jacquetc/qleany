// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database/interface_database_context.h"
#include "tools.h"
#include <QList>
#include <QSharedPointer>
#include <QSqlError>
#include <QSqlQuery>
#include "result.h"
#include "entity_schema.h"

using namespace DatabaseTest::Contracts::Database;

namespace DatabaseTest::Persistence::Database
{
template <class RightEntity> class ManyToManyUnorderedAssociator
{
  public:
    ManyToManyUnorderedAssociator(QSharedPointer<InterfaceDatabaseContext> context,
                                 const DatabaseTest::Entities::EntitySchema::RelationshipInfo &relationship)
        : m_databaseContext(context), m_relationship(relationship), m_fieldName(relationship.fieldName)
    {

        QString leftEntityName = relationship.leftEntityName;
        QString rightEntityName = RightEntity::schema.name;

        m_junctionTableName =
            leftEntityName + "_"_L1 + relationship.fieldName + "_"_L1 + rightEntityName + "_junction"_L1;
        m_junctionTableLeftEntityForeignKeyName = leftEntityName + "_id"_L1;
        m_leftEntityForeignTableName = DatabaseTest::Persistence::Database::Tools::fromPascalToSnakeCase(leftEntityName);
        m_junctionTableRightEntityForeignKeyName = rightEntityName + "_id"_L1;
        m_rightEntityForeignTableName = DatabaseTest::Persistence::Database::TableTools<RightEntity>::getEntityTableName();
    }
    ~ManyToManyUnorderedAssociator() = default;
    Result<QList<RightEntity>> getRightEntities(int leftEntityId);

    QString getTableCreationSql() const;
    Result<QList<RightEntity>> updateRightEntities(int leftEntityId, const QList<RightEntity> &rightEntities);

  private:
    Result<QList<RightEntity>> getRightEntitiesFromTheirIds(QList<int> rightEntityIds) const;
    QStringList getTablePropertyColumns(const DatabaseTest::Entities::EntitySchema::EntitySchema &entitySchema) const;
    QSharedPointer<InterfaceDatabaseContext>
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this
                            * DatabaseTableGroup.
                            */

    QString m_junctionTableName;
    QString m_junctionTableLeftEntityForeignKeyName;
    QString m_leftEntityForeignTableName;
    QString m_junctionTableRightEntityForeignKeyName;
    QString m_rightEntityForeignTableName;
    DatabaseTest::Entities::EntitySchema::RelationshipInfo m_relationship;
    DatabaseTest::Entities::EntitySchema::EntitySchema m_rightEntitySchema = RightEntity::schema;
    const QStringList m_rightEntityPropertyColumns = getTablePropertyColumns(RightEntity::schema);
    DatabaseTest::Entities::EntitySchema::EntitySchema m_leftEntitySchema;
    QString m_fieldName;
};

template <class RightEntity> QString ManyToManyUnorderedAssociator<RightEntity>::getTableCreationSql() const
{
    return "CREATE TABLE IF NOT EXISTS %1"
           " (id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON "
           "CONFLICT ROLLBACK, %2 INTEGER NOT NULL, "
           "%3 INTEGER NOT NULL ON CONFLICT ROLLBACK, FOREIGN KEY (%4) REFERENCES %5"
           " (id) ON DELETE CASCADE, FOREIGN KEY (%6) REFERENCES %7 (id) ON DELETE CASCADE, "
           "UNIQUE (%8, %9) ON CONFLICT ROLLBACK);"_L1.arg(
               m_junctionTableName, m_junctionTableLeftEntityForeignKeyName, m_junctionTableRightEntityForeignKeyName,
               m_junctionTableLeftEntityForeignKeyName, m_leftEntityForeignTableName,
               m_junctionTableRightEntityForeignKeyName, m_rightEntityForeignTableName,
               m_junctionTableLeftEntityForeignKeyName, m_junctionTableRightEntityForeignKeyName);
}

template <class RightEntity>
Result<QList<RightEntity>> ManyToManyUnorderedAssociator<RightEntity>::getRightEntities(int leftEntityId)
{
    auto connection = m_databaseContext->getConnection();

    QSqlQuery query(connection);
    QString queryStr = "SELECT "_L1 + m_junctionTableRightEntityForeignKeyName + " FROM "_L1 + m_junctionTableName +
                       " WHERE "_L1 + m_junctionTableLeftEntityForeignKeyName + " = :entityId"_L1;
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId"_L1, leftEntityId);
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
Result<QList<RightEntity>> ManyToManyUnorderedAssociator<RightEntity>::updateRightEntities(
    int leftEntityId, const QList<RightEntity> &rightEntities)
{
    // find all the right entities that are already associated with the left entity, then compare them with the new
    // ones, and delete the ones that are not in the new list. Then add the new ones.
    auto connection = m_databaseContext->getConnection();
    QSqlQuery query(connection);
    QString queryStr = "SELECT "_L1 + m_junctionTableRightEntityForeignKeyName + " FROM "_L1 + m_junctionTableName +
                       " WHERE "_L1 + m_junctionTableLeftEntityForeignKeyName + " = :entityId"_L1;
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId"_L1, leftEntityId);
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
            QString deleteQueryStr = "DELETE FROM "_L1 + m_junctionTableName + " WHERE "_L1 +
                                     m_junctionTableLeftEntityForeignKeyName + " = :leftEntityId AND "_L1 +
                                     m_junctionTableRightEntityForeignKeyName + " = :rightEntityId"_L1;
            deleteQuery.prepare(deleteQueryStr);
            deleteQuery.bindValue(":leftEntityId"_L1, leftEntityId);
            deleteQuery.bindValue(":rightEntityId"_L1, rightEntityId);
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
        QString insertQueryStr =
            "INSERT INTO "_L1 + m_junctionTableName + " ("_L1 + m_junctionTableLeftEntityForeignKeyName + ", "_L1 +
            m_junctionTableRightEntityForeignKeyName + ") VALUES (:leftEntityId, :rightEntityId)"_L1;
        insertQuery.prepare(insertQueryStr);
        insertQuery.bindValue(":leftEntityId"_L1, leftEntityId);
        insertQuery.bindValue(":rightEntityId"_L1, rightEntityId);
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
Result<QList<RightEntity>> ManyToManyUnorderedAssociator<RightEntity>::getRightEntitiesFromTheirIds(
    QList<int> rightEntityIds) const
{
    const QStringList &columns = getTablePropertyColumns(m_rightEntitySchema);

    QSqlDatabase database = m_databaseContext->getConnection();
    QHash<QString, QVariant> columnWithValues;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ","_L1;
    }
    fields.chop(1);

    QList<RightEntity> rightEntities;

    QString queryStr =
        "SELECT "_L1 + fields + " FROM "_L1 + m_rightEntityForeignTableName + " WHERE "_L1 + "id IN ("_L1;
    for (int i = 0; i < rightEntityIds.count(); i++)
    {
        queryStr += ":id"_L1 + QString::number(i) + ","_L1;
    }
    queryStr.chop(1);

    queryStr += ")"_L1;
    QSqlQuery query(database);
    if (!query.prepare(queryStr))
    {
        return Result<QList<RightEntity>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    for (int i = 0; i < rightEntityIds.count(); i++)
    {
        query.bindValue(":id"_L1 + QString::number(i), QVariant(rightEntityIds.at(i)));
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
QStringList ManyToManyUnorderedAssociator<RightEntity>::getTablePropertyColumns(
    const DatabaseTest::Entities::EntitySchema::EntitySchema &entitySchema) const
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
} // namespace DatabaseTest::Persistence::Database
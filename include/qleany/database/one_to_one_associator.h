#pragma once

#include "qleany/contracts/database/interface_database_context.h"
#include "qleany/qleany_export.h"
#include "tools.h"
#include <QList>
#include <QSharedPointer>
#include <QSqlError>
#include <QSqlQuery>
#include <qleany/common/result.h>
#include <qleany/entities/entity_schema.h>

using namespace Qleany::Contracts::Database;

namespace Qleany::Database
{
template <class RightEntity> class OneToOneAssociator
{
  public:
    OneToOneAssociator(QSharedPointer<InterfaceDatabaseContext> context,
                       const Qleany::Entities::RelationshipInfo &relationship)
        : m_databaseContext(context), m_relationship(relationship), m_fieldName(relationship.fieldName)
    {

        QString leftEntityLastName = relationship.leftEntityName;
        QString rightEntityLastName = RightEntity::schema.name;

        m_junctionTableName =
            leftEntityLastName + "_"_L1 + relationship.fieldName + "_"_L1 + rightEntityLastName + "_junction"_L1;
        m_junctionTableLeftEntityForeignKeyName = leftEntityLastName + "_id"_L1;
        m_leftEntityForeignTableName = Qleany::Database::Tools::fromPascalToSnakeCase(leftEntityLastName);
        m_junctionTableRightEntityForeignKeyName = rightEntityLastName + "_id"_L1;
        m_rightEntityForeignTableName = Qleany::Database::TableTools<RightEntity>::getEntityTableName();
    }
    ~OneToOneAssociator() = default;
    Result<RightEntity> getRightEntity(int leftEntityId);

    QString getTableCreationSql() const;
    Result<RightEntity> updateRightEntity(int leftEntityId, const RightEntity &rightEntity);

  private:
    Result<RightEntity> getRightEntityFromItsId(int rightEntityId) const;
    QStringList getTablePropertyColumns(const Qleany::Entities::EntitySchema &entitySchema) const;
    QSharedPointer<InterfaceDatabaseContext>
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this
                            * DatabaseTableGroup.
                            */

    QString m_junctionTableName;
    QString m_junctionTableLeftEntityForeignKeyName;
    QString m_leftEntityForeignTableName;
    QString m_junctionTableRightEntityForeignKeyName;
    QString m_rightEntityForeignTableName;
    Qleany::Entities::RelationshipInfo m_relationship;
    Qleany::Entities::EntitySchema m_rightEntitySchema = RightEntity::schema;
    const QStringList m_rightEntityPropertyColumns = getTablePropertyColumns(RightEntity::schema);
    QString m_fieldName;
};

template <class RightEntity> QString OneToOneAssociator<RightEntity>::getTableCreationSql() const
{
    // create a table with a ont to one relationship between entity and other entity foreign ids , enforce it by
    // constraints

    return "CREATE TABLE IF NOT EXISTS %1"
           " (id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON "
           "CONFLICT ROLLBACK, %2"
           " INTEGER NOT NULL, %3"
           " INTEGER NOT NULL, FOREIGN KEY (%4) REFERENCES %5 (id) ON DELETE CASCADE, FOREIGN KEY (%6) REFERENCES %7 "
           "(id) ON DELETE CASCADE, UNIQUE (%8"
           ") ON CONFLICT ROLLBACK);"_L1.arg(m_junctionTableName, m_junctionTableLeftEntityForeignKeyName,
                                             m_junctionTableRightEntityForeignKeyName,
                                             m_junctionTableLeftEntityForeignKeyName, m_leftEntityForeignTableName,
                                             m_junctionTableRightEntityForeignKeyName, m_rightEntityForeignTableName,
                                             m_junctionTableLeftEntityForeignKeyName);
}

template <class RightEntity> Result<RightEntity> OneToOneAssociator<RightEntity>::getRightEntity(int leftEntityId)
{
    auto connection = m_databaseContext->getConnection();

    QSqlQuery query(connection);
    QString queryStr = "SELECT "_L1 + m_junctionTableRightEntityForeignKeyName + " FROM "_L1 + m_junctionTableName +
                       " WHERE "_L1 + m_junctionTableLeftEntityForeignKeyName + " = :entityId"_L1;
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId"_L1, leftEntityId);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    if (!query.next())
    {
        return Result<RightEntity>(RightEntity());
    }
    int otherEntityId = query.value(0).toInt();

    // get the entity from the database

    return getRightEntityFromItsId(otherEntityId);
}

template <class RightEntity>
Result<RightEntity> OneToOneAssociator<RightEntity>::updateRightEntity(int leftEntityId, const RightEntity &rightEntity)
{
    auto connection = m_databaseContext->getConnection();

    // if left entity foreign id is already used in the junction table, then update the right entity foreign key, else
    // insert them. If rightEntity.id() is 0, then it's invalid and the line with leftEntityId must be cleared

    if (rightEntity.id() == 0)
    {

        // remove line with leftEntityId if it exists in the junction table

        QSqlQuery deleteQuery(connection);
        QString deleteQueryStr = "DELETE FROM "_L1 + m_junctionTableName + " WHERE "_L1 +
                                 m_junctionTableLeftEntityForeignKeyName + " = :leftEntityId"_L1;
        deleteQuery.prepare(deleteQueryStr);
        if (!deleteQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", deleteQuery.lastError().text(), deleteQueryStr));
        }
        deleteQuery.bindValue(":leftEntityId"_L1, leftEntityId);
        if (!deleteQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", deleteQuery.lastError().text(), deleteQueryStr));
        }
        return Result<RightEntity>();
    }

    // search for the leftEntityId in the junction table

    QSqlQuery query(connection);
    QString queryStr = "SELECT "_L1 + m_junctionTableRightEntityForeignKeyName + " FROM "_L1 + m_junctionTableName +
                       " WHERE "_L1 + m_junctionTableLeftEntityForeignKeyName + " = :entityId"_L1;
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId"_L1, leftEntityId);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    if (!query.next())
    {
        // insert the right entity foreign id
        QSqlQuery insertQuery(connection);
        QString insertQueryStr =
            "INSERT INTO "_L1 + m_junctionTableName + " ("_L1 + m_junctionTableLeftEntityForeignKeyName + ", "_L1 +
            m_junctionTableRightEntityForeignKeyName + ") VALUES (:leftEntityId, "_L1 + ":rightEntityId)"_L1;
        insertQuery.prepare(insertQueryStr);
        if (!insertQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertQueryStr));
        }
        insertQuery.bindValue(":leftEntityId"_L1, leftEntityId);
        insertQuery.bindValue(":rightEntityId"_L1, rightEntity.id());
        if (!insertQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertQueryStr));
        }
        if (insertQuery.lastError().isValid())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertQueryStr));
        }
    }
    else
    {
        // update the right entity foreign id
        QSqlQuery updateQuery(connection);
        QString updateQueryStr = "UPDATE "_L1 + m_junctionTableName + " SET "_L1 +
                                 m_junctionTableRightEntityForeignKeyName + " = :rightEntityId WHERE "_L1 +
                                 m_junctionTableLeftEntityForeignKeyName + " = :leftEntityId"_L1;
        updateQuery.prepare(updateQueryStr);
        if (!updateQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text(), updateQueryStr));
        }
        updateQuery.bindValue(":leftEntityId"_L1, leftEntityId);
        updateQuery.bindValue(":rightEntityId"_L1, rightEntity.id());
        if (!updateQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text(), updateQueryStr));
        }
        if (updateQuery.lastError().isValid())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text(), updateQueryStr));
        }
    }

    return getRightEntity(leftEntityId);
}

template <class RightEntity>
Result<RightEntity> OneToOneAssociator<RightEntity>::getRightEntityFromItsId(int rightEntityId) const
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

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + m_rightEntityForeignTableName + " WHERE id = :id"_L1;
        if (!query.prepare(queryStr))
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id"_L1, QVariant(rightEntityId));
        if (!query.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        while (query.next())
        {
            for (int i = 0; i < columns.count(); i++)
            {
                columnWithValues.insert(columns.at(i), query.value(i));
            }
        }
        if (columnWithValues.isEmpty())
        {
            return Result<RightEntity>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_row_missing",
                                                   "No row with id "_L1 + QString::number(rightEntityId)));
        }
    }

    return TableTools<RightEntity>::mapToEntity(columnWithValues);
}
//--------------------------------------------

template <class RightEntity>
QStringList OneToOneAssociator<RightEntity>::getTablePropertyColumns(
    const Qleany::Entities::EntitySchema &entitySchema) const
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

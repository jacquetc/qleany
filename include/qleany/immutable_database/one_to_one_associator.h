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
template <class RightEntity> class OneToOneAssociator
{
  public:
    OneToOneAssociator(QSharedPointer<Qleany::Contracts::ImmutableDatabase::InterfaceDatabaseContext> context,
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
    ~OneToOneAssociator() = default;
    Result<RightEntity> getRightEntity(int leftEntityId);

    QString getTableCreationSql() const;
    Result<RightEntity> updateRightEntity(int leftEntityId, const RightEntity &rightEntity);

  private:
    Result<RightEntity> getRightEntityFromItsId(int rightEntityId) const;
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
    QString m_fieldName;
};

template <class RightEntity> QString OneToOneAssociator<RightEntity>::getTableCreationSql() const
{
    // create a table with a ont to one relationship between entity and other entity foreign ids , enforce it by
    // constraints

    return "CREATE TABLE IF NOT EXISTS " + m_junctionTableName +
           " (id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON "
           "CONFLICT ROLLBACK, " +
           m_junctionTableLeftEntityForeignKeyName + " INTEGER NOT NULL, " + m_junctionTableRightEntityForeignKeyName +
           " INTEGER NOT NULL, FOREIGN KEY (" + m_junctionTableLeftEntityForeignKeyName + ") REFERENCES " +
           m_leftEntityForeignTableName + " (id) ON DELETE CASCADE, " + "FOREIGN KEY (" +
           m_junctionTableRightEntityForeignKeyName + ") REFERENCES " + m_rightEntityForeignTableName +
           " (id) ON DELETE CASCADE, " + "UNIQUE (" + m_junctionTableLeftEntityForeignKeyName +
           ") ON CONFLICT ROLLBACK" + ");";
}

template <class RightEntity> Result<RightEntity> OneToOneAssociator<RightEntity>::getRightEntity(int leftEntityId)
{
    auto connection = m_databaseContext->getConnection();

    QSqlQuery query(connection);
    QString queryStr = "SELECT " + m_junctionTableRightEntityForeignKeyName + " FROM " + m_junctionTableName +
                       " WHERE " + m_junctionTableLeftEntityForeignKeyName + " = :entityId";
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId", leftEntityId);
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
        QString deleteQueryStr = "DELETE FROM " + m_junctionTableName + " WHERE " +
                                 m_junctionTableLeftEntityForeignKeyName + " = :leftEntityId";
        deleteQuery.prepare(deleteQueryStr);
        if (!deleteQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", deleteQuery.lastError().text(), deleteQueryStr));
        }
        deleteQuery.bindValue(":leftEntityId", leftEntityId);
        if (!deleteQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", deleteQuery.lastError().text(), deleteQueryStr));
        }
        return Result<RightEntity>();
    }

    // search for the leftEntityId in the junction table

    QSqlQuery query(connection);
    QString queryStr = "SELECT " + m_junctionTableRightEntityForeignKeyName + " FROM " + m_junctionTableName +
                       " WHERE " + m_junctionTableLeftEntityForeignKeyName + " = :entityId";
    query.prepare(queryStr);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    query.bindValue(":entityId", leftEntityId);
    if (!query.exec())
    {
        return Result<RightEntity>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    if (!query.next())
    {
        // insert the right entity foreign id
        QSqlQuery insertQuery(connection);
        QString insertQueryStr = "INSERT INTO " + m_junctionTableName + " (" + m_junctionTableLeftEntityForeignKeyName +
                                 ", " + m_junctionTableRightEntityForeignKeyName + ") VALUES (:leftEntityId, " +
                                 ":rightEntityId)";
        insertQuery.prepare(insertQueryStr);
        if (!insertQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertQueryStr));
        }
        insertQuery.bindValue(":leftEntityId", leftEntityId);
        insertQuery.bindValue(":rightEntityId", rightEntity.id());
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
        QString updateQueryStr = "UPDATE " + m_junctionTableName + " SET " + m_junctionTableRightEntityForeignKeyName +
                                 " = :rightEntityId WHERE " + m_junctionTableLeftEntityForeignKeyName +
                                 " = :leftEntityId";
        updateQuery.prepare(updateQueryStr);
        if (!updateQuery.exec())
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text(), updateQueryStr));
        }
        updateQuery.bindValue(":leftEntityId", leftEntityId);
        updateQuery.bindValue(":rightEntityId", rightEntity.id());
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
        fields += column + ",";
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT " + fields + " FROM " + m_rightEntityForeignTableName + " WHERE " + "id = :id";
        if (!query.prepare(queryStr))
        {
            return Result<RightEntity>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id", QVariant(rightEntityId));
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
                                                   "No row with id " + QString::number(rightEntityId)));
        }
    }

    return TableTools<RightEntity>::mapToEntity(columnWithValues);
}
//--------------------------------------------

template <class RightEntity>
QStringList OneToOneAssociator<RightEntity>::getTablePropertyColumns(
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

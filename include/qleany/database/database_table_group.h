#pragma once

#include "QtCore/qdebug.h"
#include "QtCore/qmetaobject.h"
#include "one_to_many_ordered_associator.h"
#include "one_to_many_unordered_associator.h"
#include "one_to_one_associator.h"
#include "qleany/common/result.h"
#include "qleany/contracts/database/interface_database_context.h"
#include "qleany/contracts/database/interface_database_table_group.h"
#include "qleany/domain//entity_base.h"
#include "qleany/domain/entity_schema.h"
#include "qleany/qleany_global.h"
#include "tools.h"
#include <QDateTime>
#include <QMetaObject>
#include <QReadWriteLock>
#include <QRegularExpression>
#include <QSharedPointer>
#include <QSqlError>
#include <QSqlQuery>
#include <QTime>

using namespace Qleany::Contracts::Database;

struct PropertyWithList
{
    Q_GADGET
  public:
    //
    enum class ListType
    {
        Set,
        List
    };
    Q_ENUM(ListType)

    QString typeName;
    QString listTableName;
    ListType listType;
};

namespace Qleany::Database
{

template <class T> class QLEANY_EXPORT DatabaseTableGroup : public virtual InterfaceDatabaseTableGroup<T>
{
  public:
    explicit DatabaseTableGroup(QSharedPointer<InterfaceDatabaseContext> context);

    DatabaseTableGroup(const DatabaseTableGroup &other);

    Result<T> get(int id) override;
    Result<T> get(const QUuid &uuid) override;
    Result<QList<T>> get(const QList<int> &ids) override;

    Result<QList<T>> getAll() override;

    Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) override;

    Result<int> remove(int id) override;
    Result<QList<int>> remove(QList<int> ids) override;

    Result<QList<int>> changeActiveStatus(QList<int> ids, bool active) override;

    Result<T> add(T &&entity) override;

    Result<T> update(T &&entity) override;

    Result<bool> exists(const QUuid &uuid) override;
    Result<bool> exists(int id) override;
    Result<void> clear() override;

    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &saveData) override;
    Result<void> beginTransaction() override;
    Result<void> commit() override;
    Result<void> rollback() override;

    // get related entities
    Result<QList<T>> getEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int leftEntityId,
                                             const QString &field) override;
    Result<T> getEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int leftEntityId,
                                    const QString &field) override;
    Result<QList<T>> updateEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int leftEntityId,
                                                const QString &field, const QList<T> &rightEntities) override;
    Result<T> updateEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema, int leftEntityId,
                                       const QString &field, const T &rightEntity) override;

  protected:
    QSharedPointer<InterfaceDatabaseContext> databaseContext() const;
    QString tableName() const;
    QStringList properties() const;
    QStringList propertyColumns() const;

  private:
    QSharedPointer<InterfaceDatabaseContext>
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this
                            * DatabaseTableGroup.
                            */

    const QString m_tableName = TableTools<T>::getEntityTableName();
    const QStringList m_properties = TableTools<T>::getEntityProperties();
    const QStringList m_propertyColumns = TableTools<T>::getColumnNamesWithForeignKeys();
    const QStringList m_propertyColumnsWithoutForeignKeys = TableTools<T>::getColumnNamesWithoutForeignKeys();

    const QHash<QString, PropertyWithList> m_listProperties = this->getEntityPropertiesWithList();

    // list properties:
    QString getListTableName(const QString &listPropertyName, PropertyWithList::ListType type);
    bool isCommonType(int typeId);
    QHash<QString, PropertyWithList> getEntityPropertiesWithList();
    QString generateFilterQueryString(const QHash<QString, QVariant> &filters);

    QString getTableCreationSql() const;
};

//--------------------------------------------

template <class T>
DatabaseTableGroup<T>::DatabaseTableGroup(QSharedPointer<InterfaceDatabaseContext> context) : m_databaseContext(context)
{
    static_assert(std::is_base_of<Domain::EntityBase, T>::value, "T must inherit from Domain::Entity");

    // entity table creation
    m_databaseContext->appendCreationSql("entity_table", getTableCreationSql());

    for (const auto &relationship : T::schema.relationships)
    {
        if (relationship.direction == Qleany::Domain::RelationshipDirection::Backward)
        {
            QString junctionCreationSql;

            if (relationship.type == Qleany::Domain::RelationshipType::OneToOne)
            {
                OneToOneAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                     relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                     relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == Qleany::Domain::RelationshipType::ManyToMany)
            {
            }
        }
    }
}

//--------------------------------------------

template <class T>
DatabaseTableGroup<T>::DatabaseTableGroup(const DatabaseTableGroup &other) : m_databaseContext(other.databaseContext())
{
    static_assert(std::is_base_of<Domain::EntityBase, T>::value, "T must inherit from Domain::Entity");
}

template <class T> QString DatabaseTableGroup<T>::getTableCreationSql() const
{

    QString tableName = TableTools<T>::getEntityTableName();

    QString createTableSql = QString("CREATE TABLE %1 (").arg(tableName);

    QStringList relationshipPropertyNameListToIgnore;

    int propertyCount = T::staticMetaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {

        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        // ignore QList and QSet properties

        if (property.isReadable())
        {
            if (TableTools<T>::isForeign(propertyName))
            {

                relationshipPropertyNameListToIgnore.append(propertyName);
                relationshipPropertyNameListToIgnore.append(QString(propertyName) + "Loaded");
            }
        }
    }

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        // Ignore the "objectName" property
        if (strcmp(propertyName, "objectName") == 0)
        {
            continue;
        }

        if (relationshipPropertyNameListToIgnore.contains(property.name()))
        {
            continue;
        }

        int propertyMetaType = property.userType();
        const char *propertySqlType = Tools::qtMetaTypeToSqlType(propertyMetaType);

        if (propertySqlType)
        {
            createTableSql.append(QString("%1 %2").arg(Tools::fromPascalToSnakeCase(propertyName), propertySqlType));

            // Set uuid property as primary key, not null, and unique
            if (strcmp(propertyName, "id") == 0)
            {
                createTableSql.append(" PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT"
                                      " UNIQUE ON CONFLICT ROLLBACK"
                                      " NOT NULL ON CONFLICT ROLLBACK");
            }

            createTableSql.append(", ");
        }
        else
        {
            // Handle the case when an unsupported type is encountered
            QMetaType metaType(static_cast<QMetaType::Type>(propertyMetaType));
            qWarning("Unsupported property type for '%s': %s", propertyName, metaType.name());
        }
    }
    // remove last comma
    createTableSql.chop(2);

    createTableSql.append(");");

    return createTableSql;
}

template <class T> Result<T> DatabaseTableGroup<T>::get(int id)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;

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
        QString queryStr = "SELECT " + fields + " FROM " + entityName + " WHERE " + "id = :id";
        if (!query.prepare(queryStr))
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id", QVariant(id));
        if (!query.exec())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
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
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with id " + QString::number(id)));
        }
    }

    return TableTools<T>::mapToEntity(columnWithValues);
}

//--------------------------------------------

template <class T> Result<T> DatabaseTableGroup<T>::get(const QUuid &uuid)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;

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
        QString queryStr = "SELECT " + fields + " FROM " + entityName + " WHERE " + "uuid = :uuid";
        if (!query.prepare(queryStr))
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":uuid", QVariant(uuid));
        if (!query.exec())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
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
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with uuid " + uuid.toString()));
        }
    }

    return TableTools<T>::mapToEntity(columnWithValues);
}

/// @brief  get an unordered list of entities from a list of ids
/// @tparam T
/// @param ids
/// @return  Result<QList<T>>
template <class T> Result<QList<T>> DatabaseTableGroup<T>::get(const QList<int> &ids)
{

    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;

    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> listOfColumnsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT " + fields + " FROM " + entityName + " WHERE id IN (:ids)";
        QString idsString;
        for (int id : ids)
        {
            idsString += QString::number(id) + ",";
        }
        idsString.chop(1);
        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":ids", idsString);
        if (!query.exec())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        while (query.next())
        {
            QHash<QString, QVariant> columnWithValue;
            for (int i = 0; i < columns.count(); i++)
            {
                columnWithValue.insert(columns.at(i), query.value(i));
            }
            listOfColumnsWithValues.append(columnWithValue);
        }
    }

    for (const auto &valuesHash : listOfColumnsWithValues)
    {
        Result<T> entity = TableTools<T>::mapToEntity(valuesHash);
        if (entity.hasError())
        {
            return Result<QList<T>>(entity.error());
        }
        entities.append(entity.value());
    }

    return Result<QList<T>>(entities);
}

//--------------------------------------------

template <class T> Result<QList<T>> DatabaseTableGroup<T>::getAll()
{

    const QString &entityName = m_tableName;
    const QStringList &columns = m_propertyColumns;
    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> listOfColumnsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT " + fields + " FROM " + entityName;
        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (!query.exec())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        while (query.next())
        {
            QHash<QString, QVariant> columnWithValue;
            for (int i = 0; i < columns.count(); i++)
            {
                columnWithValue.insert(columns.at(i), query.value(i));
            }
            listOfColumnsWithValues.append(columnWithValue);
        }
    }

    for (const auto &valuesHash : listOfColumnsWithValues)
    {
        Result<T> entity = TableTools<T>::mapToEntity(valuesHash);
        if (entity.hasError())
        {
            return Result<QList<T>>(entity.error());
        }
        entities.append(entity.value());
    }

    return Result<QList<T>>(entities);
}

//--------------------------------------------

template <class T> QString DatabaseTableGroup<T>::generateFilterQueryString(const QHash<QString, QVariant> &filters)
{
    QStringList filterConditions;
    for (auto it = filters.constBegin(); it != filters.constEnd(); ++it)
    {
        filterConditions.append(QString("%1 = :%1").arg(Tools::fromPascalToSnakeCase(it.key())));
    }
    return filterConditions.join(" AND ");
}

//--------------------------------------------

template <class T> Result<QList<T>> DatabaseTableGroup<T>::getAll(const QHash<QString, QVariant> &filters)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;

    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> fieldsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT " + fields + " FROM " + entityName;
        QString filterStr = generateFilterQueryString(filters);

        if (!filterStr.isEmpty())
        {
            queryStr += " WHERE " + filterStr;
        }

        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        for (auto it = filters.constBegin(); it != filters.constEnd(); ++it)
        {
            query.bindValue(":" + Tools::fromPascalToSnakeCase(it.key()), it.value());
        }

        if (!query.exec())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        while (query.next())
        {
            QHash<QString, QVariant> fieldWithValue;
            for (int i = 0; i < properties.count(); i++)
            {
                fieldWithValue.insert(properties.at(i), query.value(i));
            }
            fieldsWithValues.append(fieldWithValue);
        }
    }

    for (const auto &fieldWithValue : fieldsWithValues)
    {
        Result<T> entity = TableTools<T>::mapToEntity(fieldWithValue);
        if (entity.hasError())
        {
            return Result<QList<T>>(entity.error());
        }
        entities.append(entity.value());
    }

    return Result<QList<T>>(entities);
}

//--------------------------------------------

template <class T> Result<int> DatabaseTableGroup<T>::remove(int id)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    // Generate the SQL DELETE statement
    QString queryStr = "DELETE FROM " + entityName + " WHERE id = :id";

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<int>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id", id);

        // Execute the DELETE statement with the entity ID
        if (!query.exec())
        {
            return Result<int>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        // Return an appropriate Result object based on the query execution result
        if (query.numRowsAffected() == 1)
        {
            return Result<int>(id);
        }
        else
        {
            return Result<int>(Error(Q_FUNC_INFO, Error::Critical, "sql_delete_failed",
                                     "Failed to delete row from database", QString::number(id)));
        }
    }
    return Result<int>(Error(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable", ""));
}

//--------------------------------------------

template <class T> Result<QList<int>> DatabaseTableGroup<T>::remove(QList<int> ids)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    // Generate the SQL DELETE statement
    QString queryStr = "DELETE FROM " + entityName + " WHERE id IN (:ids)";

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QList<int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        QString idsString;
        for (int id : ids)
        {
            idsString += QString::number(id) + ",";
            idsString.chop(1);
        }
        query.bindValue(":id", idsString);

        // Execute the DELETE statement with the entity ID
        if (!query.exec())
        {
            return Result<QList<int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        // Return an appropriate Result object based on the query execution result
        if (query.numRowsAffected() == ids.count())
        {
            return Result<QList<int>>(ids);
        }
        else
        {
            return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_delete_failed",
                                            "Failed to delete row from database", QString::number(ids.count())));
        }
    }
}

//--------------------------------------------

template <class T> Result<QList<int>> DatabaseTableGroup<T>::changeActiveStatus(QList<int> ids, bool active)
{

    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    // Generate the SQL UPDATE statement
    QString queryStr = "UPDATE " + entityName + " SET active = :active WHERE id IN (:ids)";

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QList<int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        QString idsString;
        for (int id : ids)
        {
            idsString += QString::number(id) + ",";
            idsString.chop(1);
        }
        query.bindValue(":ids", idsString);
        query.bindValue(":active", active);

        // Execute the UPDATE statement with the entity ID
        if (!query.exec())
        {
            return Result<QList<int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        // Return an appropriate Result object based on the query execution result
        if (query.numRowsAffected() == ids.count())
        {
            return Result<QList<int>>(ids);
        }
        else
        {
            return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_update_failed",
                                            "Failed to update row in database", QString::number(ids.count())));
        }
    }
}

//--------------------------------------------

template <class T> Result<T> DatabaseTableGroup<T>::add(T &&entity)
{

    const QString &entityTableName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;
    const QStringList &columnsWithoutForeignKeys = m_propertyColumnsWithoutForeignKeys;
    QSqlDatabase database = m_databaseContext->getConnection();
    QHash<QString, QVariant> columnNameWithValue;

    for (const QString &column : columnsWithoutForeignKeys)
    {
        int propertyIndex = T::staticMetaObject.indexOfProperty(Tools::fromSnakeCaseToCamelCase(column).toLatin1());
        QVariant value = T::staticMetaObject.property(propertyIndex).readOnGadget(&entity);
        columnNameWithValue.insert(column, value);
    }

    QString fields;
    QString placeholders;
    for (const QString &column : columnsWithoutForeignKeys)
    {
        if (entity.id() == 0 && column == "id")
        {
            continue;
        }
        fields += column + ",";
        placeholders += ":" + column + ",";
    }
    fields.chop(1);
    placeholders.chop(1);

    QString queryStrMain = "INSERT INTO " + entityTableName + " (" + fields + ") VALUES (" + placeholders + ")";

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStrMain))
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        for (const QString &column : columnsWithoutForeignKeys)
        {
            QVariant value = columnNameWithValue.value(column);
            query.bindValue(":" + column, value);
        }

        if (!query.exec())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        if (query.numRowsAffected() == 1)
        {
            int newOrderingId = query.lastInsertId().toInt();
            entity.setId(newOrderingId);
        }
        else
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_insert_failed", "Failed to insert row into database"));
        }
    }

    // manage foreign properties:
    // this->manageAfterEntityAddition(entity);

    return Result<T>(std::forward<T>(entity));
}

//--------------------------------------------

template <class T> Result<T> DatabaseTableGroup<T>::update(T &&entity)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumns;
    const QStringList &columnsWithoutForeignKeys = m_propertyColumnsWithoutForeignKeys;
    QSqlDatabase database = m_databaseContext->getConnection();
    QHash<QString, QVariant> fieldWithValue;

    for (const QString &column : columnsWithoutForeignKeys)
    {
        int propertyIndex = T::staticMetaObject.indexOfProperty(Tools::fromSnakeCaseToCamelCase(column).toLatin1());
        QVariant value = T::staticMetaObject.property(propertyIndex).readOnGadget(&entity);
        fieldWithValue.insert(column, value);
    }

    QString fields;
    for (const QString &column : columnsWithoutForeignKeys)
    {
        fields += column + " = :" + column + ",";
    }
    fields.chop(1);

    QString queryStrMain = "UPDATE " + entityName + " SET " + fields + " WHERE id = :id";

    int id = entity.id();

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStrMain))
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        for (const QString &property : columnsWithoutForeignKeys)
        {
            QVariant value = fieldWithValue.value(Tools::fromPascalToSnakeCase(property));
            query.bindValue(":" + property, value);
        }

        if (!query.exec())
        {
            return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        if (query.numRowsAffected() != 1)
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_update_failed", "Failed to update row in database"));
        }
    }

    // manage foreign properties:
    // this->manageAfterEntityUpdate(entity);

    return Result<T>(std::forward<T>(entity));
}

//--------------------------------------------

template <class T> Result<bool> DatabaseTableGroup<T>::exists(const QUuid &uuid)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    {

        QSqlQuery query(database);
        QString queryStr = "SELECT COUNT(*) FROM " + entityName + " WHERE uuid = :uuid";
        if (!query.prepare(queryStr))
        {
            return Result<bool>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":uuid", uuid.toString());
        if (!query.exec())
        {
            return Result<bool>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        if (query.next())
        {
            return Result<bool>(query.value(0).toBool());
        }
        else
        {
            return Result<bool>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with uuid " + uuid.toString()));
        }
    }
    return Result<bool>(Error(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable", ""));
}

//--------------------------------------------

template <class T> Result<bool> DatabaseTableGroup<T>::exists(int id)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    {

        QSqlQuery query(database);
        QString queryStr = "SELECT COUNT(*) FROM " + entityName + " WHERE id = :id";
        if (!query.prepare(queryStr))
        {
            return Result<bool>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id", id);
        if (!query.exec())
        {
            return Result<bool>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        if (query.next())
        {
            return Result<bool>(query.value(0).toBool());
        }
        else
        {
            return Result<bool>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with id " + QString::number(id)));
        }
    }
    return Result<bool>(Error(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable", ""));
}

//--------------------------------------------

template <class T> Result<void> DatabaseTableGroup<T>::clear()
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = this->databaseContext()->getConnection();
    QSqlQuery query(database);
    QString queryStrMain = "DELETE FROM " + entityName;

    if (!query.prepare(queryStrMain))
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
    }
    if (!query.exec())
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_clear_failed", "Failed to clear the main table"));
    }

    return Result<void>();
}

//--------------------------------------------

template <class T> Result<SaveData> DatabaseTableGroup<T>::save(const QList<int> &idList)
{
    QMap<QString, QList<QVariantHash>> resultMap;

    QSqlDatabase database = m_databaseContext->getConnection();
    const QString &entityName = m_tableName;
    const QStringList &columns = m_propertyColumns;

    QStringList tableTypes = {"entity"};

    for (const QString &tableType : tableTypes)
    {
        QString tableName = entityName;

        QString queryStr;

        if (idList.isEmpty())
        {
            // Save the whole table
            queryStr = "SELECT * FROM " + tableName;
        }
        else
        {
            // Save the specified list of rows
            QString idPlaceholders;
            for (int i = 0; i < idList.count(); ++i)
            {
                idPlaceholders += ":id" + QString::number(i) + ",";
            }
            idPlaceholders.chop(1);
            queryStr = "SELECT * FROM " + tableName + " WHERE id IN (" + idPlaceholders + ")";
        }

        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QMap<QString, QList<QVariantHash>>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (!idList.isEmpty())
        {
            for (int i = 0; i < idList.count(); ++i)
            {
                query.bindValue(":id" + QString::number(i), idList[i]);
            }
        }

        if (query.exec())
        {
            QList<QVariantHash> resultSet;
            while (query.next())
            {
                QVariantHash row;
                for (const QString &column : columns)
                {
                    row[column] = query.value(column);
                }
                resultSet.append(row);
            }
            resultMap.insert(tableType, resultSet);
        }
        else
        {
            // Handle query error
            return Result<QMap<QString, QList<QVariantHash>>>(
                Error(Q_FUNC_INFO, Error::Critical, "database_table_save_error", query.lastError().text(), queryStr));
        }
    }

    // save foreign entities

    //    auto foreignSaveResult = ForeignEntity<T>::save(idList);

    //    if (foreignSaveResult.hasError())
    //    {
    //        return Result<QMap<QString, QList<QVariantHash>>>(foreignSaveResult.error());
    //    }
    //    resultMap.insert(foreignSaveResult.value());

    return Result<QMap<QString, QList<QVariantHash>>>(resultMap);
}

//--------------------------------------------
template <class T> Result<void> DatabaseTableGroup<T>::restore(const SaveData &saveData)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    const QString &tableName = m_tableName;
    const QStringList &columns = m_propertyColumns;

    for (const QString &tableType : saveData.keys())
    {

        const QList<QVariantHash> &rows = saveData.value(tableType);

        for (const QVariantHash &row : rows)
        {
            // Check if the row exists in the table
            QSqlQuery checkQuery(database);
            QString checkQueryStr = "SELECT COUNT(*) FROM " + tableName + " WHERE id = :id";

            if (!checkQuery.prepare(checkQueryStr))
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text(), checkQueryStr));
            }
            checkQuery.bindValue(":id", row.value("id"));
            if (!checkQuery.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text(), checkQueryStr));
            }
            if (!checkQuery.next())
            {
                return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text()));
            }

            int rowCount = checkQuery.value(0).toInt();

            if (rowCount == 1)
            {
                // Update the existing row
                QString updateStr = "UPDATE " + tableName + " SET ";
                for (const QString &column : columns)
                {
                    if (column != "id")
                    {
                        updateStr += column + " = :" + column + ",";
                    }
                }
                updateStr.chop(1);
                updateStr += " WHERE id = :id";

                QSqlQuery updateQuery(database);
                updateQuery.prepare(updateStr);
                for (const QString &column : columns)
                {
                    updateQuery.bindValue(":" + column, row.value(column));
                }

                if (!updateQuery.exec())
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text()));
                }
            }
            else
            {
                // Insert the missing row
                QString insertStr = "INSERT INTO " + tableName + " (";
                QString placeholders;
                for (const QString &column : columns)
                {
                    insertStr += column + ",";
                    placeholders += ":" + column + ",";
                }
                insertStr.chop(1);
                placeholders.chop(1);
                insertStr += ") VALUES (" + placeholders + ")";

                QSqlQuery insertQuery(database);
                if (!insertQuery.prepare(insertStr))
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertStr));
                }
                for (const QString &column : columns)
                {
                    insertQuery.bindValue(":" + column, row.value(column));
                }

                if (!insertQuery.exec())
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text()));
                }
            }
        }
    }

    // restore foreign entities

    //    auto foreignRestoreResult = ForeignEntity<T>::restore(saveData);

    //    if (foreignRestoreResult.hasError())
    //    {
    //        return Result<void>(foreignRestoreResult.error());
    //    }

    return Result<void>();
}

//--------------------------------------------

template <class T> Result<void> DatabaseTableGroup<T>::beginTransaction()
{

    QSqlDatabase database = m_databaseContext->getConnection();

    bool result = database.transaction();
    if (!result)
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
    }
    return Result<void>();
}
//--------------------------------------------

template <class T> Result<void> DatabaseTableGroup<T>::commit()
{

    QSqlDatabase database = m_databaseContext->getConnection();

    bool result = database.commit();
    if (!result)
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
    }
    return Result<void>();
}
//--------------------------------------------

template <class T> Result<void> DatabaseTableGroup<T>::rollback()
{
    QSqlDatabase database = m_databaseContext->getConnection();

    bool result = database.rollback();
    if (!result)
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
    }
    return Result<void>();
}
//--------------------------------------------

template <class T> QSharedPointer<InterfaceDatabaseContext> DatabaseTableGroup<T>::databaseContext() const
{
    return m_databaseContext;
}

template <class T> QString DatabaseTableGroup<T>::tableName() const
{
    return m_tableName;
}

template <class T> QStringList DatabaseTableGroup<T>::properties() const
{
    return m_properties;
}

template <class T> QStringList DatabaseTableGroup<T>::propertyColumns() const
{
    return m_propertyColumns;
}

//--------------------------------------------

// list properties

template <class T>
QString DatabaseTableGroup<T>::getListTableName(const QString &listPropertyName, PropertyWithList::ListType type)
{
    QString tableName = TableTools<T>::getEntityClassName();
    QString upperCaseListPropertyName = listPropertyName;
    upperCaseListPropertyName[0] = upperCaseListPropertyName[0].toUpper();

    switch (type)
    {
    case PropertyWithList::ListType::List:
        tableName += upperCaseListPropertyName + "List";
        break;
    case PropertyWithList::ListType::Set:
        tableName += upperCaseListPropertyName + "Set";
        break;
    }

    return tableName;
}

//--------------------------------------------

template <class T> bool DatabaseTableGroup<T>::isCommonType(int typeId)
{
    static const QSet<int> commonTypes{
        qMetaTypeId<int>(),         qMetaTypeId<uint>(),         qMetaTypeId<long>(),         qMetaTypeId<ulong>(),
        qMetaTypeId<long long>(),   qMetaTypeId<double>(),       qMetaTypeId<float>(),        qMetaTypeId<bool>(),
        qMetaTypeId<QChar>(),       qMetaTypeId<QString>(),      qMetaTypeId<QByteArray>(),   qMetaTypeId<QDate>(),
        qMetaTypeId<QTime>(),       qMetaTypeId<QDateTime>(),    qMetaTypeId<QUrl>(),         qMetaTypeId<QUuid>(),
        qMetaTypeId<QVariantMap>(), qMetaTypeId<QVariantList>(), qMetaTypeId<QVariantHash>(),
    };
    return commonTypes.contains(typeId);
}

//--------------------------------------------

template <class T> QHash<QString, PropertyWithList> DatabaseTableGroup<T>::getEntityPropertiesWithList()
{
    QHash<QString, PropertyWithList> propertiesWithList;
    const QMetaObject &metaObject = T::staticMetaObject;

    QRegularExpression listRegex(R"(QList<(.+)>)");
    QRegularExpression setRegex(R"(QSet<(.+)>)");

    for (int i = 0; i < metaObject.propertyCount(); ++i)
    {
        QMetaProperty property = metaObject.property(i);
        QString propertyTypeName = property.typeName();

        QRegularExpressionMatch listMatch = listRegex.match(propertyTypeName);
        QRegularExpressionMatch setMatch = setRegex.match(propertyTypeName);

        if (listMatch.hasMatch() || setMatch.hasMatch())
        {
            QString innerTypeName = listMatch.hasMatch() ? listMatch.captured(1) : setMatch.captured(1);
            QMetaType innerMetaType = QMetaType::fromName(innerTypeName.toLatin1().constData());

            if (isCommonType(innerMetaType.id()))
            {
                PropertyWithList::ListType listType =
                    listMatch.hasMatch() ? PropertyWithList::ListType::List : PropertyWithList::ListType::Set;
                QString listTableName = getListTableName(property.name(), listType);
                PropertyWithList propertyWithList{innerTypeName, listTableName, listType};
                propertiesWithList.insert(property.name(), propertyWithList);
            }
        }
    }

    return propertiesWithList;
}

//--------------------------------------------

template <class T>
Result<QList<T>> DatabaseTableGroup<T>::getEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                                int leftEntityId, const QString &field)
{
    Result<QList<T>> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == Qleany::Domain::RelationshipDirection::Forward && relationship.fieldName == field)
        {
            // One to Many Unordered:
            if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.getRightEntities(leftEntityId);
            }
            // One to Many Ordered:
            else if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                     relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.getRightEntities(leftEntityId);
            }
            // Many to Many Unordered:
            else if (relationship.type == Qleany::Domain::RelationshipType::ManyToMany)
            {
                //                ManyToManyAssociator<T, OtherEntity> associator(m_databaseContext, relationship);
                //                result = associator.getRelatedEntities(relationship.field);
            }
            else
            {
                result = Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "not_implemented", "not implemented"));
            }
            break;
        }
    }

    return result;
}

//--------------------------------------------

template <class T>
Result<QList<T>> DatabaseTableGroup<T>::updateEntitiesInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                                   int leftEntityId, const QString &field,
                                                                   const QList<T> &rightEntities)
{
    Result<QList<T>> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == Qleany::Domain::RelationshipDirection::Forward && relationship.fieldName == field)
        {
            // One to Many Unordered:
            if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            // One to Many Ordered:
            else if (relationship.type == Qleany::Domain::RelationshipType::OneToMany &&
                     relationship.cardinality == Qleany::Domain::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            // Many to Many Unordered:
            else if (relationship.type == Qleany::Domain::RelationshipType::ManyToMany)
            {
                //                ManyToManyAssociator<T, OtherEntity> associator(m_databaseContext, relationship);
                //                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            else
            {
                result = Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "not_implemented", "not implemented"));
            }
            break;
        }
    }

    return result;
}

//--------------------------------------------

template <class T>
Result<T> DatabaseTableGroup<T>::getEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                       int leftEntityId, const QString &field)
{

    Result<T> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == Qleany::Domain::RelationshipDirection::Forward &&
            relationship.fieldName == field && relationship.type == Qleany::Domain::RelationshipType::OneToOne &&
            relationship.cardinality == Qleany::Domain::RelationshipCardinality::One)
        {

            OneToOneAssociator<T> associator(m_databaseContext, relationship);
            result = associator.getRightEntity(leftEntityId);
        }
        break;
    }

    return result;
}

//--------------------------------------------

template <class T>
Result<T> DatabaseTableGroup<T>::updateEntityInRelationOf(const Qleany::Domain::EntitySchema &leftEntitySchema,
                                                          int leftEntityId, const QString &field, const T &rightEntity)
{
    Result<T> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == Qleany::Domain::RelationshipDirection::Forward &&
            relationship.fieldName == field && relationship.type == Qleany::Domain::RelationshipType::OneToOne &&
            relationship.cardinality == Qleany::Domain::RelationshipCardinality::One)
        {

            OneToOneAssociator<T> associator(m_databaseContext, relationship);
            result = associator.updateRightEntity(leftEntityId, rightEntity);
        }
        break;
    }
    return result;
}
} // namespace Qleany::Database

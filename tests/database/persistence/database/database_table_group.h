// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "QtCore/qdebug.h"
#include "QtCore/qmetaobject.h"
#include "one_to_many_ordered_associator.h"
#include "one_to_many_unordered_associator.h"
#include "many_to_many_unordered_associator.h"
#include "one_to_one_associator.h"
#include "result.h"
#include "database/interface_database_context.h"
#include "database/interface_database_table_group.h"
#include "entity_base.h"
#include "entity_schema.h"
#include "database_test_persistence_export.h"
#include "tools.h"
#include <QDateTime>
#include <QMetaObject>
#include <QReadWriteLock>
#include <QRegularExpression>
#include <QSharedPointer>
#include <QSqlError>
#include <QSqlQuery>
#include <QTime>

using namespace DatabaseTest::Contracts::Database;

struct DATABASE_TEST_PERSISTENCE_EXPORT PropertyWithList
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

namespace DatabaseTest::Persistence::Database
{

template <class T> class DatabaseTableGroup : public virtual InterfaceDatabaseTableGroup<T>
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
    Result<QList<T>> getEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int leftEntityId,
                                             const QString &field) override;
    Result<T> getEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int leftEntityId,
                                    const QString &field) override;
    Result<QList<T>> updateEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                int leftEntityId, const QString &field,
                                                const QList<T> &rightEntities) override;
    Result<T> updateEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int leftEntityId,
                                       const QString &field, const T &rightEntity) override;
    Result<void> removeAssociationsWith(QList<int> rightEntityIds) override;

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
    static_assert(std::is_base_of<Entities::EntityBase, T>::value, "T must inherit from Entities::Entity");

    // entity table creation
    m_databaseContext->appendCreationSql("entity_table", getTableCreationSql());

    for (const auto &relationship : T::schema.relationships)
    {
        if (relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Backward)
        {
            QString junctionCreationSql;

            if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToOne)
            {
                OneToOneAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                     relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                     relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                m_databaseContext->appendCreationSql("junction_table", associator.getTableCreationSql());
            }
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::ManyToMany)
            {
            }
        }
    }
}

//--------------------------------------------

template <class T>
DatabaseTableGroup<T>::DatabaseTableGroup(const DatabaseTableGroup &other) : m_databaseContext(other.databaseContext())
{
    static_assert(std::is_base_of<Entities::EntityBase, T>::value, "T must inherit from Entities::Entity");
}

template <class T> QString DatabaseTableGroup<T>::getTableCreationSql() const
{

    QString tableName = TableTools<T>::getEntityTableName();

    QString createTableSql = "CREATE TABLE %1 ("_L1.arg(tableName);

    QStringList relationshipPropertyNameListToIgnore;

    int propertyCount = T::staticMetaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {

        QMetaProperty property = T::staticMetaObject.property(i);
        const QString &propertyName = QString::fromLatin1(property.name());

        // ignore QList and QSet properties

        if (property.isReadable())
        {
            if (TableTools<T>::isForeign(propertyName))
            {

                relationshipPropertyNameListToIgnore.append(propertyName);
                relationshipPropertyNameListToIgnore.append(propertyName + "Loaded"_L1);
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

        if (relationshipPropertyNameListToIgnore.contains(QString::fromLatin1(property.name())))
        {
            continue;
        }

        int propertyMetaType = property.userType();
        const char *propertySqlType = Tools::qtMetaTypeToSqlType(propertyMetaType);

        if (propertySqlType)
        {
            createTableSql.append("%1 %2"_L1.arg(Tools::fromPascalToSnakeCase(QString::fromLatin1(propertyName)),
                                                 QString::fromLatin1(propertySqlType)));

            // Set uuid property as primary key, not null, and unique
            if (strcmp(propertyName, "id") == 0)
            {
                createTableSql.append(" PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT"
                                      " UNIQUE ON CONFLICT ROLLBACK"
                                      " NOT NULL ON CONFLICT ROLLBACK"_L1);
            }

            createTableSql.append(", "_L1);
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

    createTableSql.append(");"_L1);

    return createTableSql;
}

template <class T> Result<T> DatabaseTableGroup<T>::get(int id)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumnsWithoutForeignKeys;

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
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + entityName + " WHERE "_L1 + "id = :id"_L1;
        if (!query.prepare(queryStr))
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id"_L1, QVariant(id));
        if (!query.exec())
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<T>(
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
            return Result<T>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_row_missing",
                                         "No row with id "_L1 + QString::number(id)));
        }
    }

    return TableTools<T>::mapToEntity(columnWithValues);
}

//--------------------------------------------

template <class T> Result<T> DatabaseTableGroup<T>::get(const QUuid &uuid)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_properties;
    const QStringList &columns = m_propertyColumnsWithoutForeignKeys;

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
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + entityName + " WHERE "_L1 + "uuid = :uuid"_L1;
        if (!query.prepare(queryStr))
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":uuid"_L1, QVariant(uuid));
        if (!query.exec())
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<T>(
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
            return Result<T>(
                QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with uuid "_L1 + uuid.toString()));
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
    const QStringList &columns = m_propertyColumnsWithoutForeignKeys;

    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> listOfColumnsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ","_L1;
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + entityName + " WHERE id IN (:ids)"_L1;
        QString idsString;
        for (int id : ids)
        {
            idsString += QString::number(id) + ","_L1;
        }
        idsString.chop(1);
        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":ids"_L1, idsString);
        if (!query.exec())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
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
    const QStringList &columns = m_propertyColumnsWithoutForeignKeys;
    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> listOfColumnsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ","_L1;
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + entityName;
        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (!query.exec())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
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
        filterConditions.append("%1 = :%1"_L1.arg(Tools::fromPascalToSnakeCase(it.key())));
    }
    return filterConditions.join(" AND "_L1);
}

//--------------------------------------------

template <class T> Result<QList<T>> DatabaseTableGroup<T>::getAll(const QHash<QString, QVariant> &filters)
{
    const QString &entityName = m_tableName;
    const QStringList &properties = m_propertyColumnsWithoutForeignKeys;
    const QStringList &columns = m_propertyColumns;

    QSqlDatabase database = m_databaseContext->getConnection();
    QList<QHash<QString, QVariant>> fieldsWithValues;
    QList<T> entities;

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ","_L1;
    }
    fields.chop(1);

    {
        QSqlQuery query(database);
        QString queryStr = "SELECT "_L1 + fields + " FROM "_L1 + entityName;
        QString filterStr = generateFilterQueryString(filters);

        if (!filterStr.isEmpty())
        {
            queryStr += " WHERE "_L1 + filterStr;
        }

        if (!query.prepare(queryStr))
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        for (auto it = filters.constBegin(); it != filters.constEnd(); ++it)
        {
            query.bindValue(":"_L1 + Tools::fromPascalToSnakeCase(it.key()), it.value());
        }

        if (!query.exec())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (query.lastError().isValid())
        {
            return Result<QList<T>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
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
    QString queryStr = "DELETE FROM %1 WHERE id = :id"_L1.arg(entityName);

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<int>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id"_L1, id);

        // Execute the DELETE statement with the entity ID
        if (!query.exec())
        {
            return Result<int>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        // Return an appropriate Result object based on the query execution result
        if (query.numRowsAffected() == 1)
        {
            return Result<int>(id);
        }
        else
        {
            return Result<int>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_delete_failed",
                                           "Failed to delete row from database", QString::number(id)));
        }
    }
    return Result<int>(QLN_ERROR_1(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable"));
}

//--------------------------------------------

template <class T> Result<QList<int>> DatabaseTableGroup<T>::remove(QList<int> ids)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    // Generate the SQL DELETE statement
    QString queryStr = "DELETE FROM %1 WHERE id IN ("_L1;

    for (int id : ids)
    {
        queryStr += QString::number(id) + ","_L1;
    }
    queryStr.chop(1);
    queryStr += ")"_L1;
    queryStr = queryStr.arg(entityName);

    QSqlQuery query(database);
    if (!query.prepare(queryStr))
    {
        return Result<QList<int>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    // Execute the DELETE statement with the entity ID
    if (!query.exec())
    {
        return Result<QList<int>>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    // Return an appropriate Result object based on the query execution result
    if (query.numRowsAffected() == ids.count())
    {
        return Result<QList<int>>(ids);
    }
    else
    {
        return Result<QList<int>>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_delete_failed",
                                              "Failed to delete row from database", QString::number(ids.count())));
    }
}

//--------------------------------------------

template <class T> Result<QList<int>> DatabaseTableGroup<T>::changeActiveStatus(QList<int> ids, bool active)
{

    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    // Generate the SQL UPDATE statement
    QString queryStr = "UPDATE "_L1 + entityName + " SET active = :active WHERE id IN (:ids)"_L1;

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QList<int>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        QString idsString;
        for (int id : ids)
        {
            idsString += QString::number(id) + ","_L1;
            idsString.chop(1);
        }
        query.bindValue(":ids"_L1, idsString);
        query.bindValue(":active"_L1, active);

        // Execute the UPDATE statement with the entity ID
        if (!query.exec())
        {
            return Result<QList<int>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        // Return an appropriate Result object based on the query execution result
        if (query.numRowsAffected() == ids.count())
        {
            return Result<QList<int>>(ids);
        }
        else
        {
            return Result<QList<int>>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_update_failed",
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
        int propertyIndex =
            T::staticMetaObject.indexOfProperty(Tools::fromSnakeCaseToCamelCase(column).toLatin1().constData());
        QVariant value = T::staticMetaObject.property(propertyIndex).readOnGadget(&entity);
        columnNameWithValue.insert(column, value);
    }

    QString fields;
    QString placeholders;
    for (const QString &column : columnsWithoutForeignKeys)
    {
        if (entity.id() == 0 && column == "id"_L1)
        {
            continue;
        }
        fields += column + ","_L1;
        placeholders += ":"_L1 + column + ","_L1;
    }
    fields.chop(1);
    placeholders.chop(1);

    QString queryStrMain =
        "INSERT INTO "_L1 + entityTableName + " ("_L1 + fields + ") VALUES ("_L1 + placeholders + ")"_L1;

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStrMain))
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        for (const QString &column : columnsWithoutForeignKeys)
        {
            QVariant value = columnNameWithValue.value(column);
            query.bindValue(":"_L1 + column, value);
        }

        if (!query.exec())
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        if (query.numRowsAffected() == 1)
        {
            int newOrderingId = query.lastInsertId().toInt();
            entity.setId(newOrderingId);
        }
        else
        {
            return Result<T>(
                QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_insert_failed", "Failed to insert row into database"));
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
        int propertyIndex =
            T::staticMetaObject.indexOfProperty(Tools::fromSnakeCaseToCamelCase(column).toLatin1().constData());
        QVariant value = T::staticMetaObject.property(propertyIndex).readOnGadget(&entity);
        fieldWithValue.insert(column, value);
    }

    QString fields;
    for (const QString &column : columnsWithoutForeignKeys)
    {
        fields += column + " = :"_L1 + column + ","_L1;
    }
    fields.chop(1);

    QString queryStrMain = "UPDATE "_L1 + entityName + " SET "_L1 + fields + " WHERE id = :id"_L1;

    int id = entity.id();

    {
        QSqlQuery query(database);
        if (!query.prepare(queryStrMain))
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        for (const QString &property : columnsWithoutForeignKeys)
        {
            QVariant value = fieldWithValue.value(Tools::fromPascalToSnakeCase(property));
            query.bindValue(":"_L1 + property, value);
        }

        if (!query.exec())
        {
            return Result<T>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
        }

        if (query.numRowsAffected() != 1)
        {
            return Result<T>(
                QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_update_failed", "Failed to update row in database"));
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
        QString queryStr = "SELECT COUNT(*) FROM "_L1 + entityName + " WHERE uuid = :uuid"_L1;
        if (!query.prepare(queryStr))
        {
            return Result<bool>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":uuid"_L1, uuid.toString());
        if (!query.exec())
        {
            return Result<bool>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        if (query.next())
        {
            return Result<bool>(query.value(0).toBool());
        }
        else
        {
            return Result<bool>(
                QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_row_missing", "No row with uuid "_L1 + uuid.toString()));
        }
    }
    return Result<bool>(QLN_ERROR_1(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable"));
}

//--------------------------------------------

template <class T> Result<bool> DatabaseTableGroup<T>::exists(int id)
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = m_databaseContext->getConnection();

    {

        QSqlQuery query(database);
        QString queryStr = "SELECT COUNT(*) FROM "_L1 + entityName + " WHERE id = :id"_L1;
        if (!query.prepare(queryStr))
        {
            return Result<bool>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        query.bindValue(":id"_L1, id);
        if (!query.exec())
        {
            return Result<bool>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }

        if (query.next())
        {
            return Result<bool>(query.value(0).toBool());
        }
        else
        {
            return Result<bool>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_row_missing",
                                            "No row with id "_L1 + QString::number(id)));
        }
    }
    return Result<bool>(QLN_ERROR_1(Q_FUNC_INFO, Error::Fatal, "normaly_unreacheable"));
}

//--------------------------------------------

template <class T> Result<void> DatabaseTableGroup<T>::clear()
{
    const QString &entityName = m_tableName;
    QSqlDatabase database = this->databaseContext()->getConnection();
    QSqlQuery query(database);
    QString queryStrMain = "DELETE FROM "_L1 + entityName;

    if (!query.prepare(queryStrMain))
    {
        return Result<void>(
            QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
    }
    if (!query.exec())
    {
        return Result<void>(
            QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_clear_failed", "Failed to clear the main table"));
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

    QStringList tableTypes = {"entity"_L1};

    for (const QString &tableType : tableTypes)
    {
        QString tableName = entityName;

        QString queryStr;

        if (idList.isEmpty())
        {
            // Save the whole table
            queryStr = "SELECT * FROM "_L1 + tableName;
        }
        else
        {
            // Save the specified list of rows
            QString idPlaceholders;
            for (int i = 0; i < idList.count(); ++i)
            {
                idPlaceholders += ":id"_L1 + QString::number(i) + ","_L1;
            }
            idPlaceholders.chop(1);
            queryStr = "SELECT * FROM "_L1 + tableName + " WHERE id IN ("_L1 + idPlaceholders + ")"_L1;
        }

        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QMap<QString, QList<QVariantHash>>>(
                QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (!idList.isEmpty())
        {
            for (int i = 0; i < idList.count(); ++i)
            {
                query.bindValue(":id"_L1 + QString::number(i), idList[i]);
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
            return Result<QMap<QString, QList<QVariantHash>>>(QLN_ERROR_3(
                Q_FUNC_INFO, Error::Critical, "database_table_save_error", query.lastError().text(), queryStr));
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
            QString checkQueryStr = "SELECT COUNT(*) FROM "_L1 + tableName + " WHERE id = :id"_L1;

            if (!checkQuery.prepare(checkQueryStr))
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                checkQuery.lastError().text(), checkQueryStr));
            }
            checkQuery.bindValue(":id"_L1, row.value("id"_L1));
            if (!checkQuery.exec())
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                checkQuery.lastError().text(), checkQueryStr));
            }
            if (!checkQuery.next())
            {
                return Result<void>(
                    QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text()));
            }

            int rowCount = checkQuery.value(0).toInt();

            if (rowCount == 1)
            {
                // Update the existing row
                QString updateStr = "UPDATE "_L1 + tableName + " SET "_L1;
                for (const QString &column : columns)
                {
                    if (column != "id"_L1)
                    {
                        updateStr += column + " = :"_L1 + column + ","_L1;
                    }
                }
                updateStr.chop(1);
                updateStr += " WHERE id = :id"_L1;

                QSqlQuery updateQuery(database);
                updateQuery.prepare(updateStr);
                for (const QString &column : columns)
                {
                    updateQuery.bindValue(":"_L1 + column, row.value(column));
                }

                if (!updateQuery.exec())
                {
                    return Result<void>(
                        QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text()));
                }
            }
            else
            {
                // Insert the missing row
                QString insertStr = "INSERT INTO "_L1 + tableName + " ("_L1;
                QString placeholders;
                for (const QString &column : columns)
                {
                    insertStr += column + ","_L1;
                    placeholders += ":"_L1 + column + ","_L1;
                }
                insertStr.chop(1);
                placeholders.chop(1);
                insertStr += ") VALUES ("_L1 + placeholders + ")"_L1;

                QSqlQuery insertQuery(database);
                if (!insertQuery.prepare(insertStr))
                {
                    return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                    insertQuery.lastError().text(), insertStr));
                }
                for (const QString &column : columns)
                {
                    insertQuery.bindValue(":"_L1 + column, row.value(column));
                }

                if (!insertQuery.exec())
                {
                    return Result<void>(
                        QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text()));
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
        return Result<void>(
            QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
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
        return Result<void>(
            QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
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
        return Result<void>(
            QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "transaction_error", database.lastError().text()));
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
        tableName += upperCaseListPropertyName + "List"_L1;
        break;
    case PropertyWithList::ListType::Set:
        tableName += upperCaseListPropertyName + "Set"_L1;
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

    QRegularExpression listRegex(R"(QList<(.+)>)"_L1);
    QRegularExpression setRegex(R"(QSet<(.+)>)"_L1);

    for (int i = 0; i < metaObject.propertyCount(); ++i)
    {
        QMetaProperty property = metaObject.property(i);
        QString propertyTypeName = QString::fromLatin1(property.typeName());

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
                QString listTableName = getListTableName(QString::fromLatin1(property.name()), listType);
                PropertyWithList propertyWithList{innerTypeName, listTableName, listType};
                propertiesWithList.insert(QString::fromLatin1(property.name()), propertyWithList);
            }
        }
    }

    return propertiesWithList;
}

//--------------------------------------------

template <class T>
Result<QList<T>> DatabaseTableGroup<T>::getEntitiesInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                                int leftEntityId, const QString &field)
{
    Result<QList<T>> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Forward &&
            relationship.fieldName == field)
        {
            // One to Many Unordered:
            if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.getRightEntities(leftEntityId);
            }
            // One to Many Ordered:
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                     relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.getRightEntities(leftEntityId);
            }
            // Many to Many Unordered:
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::ManyToMany)
            {
                ManyToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.getRightEntities(leftEntityId);
            }
            else
            {
                result =
                    Result<QList<T>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "not_implemented", "not implemented"));
            }
            break;
        }
    }

    return result;
}

//--------------------------------------------

template <class T>
Result<QList<T>> DatabaseTableGroup<T>::updateEntitiesInRelationOf(
    const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema, int leftEntityId, const QString &field,
    const QList<T> &rightEntities)
{
    Result<QList<T>> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Forward &&
            relationship.fieldName == field)
        {
            // One to Many Unordered:
            if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyUnordered)
            {
                OneToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            // One to Many Ordered:
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
                     relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyOrdered)
            {
                OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            // Many to Many Unordered:
            else if (relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::ManyToMany)
            {
                ManyToManyUnorderedAssociator<T> associator(m_databaseContext, relationship);
                result = associator.updateRightEntities(leftEntityId, rightEntities);
            }
            else
            {
                result =
                    Result<QList<T>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "not_implemented", "not implemented"));
            }
            break;
        }
    }

    return result;
}

//--------------------------------------------

template <class T>
Result<T> DatabaseTableGroup<T>::getEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                       int leftEntityId, const QString &field)
{

    Result<T> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Forward &&
            relationship.fieldName == field && relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToOne &&
            relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::One)
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
Result<T> DatabaseTableGroup<T>::updateEntityInRelationOf(const DatabaseTest::Entities::EntitySchema::EntitySchema &leftEntitySchema,
                                                          int leftEntityId, const QString &field, const T &rightEntity)
{
    Result<T> result;

    for (const auto &relationship : leftEntitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Forward &&
            relationship.fieldName == field && relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToOne &&
            relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::One)
        {

            OneToOneAssociator<T> associator(m_databaseContext, relationship);
            result = associator.updateRightEntity(leftEntityId, rightEntity);
        }
        break;
    }
    return result;
}

template <class T> Result<void> DatabaseTableGroup<T>::removeAssociationsWith(QList<int> rightEntityIds)
{
    // only reordering OneToManyOrdered relationships on backward relationships, meaning this entity T is the "target"
    // of the relationship. Other associations types are deleted in cascade by the database.
    const DatabaseTest::Entities::EntitySchema::EntitySchema &entitySchema = T::schema;
    for (const auto &relationship : entitySchema.relationships)
    {
        if (relationship.rightEntityId == T::enumValue() &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Backward &&
            relationship.type == DatabaseTest::Entities::EntitySchema::RelationshipType::OneToMany &&
            relationship.cardinality == DatabaseTest::Entities::EntitySchema::RelationshipCardinality::ManyOrdered)
        {
            OneToManyOrderedAssociator<T> associator(m_databaseContext, relationship);
            auto result = associator.removeTheseRightIds(rightEntityIds);
            QLN_RETURN_IF_ERROR(void, result)
        }
    }
    return Result<void>();
}
} // namespace DatabaseTest::Database
// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QMetaProperty>
#include <QSqlQuery>
#include <QStringList>
#include "result.h"
#include "entity_schema.h"

namespace DatabaseTest::Persistence::Database
{

class Tools
{

  public:
    static QString fromPascalToSnakeCase(const QString &string);
    static QString fromSnakeCaseToPascalCase(const QString &string);
    static QString fromSnakeCaseToCamelCase(const QString &string);
    static const char *qtMetaTypeToSqlType(int qtMetaType);
};

template <class T> class TableTools
{
  public:
    /**
     * @brief getEntityClassName returns the name of the class associated with this database.
     * @return The class name as a QString.
     */
    static QString getEntityClassName();
    static QString getEntityTableName();
    static QString getTableNameFromClassName(const QString &className);

    /**
     * @brief getEntityProperties returns the list of properties associated with the class
     * associated with this database.
     * @return A QStringList containing the property names.
     */
    static QStringList getEntityProperties();
    static QVariant getEntityPropertyValue(const T &entity, const QString &propertyName);
    static void setEntityPropertyValue(T &entity, const QString &propertyName, const QVariant &propertyValue);

    /**
     * @brief Maps a hash of field names to their corresponding values to an entity of type T.
     * @param fieldWithValue The hash of field names to their corresponding values to be mapped.
     * @return Result<T> The result of the mapping operation, containing either the mapped entity of type T or an error.
     */
    static Result<T> mapToEntity(const QHash<QString, QVariant> &valuesHash);
    static void readEntityFromQuery(T &entity, const QSqlQuery &query);
    static void bindValueFromEntityToQuery(const T &entity, QSqlQuery &query);
    static bool isForeign(const QString &propertyName);
    static QStringList getColumnNamesWithoutForeignKeys();
    static QStringList getColumnNamesWithForeignKeys();
};

//--------------------------------------------

template <class T> QString TableTools<T>::getEntityClassName()
{

    const QMetaObject &sourceMetaObject = T::staticMetaObject;
    return QString::fromLatin1(sourceMetaObject.className()).split("::"_L1).last();
}

//--------------------------------------------

template <class T> QString TableTools<T>::getEntityTableName()
{
    QString className = TableTools<T>::getEntityClassName();
    return getTableNameFromClassName(className);
}

//--------------------------------------------

template <class T> QString TableTools<T>::getTableNameFromClassName(const QString &className)
{
    return Tools::fromPascalToSnakeCase(className.split("::"_L1).last());
}

//--------------------------------------------

template <class T> QStringList TableTools<T>::getEntityProperties()
{
    QStringList propertyList;

    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            if (QString::fromLatin1(property.name()) == "objectName"_L1)
            {
                continue;
            }
            propertyList.append(QString::fromLatin1(property.name()));
        }
    }

    return propertyList;
}

//--------------------------------------------

template <class T> QVariant TableTools<T>::getEntityPropertyValue(const T &entity, const QString &propertyName)
{
    QVariant propertyValue;

    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            if (QString::fromLatin1(property.name()) == "objectName"_L1)
            {
                continue;
            }
            if (QString::fromLatin1(property.name()) == propertyName)
            {
                propertyValue = property.readOnGadget(&entity);
                break;
            }
        }
    }

    return propertyValue;
}

template <class T>
void TableTools<T>::setEntityPropertyValue(T &entity, const QString &propertyName, const QVariant &propertyValue)
{
    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isWritable())
        {
            if (QString::fromLatin1(property.name()) == "objectName"_L1)
            {
                continue;
            }
            if (QString::fromLatin1(property.name()) == propertyName)
            {
                property.writeOnGadget(&entity, propertyValue);
                break;
            }
        }
    }
}

//--------------------------------------------

template <class T> Result<T> TableTools<T>::mapToEntity(const QHash<QString, QVariant> &valuesHash)
{
    T entity;
    const QMetaObject &metaObject = T::staticMetaObject;

    QHash<QString, QVariant>::const_iterator i = valuesHash.constBegin();
    while (i != valuesHash.constEnd())
    {

        QString columnName = i.key();
        QString propertyName = Tools::fromSnakeCaseToCamelCase(columnName);

        int destinationPropertyIndex = metaObject.indexOfProperty(propertyName.toLatin1().constData());
        if (destinationPropertyIndex >= 0)
        {
            QVariant value = i.value();
            QMetaProperty destinationProperty = metaObject.property(destinationPropertyIndex);

            if (destinationProperty.isWritable() &&
                QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
            {
                bool success = destinationProperty.writeOnGadget(&entity, value);
                if (!success)
                {
                    Result<T>(QLN_ERROR_3(Q_FUNC_INFO, Error::Fatal, "map_write_failed",
                                          "Failed to write value to destination property", propertyName));
                }
            }
        }
        else
        {
            Result<T>(QLN_ERROR_3(Q_FUNC_INFO, Error::Fatal, "map_missing_property",
                                  "Missing property in destination object", propertyName));
        }
        ++i;
    }
    return Result<T>(entity);
}

//--------------------------------------------

template <class T> void TableTools<T>::readEntityFromQuery(T &entity, const QSqlQuery &query)
{
    const QStringList &properties = getEntityProperties();
    for (int i = 0; i < properties.count(); i++)
    {
        const QString &property = properties.at(i);
        QVariant value = query.value(i);
        QByteArray truePropertyName = property.toLatin1();
        if (!entity.setProperty(truePropertyName, value))
        {

            qCritical() << "setting property "_L1 << truePropertyName << "failed on"_L1 << getEntityClassName();
        }
    }
}

template <class T> bool TableTools<T>::isForeign(const QString &propertyName)
{
    bool result = false;

    for (const auto &relationship : T::schema.relationships)
    {
        if (relationship.fieldName == propertyName &&
            relationship.direction == DatabaseTest::Entities::EntitySchema::RelationshipDirection::Forward)
        {
            result = true;
            break;
        }
    }

    return result;
}

//--------------------------------------------

template <class T> QStringList TableTools<T>::getColumnNamesWithoutForeignKeys()
{
    QStringList result;
    for (const auto &field : T::schema.fields)
    {
        if (field.isLinkedToAnotherEntity)
        {
            continue;
        }

        result << Tools::fromPascalToSnakeCase(field.name);
    }

    return result;
}
//--------------------------------------------

template <class T> QStringList TableTools<T>::getColumnNamesWithForeignKeys()
{
    QStringList result;
    for (const auto &field : T::schema.fields)
    {
        result << Tools::fromPascalToSnakeCase(field.name);
    }

    return result;
}

//--------------------------------------------

inline QString Tools::fromPascalToSnakeCase(const QString &string)
{
    QString finalString;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character.isUpper())
        {
            if (i != 0)
            {
                finalString.append("_"_L1);
            }
            finalString.append(character.toLower());
        }
        else
        {
            finalString.append(character);
        }
    }
    return finalString;
}

//--------------------------------------------

inline QString Tools::fromSnakeCaseToPascalCase(const QString &string)
{
    QString finalString;
    bool next_letter_must_be_upper = false;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character == QChar::fromLatin1('_'))
        {
            next_letter_must_be_upper = true;
            continue;
        }
        else if (next_letter_must_be_upper || i == 0)
        {
            finalString.append(character.toUpper());
            next_letter_must_be_upper = false;
        }
        else
        {
            finalString.append(character.toLower());
        }
    }
    return finalString;
}
//--------------------------------------------

inline QString Tools::fromSnakeCaseToCamelCase(const QString &string)
{
    QString finalString;
    bool next_letter_must_be_upper = false;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character == QChar::fromLatin1('_'))
        {
            next_letter_must_be_upper = true;
            continue;
        }
        else if (next_letter_must_be_upper)
        {
            finalString.append(character.toUpper());
            next_letter_must_be_upper = false;
        }
        else
        {
            finalString.append(character.toLower());
        }
    }
    return finalString;
}

//--------------------------------------------

inline const char *Tools::qtMetaTypeToSqlType(int qtMetaType)
{
    switch (qtMetaType)
    {
    case QMetaType::Bool:
        return "BOOLEAN";
    case QMetaType::Int:
        return "INTEGER";
    case QMetaType::UInt:
        return "INTEGER";
    case QMetaType::LongLong:
        return "INTEGER";
    case QMetaType::ULongLong:
        return "INTEGER";
    case QMetaType::Float:
        return "REAL";
    case QMetaType::Double:
        return "REAL";
    case QMetaType::QString:
        return "TEXT";
    case QMetaType::QUuid:
        return "TEXT";
    case QMetaType::QDateTime:
        return "DATETIME";
    default:
        return nullptr;
    }
}
} // namespace DatabaseTest::Persistence::Database
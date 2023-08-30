#pragma once

#include "qleany/qleany_global.h"
#include <QList>
#include <QString>

namespace Qleany::Domain
{
Q_NAMESPACE_EXPORT(QLEANY_EXPORT)

enum FieldType
{
    Bool,
    Integer,
    Float,
    String,
    Uuid,
    DateTime,
    Entity
};
Q_ENUM_NS(FieldType);

enum RelationshipType
{
    OneToOne,
    OneToMany,
    ManyToMany
};
Q_ENUM_NS(RelationshipType);

enum RelationshipStrength
{
    Strong,
    Weak
};
Q_ENUM_NS(RelationshipStrength);

/// @brief RelationshipDirection
/// Forward: the relationship is defined in the current entity
/// Backward: the relationship is defined in the related entity
/// @note: this is used to determine the name of the relationship in the related
/// entity or the junction table name
enum RelationshipDirection
{
    Forward,
    Backward
};
Q_ENUM_NS(RelationshipDirection);

/// @brief RelationshipCardinality
/// One: the relationship is defined by a foreign key in the related entity
/// ManyOrdered: the relationship is defined by a junction table, a ManyToMany relationship couldn't be ordered
/// ManyUnordered: the relationship is defined by a foreign key in the related entity

enum RelationshipCardinality
{
    One,
    ManyOrdered,
    ManyUnordered
};
Q_ENUM_NS(RelationshipCardinality);

struct QLEANY_EXPORT RelationshipInfo
{
    int leftEntityId;
    QString leftEntityName;
    int rightEntityId;
    QString rightEntityName;
    QString fieldName;
    RelationshipType type;
    RelationshipStrength strength;
    RelationshipCardinality cardinality;
    RelationshipDirection direction;
};

struct QLEANY_EXPORT FieldInfo
{
    QString name;
    FieldType type;
    bool isPrimaryKey;
    bool isLinkedToAnotherEntity;
};

struct QLEANY_EXPORT EntitySchema
{
    int entityId;
    QString name;
    QList<RelationshipInfo> relationships;
    QList<FieldInfo> fields;
};

} // namespace Qleany::Domain

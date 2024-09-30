// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QList>
#include <QString>

namespace Simple::Entities
{
Q_NAMESPACE

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

struct RelationshipInfo
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

struct FieldInfo
{
    QString name;
    FieldType type;
    bool isPrimaryKey;
    bool isLinkedToAnotherEntity;
};

struct EntitySchema
{
    int entityId;
    QString name;
    QList<RelationshipInfo> relationships;
    QList<FieldInfo> fields;
};

} // namespace Simple::Entities
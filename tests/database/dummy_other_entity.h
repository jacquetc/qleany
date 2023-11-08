// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "qleany/domain/entity_schema.h"

using namespace Qleany::Domain;

namespace DatabaseTest::Domain
{

class DummyOtherEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    DummyOtherEntity() : DummyEntity(), m_name(QString())
    {
    }

    ~DummyOtherEntity()
    {
    }

    DummyOtherEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                     const QString &name)
        : DummyEntity(id, uuid, creationDate, updateDate), m_name(name)
    {
    }

    DummyOtherEntity(const DummyOtherEntity &other) : DummyEntity(other), m_name(other.m_name)
    {
    }

    static DatabaseTest::Domain::Entities::EntityEnum enumValue()
    {
        return DatabaseTest::Domain::Entities::EntityEnum::DummyOtherEntity;
    }

    DummyOtherEntity &operator=(const DummyOtherEntity &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
    }

    friend bool operator==(const DummyOtherEntity &lhs, const DummyOtherEntity &rhs);

    friend uint qHash(const DummyOtherEntity &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    static Qleany::Domain::EntitySchema schema;

  private:
    QString m_name;
};

inline bool operator==(const DummyOtherEntity &lhs, const DummyOtherEntity &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const DummyOtherEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

/// Schema for DummyOtherEntity entity
inline Qleany::Domain::EntitySchema DummyOtherEntity::schema = {
    DatabaseTest::Domain::Entities::EntityEnum::DummyOtherEntity,
    "DummyOtherEntity",

    // relationships:
    {{DatabaseTest::Domain::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign",
      DatabaseTest::Domain::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity", "unique",
      RelationshipType::OneToOne, RelationshipStrength::Weak, RelationshipCardinality::One,
      RelationshipDirection::Backward},
     {DatabaseTest::Domain::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign",
      DatabaseTest::Domain::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity", "unorderedList",
      RelationshipType::OneToMany, RelationshipStrength::Weak, RelationshipCardinality::ManyUnordered,
      RelationshipDirection::Backward},
     {DatabaseTest::Domain::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign",
      DatabaseTest::Domain::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity", "orderedList",
      RelationshipType::OneToMany, RelationshipStrength::Weak, RelationshipCardinality::ManyOrdered,
      RelationshipDirection::Backward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"name", FieldType::String, false, false}}};

} // namespace DatabaseTest::Domain
Q_DECLARE_METATYPE(DatabaseTest::Domain::DummyOtherEntity)

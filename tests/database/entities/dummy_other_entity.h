// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "entity_schema.h"

namespace DatabaseTest::Entities
{

class DummyOtherEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    struct MetaData
    {
        MetaData(DummyOtherEntity *entity) : m_entity(entity)
        {
        }
        MetaData(DummyOtherEntity *entity, const MetaData &other) : m_entity(entity)
        {

            Q_UNUSED(other);
        }

        // Getters for the fields' metadata. Normal fields are always set, but lazy-loaded fields may not be
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "name"_L1)
            {
                return true;
            }
            // If the field is not found, we delegate to the parent class
            return m_entity->DummyEntity::metaData().getSet(fieldName);
        }

        // Getters for the fields' metadata. Normal fields are always set, but lazy-loaded fields may not be
        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "name"_L1)
            {
                return true;
            }
            // If the field is not found, we delegate to the parent class
            return m_entity->DummyEntity::metaData().getLoaded(fieldName);
        }

      private:
        DummyOtherEntity *m_entity = nullptr;
    };

    DummyOtherEntity() : DummyEntity(), m_metaData(this), m_name(QString())
    {
    }

    ~DummyOtherEntity()
    {
    }

    DummyOtherEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                     const QString &name)
        : DummyEntity(id, uuid, creationDate, updateDate), m_metaData(this), m_name(name)
    {
    }

    DummyOtherEntity(const DummyOtherEntity &other)
        : DummyEntity(other), m_metaData(other.m_metaData), m_name(other.m_name)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static DatabaseTest::Entities::Entities::EntityEnum enumValue()
    {
        return DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity;
    }

    DummyOtherEntity &operator=(const DummyOtherEntity &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;

            m_metaData = MetaData(this, other.metaData());
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

    static DatabaseTest::Entities::EntitySchema::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_name;
};

inline bool operator==(const DummyOtherEntity &lhs, const DummyOtherEntity &rhs)
{

    return static_cast<const DatabaseTest::Entities::DummyEntity &>(lhs) ==
               static_cast<const DatabaseTest::Entities::DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const DummyOtherEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DatabaseTest::Entities::DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

/// Schema for DummyOtherEntity entity
inline DatabaseTest::Entities::EntitySchema::EntitySchema DummyOtherEntity::schema = {
    DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity,
    "DummyOtherEntity"_L1,

    // relationships:
    {{DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "unique"_L1,
      EntitySchema::RelationshipType::OneToOne, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::One, EntitySchema::RelationshipDirection::Backward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "unorderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::ManyUnordered, EntitySchema::RelationshipDirection::Backward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "orderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::ManyOrdered, EntitySchema::RelationshipDirection::Backward}},

    // fields:
    {{"id"_L1, EntitySchema::FieldType::Integer, true, false},
     {"uuid"_L1, EntitySchema::FieldType::Uuid, false, false},
     {"creationDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"updateDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"name"_L1, EntitySchema::FieldType::String, false, false}}};

} // namespace DatabaseTest::Entities
Q_DECLARE_METATYPE(DatabaseTest::Entities::DummyOtherEntity)
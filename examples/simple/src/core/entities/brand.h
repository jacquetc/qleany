// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "entities.h"
#include "entity.h"
#include "entity_schema.h"

namespace Simple::Entities
{

class Brand : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    struct MetaData
    {
        MetaData(Brand *entity) : m_entity(entity)
        {
        }
        MetaData(Brand *entity, const MetaData &other) : m_entity(entity)
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
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        // Getters for the fields' metadata. Normal fields are always set, but lazy-loaded fields may not be
        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "name"_L1)
            {
                return true;
            }
            // If the field is not found, we delegate to the parent class
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Brand *m_entity = nullptr;
    };

    Brand() : Entity(), m_metaData(this), m_name(QString())
    {
    }

    ~Brand()
    {
    }

    Brand(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
          const QString &name)
        : Entity(id, uuid, creationDate, updateDate), m_metaData(this), m_name(name)
    {
    }

    Brand(const Brand &other) : Entity(other), m_metaData(other.m_metaData), m_name(other.m_name)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Simple::Entities::Entities::EntityEnum enumValue()
    {
        return Simple::Entities::Entities::EntityEnum::Brand;
    }

    Brand &operator=(const Brand &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Brand &lhs, const Brand &rhs);

    friend uint qHash(const Brand &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    static Simple::Entities::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_name;
};

inline bool operator==(const Brand &lhs, const Brand &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const Brand &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

/// Schema for Brand entity
inline Simple::Entities::EntitySchema Brand::schema = {
    Simple::Entities::Entities::EntityEnum::Brand,
    "Brand"_L1,

    // relationships:
    {{Simple::Entities::Entities::EntityEnum::Car, "Car"_L1, Simple::Entities::Entities::EntityEnum::Brand, "Brand"_L1,
      "brand"_L1, RelationshipType::OneToOne, RelationshipStrength::Strong, RelationshipCardinality::One,
      RelationshipDirection::Backward}},

    // fields:
    {{"id"_L1, FieldType::Integer, true, false},
     {"uuid"_L1, FieldType::Uuid, false, false},
     {"creationDate"_L1, FieldType::DateTime, false, false},
     {"updateDate"_L1, FieldType::DateTime, false, false},
     {"name"_L1, FieldType::String, false, false}}};

} // namespace Simple::Entities
Q_DECLARE_METATYPE(Simple::Entities::Brand)
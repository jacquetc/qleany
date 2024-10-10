// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "dummy_other_entity.h"
#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "entity_schema.h"

namespace DatabaseTest::Entities
{

class DummyEntityWithForeign : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(DummyOtherEntity unique READ unique WRITE setUnique)

    Q_PROPERTY(QList<DummyOtherEntity> unorderedList READ unorderedList WRITE setUnorderedList)

    Q_PROPERTY(QList<DummyOtherEntity> orderedList READ orderedList WRITE setOrderedList)

  public:
    struct MetaData
    {
        MetaData(DummyEntityWithForeign *entity) : m_entity(entity)
        {
        }
        MetaData(DummyEntityWithForeign *entity, const MetaData &other) : m_entity(entity)
        {
            this->uniqueSet = other.uniqueSet;
            this->uniqueLoaded = other.uniqueLoaded;
            this->unorderedListSet = other.unorderedListSet;
            this->unorderedListLoaded = other.unorderedListLoaded;
            this->orderedListSet = other.orderedListSet;
            this->orderedListLoaded = other.orderedListLoaded;
        }

        bool uniqueSet = false;
        bool uniqueLoaded = false;

        bool unorderedListSet = false;
        bool unorderedListLoaded = false;

        bool orderedListSet = false;
        bool orderedListLoaded = false;

        // Getters for the fields' metadata. Normal fields are always set, but lazy-loaded fields may not be
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "name"_L1)
            {
                return true;
            }
            if (fieldName == "unique"_L1)
            {
                return uniqueSet;
            }
            if (fieldName == "unorderedList"_L1)
            {
                return unorderedListSet;
            }
            if (fieldName == "orderedList"_L1)
            {
                return orderedListSet;
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
            if (fieldName == "unique"_L1)
            {
                return uniqueLoaded;
            }
            if (fieldName == "unorderedList"_L1)
            {
                return unorderedListLoaded;
            }
            if (fieldName == "orderedList"_L1)
            {
                return orderedListLoaded;
            }
            // If the field is not found, we delegate to the parent class
            return m_entity->DummyEntity::metaData().getLoaded(fieldName);
        }

      private:
        DummyEntityWithForeign *m_entity = nullptr;
    };

    DummyEntityWithForeign() : DummyEntity(), m_metaData(this), m_name(QString())
    {
    }

    ~DummyEntityWithForeign()
    {
    }

    DummyEntityWithForeign(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                           const QString &name, const DummyOtherEntity &unique,
                           const QList<DummyOtherEntity> &unorderedList, const QList<DummyOtherEntity> &orderedList)
        : DummyEntity(id, uuid, creationDate, updateDate), m_metaData(this), m_name(name), m_unique(unique),
          m_unorderedList(unorderedList), m_orderedList(orderedList)
    {
    }

    DummyEntityWithForeign(const DummyEntityWithForeign &other)
        : DummyEntity(other), m_metaData(other.m_metaData), m_name(other.m_name), m_unique(other.m_unique),
          m_unorderedList(other.m_unorderedList), m_orderedList(other.m_orderedList)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static DatabaseTest::Entities::Entities::EntityEnum enumValue()
    {
        return DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign;
    }

    DummyEntityWithForeign &operator=(const DummyEntityWithForeign &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
            m_unique = other.m_unique;
            m_unorderedList = other.m_unorderedList;
            m_orderedList = other.m_orderedList;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const DummyEntityWithForeign &lhs, const DummyEntityWithForeign &rhs);

    friend uint qHash(const DummyEntityWithForeign &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ unique : -----

    DummyOtherEntity unique()
    {
        if (!m_metaData.uniqueLoaded && m_uniqueLoader)
        {
            m_unique = m_uniqueLoader(this->id());
            m_metaData.uniqueLoaded = true;
        }
        return m_unique;
    }

    void setUnique(const DummyOtherEntity &unique)
    {
        m_unique = unique;

        m_metaData.uniqueSet = true;
    }

    using UniqueLoader = std::function<DummyOtherEntity(int entityId)>;

    void setUniqueLoader(const UniqueLoader &loader)
    {
        m_uniqueLoader = loader;
    }

    // ------ unorderedList : -----

    QList<DummyOtherEntity> unorderedList()
    {
        if (!m_metaData.unorderedListLoaded && m_unorderedListLoader)
        {
            m_unorderedList = m_unorderedListLoader(this->id());
            m_metaData.unorderedListLoaded = true;
        }
        return m_unorderedList;
    }

    void setUnorderedList(const QList<DummyOtherEntity> &unorderedList)
    {
        m_unorderedList = unorderedList;

        m_metaData.unorderedListSet = true;
    }

    using UnorderedListLoader = std::function<QList<DummyOtherEntity>(int entityId)>;

    void setUnorderedListLoader(const UnorderedListLoader &loader)
    {
        m_unorderedListLoader = loader;
    }

    // ------ orderedList : -----

    QList<DummyOtherEntity> orderedList()
    {
        if (!m_metaData.orderedListLoaded && m_orderedListLoader)
        {
            m_orderedList = m_orderedListLoader(this->id());
            m_metaData.orderedListLoaded = true;
        }
        return m_orderedList;
    }

    void setOrderedList(const QList<DummyOtherEntity> &orderedList)
    {
        m_orderedList = orderedList;

        m_metaData.orderedListSet = true;
    }

    using OrderedListLoader = std::function<QList<DummyOtherEntity>(int entityId)>;

    void setOrderedListLoader(const OrderedListLoader &loader)
    {
        m_orderedListLoader = loader;
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
    DummyOtherEntity m_unique;
    UniqueLoader m_uniqueLoader;
    QList<DummyOtherEntity> m_unorderedList;
    UnorderedListLoader m_unorderedListLoader;
    QList<DummyOtherEntity> m_orderedList;
    OrderedListLoader m_orderedListLoader;
};

inline bool operator==(const DummyEntityWithForeign &lhs, const DummyEntityWithForeign &rhs)
{

    return static_cast<const DatabaseTest::Entities::DummyEntity &>(lhs) ==
               static_cast<const DatabaseTest::Entities::DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_unique == rhs.m_unique && lhs.m_unorderedList == rhs.m_unorderedList &&
           lhs.m_orderedList == rhs.m_orderedList;
}

inline uint qHash(const DummyEntityWithForeign &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DatabaseTest::Entities::DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_unique, seed);
    hash ^= ::qHash(entity.m_unorderedList, seed);
    hash ^= ::qHash(entity.m_orderedList, seed);

    return hash;
}

/// Schema for DummyEntityWithForeign entity
inline DatabaseTest::Entities::EntitySchema::EntitySchema DummyEntityWithForeign::schema = {
    DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign,
    "DummyEntityWithForeign"_L1,

    // relationships:
    {{DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "unique"_L1,
      EntitySchema::RelationshipType::OneToOne, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::One, EntitySchema::RelationshipDirection::Forward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "unorderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::ManyUnordered, EntitySchema::RelationshipDirection::Forward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "orderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak,
      EntitySchema::RelationshipCardinality::ManyOrdered, EntitySchema::RelationshipDirection::Forward}},

    // fields:
    {{"id"_L1, EntitySchema::FieldType::Integer, true, false},
     {"uuid"_L1, EntitySchema::FieldType::Uuid, false, false},
     {"creationDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"updateDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"name"_L1, EntitySchema::FieldType::String, false, false},
     {"unique"_L1, EntitySchema::FieldType::Entity, false, true},
     {"unorderedList"_L1, EntitySchema::FieldType::Entity, false, true},
     {"orderedList"_L1, EntitySchema::FieldType::Entity, false, true}}};

} // namespace DatabaseTest::Entities
Q_DECLARE_METATYPE(DatabaseTest::Entities::DummyEntityWithForeign)
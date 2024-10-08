// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "dummy_entity.h"
#include "dummy_other_entity.h"
#include "entities.h"
#include "entity_schema.h"
#include <QString>


namespace DatabaseTest::Entities
{

class DummyEntityWithForeign : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(DummyOtherEntity unique READ unique WRITE setUnique)

    Q_PROPERTY(bool uniqueLoaded MEMBER m_uniqueLoaded)

    Q_PROPERTY(QList<DummyOtherEntity> unorderedList READ unorderedList WRITE setUnorderedList)

    Q_PROPERTY(bool unorderedListLoaded MEMBER m_unorderedListLoaded)

    Q_PROPERTY(QList<DummyOtherEntity> orderedList READ orderedList WRITE setOrderedList)

    Q_PROPERTY(bool orderedListLoaded MEMBER m_orderedListLoaded)

  public:
    DummyEntityWithForeign() : DummyEntity(), m_name(QString())
    {
    }

    ~DummyEntityWithForeign()
    {
    }

    DummyEntityWithForeign(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                           const QString &name, const DummyOtherEntity &unique,
                           const QList<DummyOtherEntity> &unorderedList, const QList<DummyOtherEntity> &orderedList)
        : DummyEntity(id, uuid, creationDate, updateDate), m_name(name), m_unique(unique),
          m_unorderedList(unorderedList), m_orderedList(orderedList)
    {
    }

    DummyEntityWithForeign(const DummyEntityWithForeign &other)
        : DummyEntity(other), m_name(other.m_name), m_unique(other.m_unique), m_uniqueLoaded(other.m_uniqueLoaded),
          m_unorderedList(other.m_unorderedList), m_unorderedListLoaded(other.m_unorderedListLoaded),
          m_orderedList(other.m_orderedList), m_orderedListLoaded(other.m_orderedListLoaded)
    {
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
            m_uniqueLoaded = other.m_uniqueLoaded;
            m_unorderedList = other.m_unorderedList;
            m_unorderedListLoaded = other.m_unorderedListLoaded;
            m_orderedList = other.m_orderedList;
            m_orderedListLoaded = other.m_orderedListLoaded;
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
        if (!m_uniqueLoaded && m_uniqueLoader)
        {
            m_unique = m_uniqueLoader(this->id());
            m_uniqueLoaded = true;
        }
        return m_unique;
    }

    void setUnique(const DummyOtherEntity &unique)
    {
        m_unique = unique;
    }

    using UniqueLoader = std::function<DummyOtherEntity(int entityId)>;

    void setUniqueLoader(const UniqueLoader &loader)
    {
        m_uniqueLoader = loader;
    }

    // ------ unorderedList : -----

    QList<DummyOtherEntity> unorderedList()
    {
        if (!m_unorderedListLoaded && m_unorderedListLoader)
        {
            m_unorderedList = m_unorderedListLoader(this->id());
            m_unorderedListLoaded = true;
        }
        return m_unorderedList;
    }

    void setUnorderedList(const QList<DummyOtherEntity> &unorderedList)
    {
        m_unorderedList = unorderedList;
    }

    using UnorderedListLoader = std::function<QList<DummyOtherEntity>(int entityId)>;

    void setUnorderedListLoader(const UnorderedListLoader &loader)
    {
        m_unorderedListLoader = loader;
    }

    // ------ orderedList : -----

    QList<DummyOtherEntity> orderedList()
    {
        if (!m_orderedListLoaded && m_orderedListLoader)
        {
            m_orderedList = m_orderedListLoader(this->id());
            m_orderedListLoaded = true;
        }
        return m_orderedList;
    }

    void setOrderedList(const QList<DummyOtherEntity> &orderedList)
    {
        m_orderedList = orderedList;
    }

    using OrderedListLoader = std::function<QList<DummyOtherEntity>(int entityId)>;

    void setOrderedListLoader(const OrderedListLoader &loader)
    {
        m_orderedListLoader = loader;
    }

    static DatabaseTest::Entities::EntitySchema::EntitySchema schema;

  private:
    QString m_name;
    DummyOtherEntity m_unique;
    UniqueLoader m_uniqueLoader;
    bool m_uniqueLoaded = false;
    QList<DummyOtherEntity> m_unorderedList;
    UnorderedListLoader m_unorderedListLoader;
    bool m_unorderedListLoaded = false;
    QList<DummyOtherEntity> m_orderedList;
    OrderedListLoader m_orderedListLoader;
    bool m_orderedListLoaded = false;
};

inline bool operator==(const DummyEntityWithForeign &lhs, const DummyEntityWithForeign &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_unique == rhs.m_unique && lhs.m_unorderedList == rhs.m_unorderedList &&
           lhs.m_orderedList == rhs.m_orderedList;
}

inline uint qHash(const DummyEntityWithForeign &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

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
      EntitySchema::RelationshipType::OneToOne, EntitySchema::RelationshipStrength::Weak, EntitySchema::RelationshipCardinality::One,
      EntitySchema::RelationshipDirection::Forward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "unorderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak, EntitySchema::RelationshipCardinality::ManyUnordered,
      EntitySchema::RelationshipDirection::Forward},
     {DatabaseTest::Entities::Entities::EntityEnum::DummyEntityWithForeign, "DummyEntityWithForeign"_L1,
      DatabaseTest::Entities::Entities::EntityEnum::DummyOtherEntity, "DummyOtherEntity"_L1, "orderedList"_L1,
      EntitySchema::RelationshipType::OneToMany, EntitySchema::RelationshipStrength::Weak, EntitySchema::RelationshipCardinality::ManyOrdered,
      EntitySchema::RelationshipDirection::Forward}},

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

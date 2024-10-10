// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "entity_schema.h"

namespace DatabaseTest::Entities
{

class DummyBasicEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QString author READ author WRITE setAuthor)

  public:
    struct MetaData
    {
        MetaData(DummyBasicEntity *entity) : m_entity(entity)
        {
        }
        MetaData(DummyBasicEntity *entity, const MetaData &other) : m_entity(entity)
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
            if (fieldName == "author"_L1)
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
            if (fieldName == "author"_L1)
            {
                return true;
            }
            // If the field is not found, we delegate to the parent class
            return m_entity->DummyEntity::metaData().getLoaded(fieldName);
        }

      private:
        DummyBasicEntity *m_entity = nullptr;
    };

    DummyBasicEntity() : DummyEntity(), m_metaData(this), m_name(QString()), m_author(QString())
    {
    }

    ~DummyBasicEntity()
    {
    }

    DummyBasicEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                     const QString &name, const QString &author)
        : DummyEntity(id, uuid, creationDate, updateDate), m_metaData(this), m_name(name), m_author(author)
    {
    }

    DummyBasicEntity(const DummyBasicEntity &other)
        : DummyEntity(other), m_metaData(other.m_metaData), m_name(other.m_name), m_author(other.m_author)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static DatabaseTest::Entities::Entities::EntityEnum enumValue()
    {
        return DatabaseTest::Entities::Entities::EntityEnum::DummyBasicEntity;
    }

    DummyBasicEntity &operator=(const DummyBasicEntity &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
            m_author = other.m_author;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const DummyBasicEntity &lhs, const DummyBasicEntity &rhs);

    friend uint qHash(const DummyBasicEntity &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ author : -----

    QString author() const
    {

        return m_author;
    }

    void setAuthor(const QString &author)
    {
        m_author = author;
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
    QString m_author;
};

inline bool operator==(const DummyBasicEntity &lhs, const DummyBasicEntity &rhs)
{

    return static_cast<const DatabaseTest::Entities::DummyEntity &>(lhs) ==
               static_cast<const DatabaseTest::Entities::DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_author == rhs.m_author;
}

inline uint qHash(const DummyBasicEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DatabaseTest::Entities::DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_author, seed);

    return hash;
}

/// Schema for DummyBasicEntity entity
inline DatabaseTest::Entities::EntitySchema::EntitySchema DummyBasicEntity::schema = {
    DatabaseTest::Entities::Entities::EntityEnum::DummyBasicEntity,
    "DummyBasicEntity"_L1,

    // relationships:
    {

    },

    // fields:
    {{"id"_L1, EntitySchema::FieldType::Integer, true, false},
     {"uuid"_L1, EntitySchema::FieldType::Uuid, false, false},
     {"creationDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"updateDate"_L1, EntitySchema::FieldType::DateTime, false, false},
     {"name"_L1, EntitySchema::FieldType::String, false, false},
     {"author"_L1, EntitySchema::FieldType::String, false, false}}};

} // namespace DatabaseTest::Entities
Q_DECLARE_METATYPE(DatabaseTest::Entities::DummyBasicEntity)
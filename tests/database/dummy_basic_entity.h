// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "examples/simple/src/core/entities/entity_schema.h"

using namespace Qleany::Entities;

namespace DatabaseTest::Entities
{

class DummyBasicEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QString author READ author WRITE setAuthor)

  public:
    DummyBasicEntity() : DummyEntity(), m_name(QString()), m_author(QString())
    {
    }

    ~DummyBasicEntity()
    {
    }

    DummyBasicEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                     const QString &name, const QString &author)
        : DummyEntity(id, uuid, creationDate, updateDate), m_name(name), m_author(author)
    {
    }

    DummyBasicEntity(const DummyBasicEntity &other) : DummyEntity(other), m_name(other.m_name), m_author(other.m_author)
    {
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

    static Qleany::Entities::EntitySchema schema;

  private:
    QString m_name;
    QString m_author;
};

inline bool operator==(const DummyBasicEntity &lhs, const DummyBasicEntity &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_author == rhs.m_author;
}

inline uint qHash(const DummyBasicEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_author, seed);

    return hash;
}

/// Schema for DummyBasicEntity entity
inline Qleany::Entities::EntitySchema DummyBasicEntity::schema = {
    DatabaseTest::Entities::Entities::EntityEnum::DummyBasicEntity,
    "DummyBasicEntity"_L1,

    // relationships:
    {

    },

    // fields:
    {{"id"_L1, FieldType::Integer, true, false},
     {"uuid"_L1, FieldType::Uuid, false, false},
     {"creationDate"_L1, FieldType::DateTime, false, false},
     {"updateDate"_L1, FieldType::DateTime, false, false},
     {"name"_L1, FieldType::String, false, false},
     {"author"_L1, FieldType::String, false, false}}};

} // namespace DatabaseTest::Entities
Q_DECLARE_METATYPE(DatabaseTest::Entities::DummyBasicEntity)

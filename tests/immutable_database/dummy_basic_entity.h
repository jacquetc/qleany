// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "dummy_entity.h"
#include "entities.h"
#include "qleany/domain/entity_schema.h"

using namespace Qleany::Domain;

namespace ImmutableDatabaseTest::Domain
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

    static ImmutableDatabaseTest::Domain::Entities::EntityEnum enumValue()
    {
        return ImmutableDatabaseTest::Domain::Entities::EntityEnum::DummyBasicEntity;
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

    static Qleany::Domain::EntitySchema schema;

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
inline Qleany::Domain::EntitySchema DummyBasicEntity::schema = {
    ImmutableDatabaseTest::Domain::Entities::EntityEnum::DummyBasicEntity,
    "DummyBasicEntity",

    // relationships:
    {

    },

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"name", FieldType::String, false, false},
     {"author", FieldType::String, false, false}}};

} // namespace DatabaseTest::Domain
Q_DECLARE_METATYPE(ImmutableDatabaseTest::Domain::DummyBasicEntity)

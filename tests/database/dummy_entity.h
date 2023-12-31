// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QUuid>

#include "entities.h"
#include "qleany/domain/entity_base.h"
#include "qleany/domain/entity_schema.h"

using namespace Qleany::Domain;

namespace DatabaseTest::Domain
{

class DummyEntity : public EntityBase
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)

    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)

    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)

  public:
    DummyEntity() : EntityBase(), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime())
    {
    }

    ~DummyEntity()
    {
    }

    DummyEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate)
        : EntityBase(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate)
    {
    }

    DummyEntity(const DummyEntity &other)
        : EntityBase(other), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate)
    {
    }

    static DatabaseTest::Domain::Entities::EntityEnum enumValue()
    {
        return DatabaseTest::Domain::Entities::EntityEnum::DummyEntity;
    }

    DummyEntity &operator=(const DummyEntity &other)
    {
        if (this != &other)
        {
            EntityBase::operator=(other);
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
        }
        return *this;
    }

    friend bool operator==(const DummyEntity &lhs, const DummyEntity &rhs);

    friend uint qHash(const DummyEntity &entity, uint seed) noexcept;

    // ------ uuid : -----

    QUuid uuid() const
    {

        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {

        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {

        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }

    static Qleany::Domain::EntitySchema schema;

  private:
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
};

inline bool operator==(const DummyEntity &lhs, const DummyEntity &rhs)
{

    return static_cast<const EntityBase &>(lhs) == static_cast<const EntityBase &>(rhs) &&

           lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate && lhs.m_updateDate == rhs.m_updateDate;
}

inline uint qHash(const DummyEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const EntityBase &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_uuid, seed);
    hash ^= ::qHash(entity.m_creationDate, seed);
    hash ^= ::qHash(entity.m_updateDate, seed);

    return hash;
}

/// Schema for DummyEntity entity
inline Qleany::Domain::EntitySchema DummyEntity::schema = {DatabaseTest::Domain::Entities::EntityEnum::DummyEntity,
                                                           "DummyEntity",

                                                           // relationships:
                                                           {

                                                           },

                                                           // fields:
                                                           {{"id", FieldType::Integer, true, false},
                                                            {"uuid", FieldType::Uuid, false, false},
                                                            {"creationDate", FieldType::DateTime, false, false},
                                                            {"updateDate", FieldType::DateTime, false, false}}};

} // namespace DatabaseTest::Domain
Q_DECLARE_METATYPE(DatabaseTest::Domain::DummyEntity)

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "domain_export.h"
#include <QString>

#include "entities.h"
#include "entity.h"

using namespace Qleany::Domain;

namespace Simple::Domain
{

class SIMPLEEXAMPLE_DOMAIN_EXPORT Passenger : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    Passenger() : Entity(), m_name(QString())
    {
    }

    ~Passenger()
    {
    }

    Passenger(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
              const QString &name)
        : Entity(id, uuid, creationDate, updateDate), m_name(name)
    {
    }

    Passenger(const Passenger &other) : Entity(other), m_name(other.m_name)
    {
    }

    static Simple::Domain::Entities::EntityEnum enumValue()
    {
        return Simple::Domain::Entities::EntityEnum::Passenger;
    }

    Passenger &operator=(const Passenger &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
    }

    friend bool operator==(const Passenger &lhs, const Passenger &rhs);

    friend uint qHash(const Passenger &entity, uint seed) noexcept;

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

inline bool operator==(const Passenger &lhs, const Passenger &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const Passenger &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

/// Schema for Passenger entity
inline Qleany::Domain::EntitySchema Passenger::schema = {
    Simple::Domain::Entities::EntityEnum::Passenger,
    "Passenger",

    // relationships:
    {{Simple::Domain::Entities::EntityEnum::Car, "Car", Simple::Domain::Entities::EntityEnum::Passenger, "Passenger",
      "passengers", RelationshipType::OneToMany, RelationshipStrength::Strong, RelationshipCardinality::ManyOrdered,
      RelationshipDirection::Backward},
     {Simple::Domain::Entities::EntityEnum::Client, "Client", Simple::Domain::Entities::EntityEnum::Passenger,
      "Passenger", "client", RelationshipType::OneToOne, RelationshipStrength::Weak, RelationshipCardinality::One,
      RelationshipDirection::Backward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"name", FieldType::String, false, false}}};

} // namespace Simple::Domain
Q_DECLARE_METATYPE(Simple::Domain::Passenger)

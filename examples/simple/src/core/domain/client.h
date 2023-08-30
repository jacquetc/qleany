// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "domain_export.h"
#include "passenger.h"

#include "entities.h"
#include "entity.h"

using namespace Qleany::Domain;

namespace Simple::Domain
{

class SIMPLEEXAMPLE_DOMAIN_EXPORT Client : public Entity
{
    Q_GADGET

    Q_PROPERTY(Passenger client READ client WRITE setClient)

    Q_PROPERTY(bool clientLoaded MEMBER m_clientLoaded)

  public:
    Client() : Entity()
    {
    }

    ~Client()
    {
    }

    Client(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
           const Passenger &client)
        : Entity(id, uuid, creationDate, updateDate), m_client(client)
    {
    }

    Client(const Client &other) : Entity(other), m_client(other.m_client), m_clientLoaded(other.m_clientLoaded)
    {
    }

    static Simple::Domain::Entities::EntityEnum enumValue()
    {
        return Simple::Domain::Entities::EntityEnum::Client;
    }

    Client &operator=(const Client &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_client = other.m_client;
            m_clientLoaded = other.m_clientLoaded;
        }
        return *this;
    }

    friend bool operator==(const Client &lhs, const Client &rhs);

    friend uint qHash(const Client &entity, uint seed) noexcept;

    // ------ client : -----

    Passenger client()
    {
        if (!m_clientLoaded && m_clientLoader)
        {
            m_client = m_clientLoader(this->id());
            m_clientLoaded = true;
        }
        return m_client;
    }

    void setClient(const Passenger &client)
    {
        m_client = client;

        m_clientSet = true;
    }

    using ClientLoader = std::function<Passenger(int entityId)>;

    void setClientLoader(const ClientLoader &loader)
    {
        m_clientLoader = loader;
    }

    bool clientSet() const
    {
        return m_clientSet;
    }

    static Qleany::Domain::EntitySchema schema;

  private:
    Passenger m_client;
    ClientLoader m_clientLoader;
    bool m_clientLoaded = false;
    bool m_clientSet = false;
};

inline bool operator==(const Client &lhs, const Client &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_client == rhs.m_client;
}

inline uint qHash(const Client &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_client, seed);

    return hash;
}

/// Schema for Client entity
inline Qleany::Domain::EntitySchema Client::schema = {
    Simple::Domain::Entities::EntityEnum::Client,
    "Client",

    // relationships:
    {{Simple::Domain::Entities::EntityEnum::Client, "Client", Simple::Domain::Entities::EntityEnum::Passenger,
      "Passenger", "client", RelationshipType::OneToOne, RelationshipStrength::Weak, RelationshipCardinality::One,
      RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"client", FieldType::Entity, false, true}}};

} // namespace Simple::Domain
Q_DECLARE_METATYPE(Simple::Domain::Client)

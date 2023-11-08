// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger.h"

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Simple::Domain
{

class Client : public Entity
{
    Q_GADGET

    Q_PROPERTY(Passenger client READ client WRITE setClient)

    Q_PROPERTY(QList<Passenger> clientFriends READ clientFriends WRITE setClientFriends)

  public:
    struct MetaData
    {
        MetaData(Client *entity) : m_entity(entity)
        {
        }
        MetaData(Client *entity, const MetaData &other) : m_entity(entity)
        {
            this->clientSet = other.clientSet;
            this->clientLoaded = other.clientLoaded;
            this->clientFriendsSet = other.clientFriendsSet;
            this->clientFriendsLoaded = other.clientFriendsLoaded;
        }

        bool clientSet = false;
        bool clientLoaded = false;

        bool clientFriendsSet = false;
        bool clientFriendsLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "client")
            {
                return clientSet;
            }
            if (fieldName == "clientFriends")
            {
                return clientFriendsSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "client")
            {
                return clientLoaded;
            }
            if (fieldName == "clientFriends")
            {
                return clientFriendsLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Client *m_entity = nullptr;
    };

    Client() : Entity(), m_metaData(this)
    {
    }

    ~Client()
    {
    }

    Client(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
           const Passenger &client, const QList<Passenger> &clientFriends)
        : Entity(id, uuid, creationDate, updateDate), m_client(client), m_clientFriends(clientFriends), m_metaData(this)
    {
    }

    Client(const Client &other)
        : Entity(other), m_metaData(other.m_metaData), m_client(other.m_client), m_clientFriends(other.m_clientFriends)
    {
        m_metaData = MetaData(this, other.metaData());
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
            m_clientFriends = other.m_clientFriends;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Client &lhs, const Client &rhs);

    friend uint qHash(const Client &entity, uint seed) noexcept;

    // ------ client : -----

    Passenger client()
    {
        if (!m_metaData.clientLoaded && m_clientLoader)
        {
            m_client = m_clientLoader(this->id());
            m_metaData.clientLoaded = true;
        }
        return m_client;
    }

    void setClient(const Passenger &client)
    {
        m_client = client;

        m_metaData.clientSet = true;
    }

    using ClientLoader = std::function<Passenger(int entityId)>;

    void setClientLoader(const ClientLoader &loader)
    {
        m_clientLoader = loader;
    }

    // ------ clientFriends : -----

    QList<Passenger> clientFriends()
    {
        if (!m_metaData.clientFriendsLoaded && m_clientFriendsLoader)
        {
            m_clientFriends = m_clientFriendsLoader(this->id());
            m_metaData.clientFriendsLoaded = true;
        }
        return m_clientFriends;
    }

    void setClientFriends(const QList<Passenger> &clientFriends)
    {
        m_clientFriends = clientFriends;

        m_metaData.clientFriendsSet = true;
    }

    using ClientFriendsLoader = std::function<QList<Passenger>(int entityId)>;

    void setClientFriendsLoader(const ClientFriendsLoader &loader)
    {
        m_clientFriendsLoader = loader;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    Passenger m_client;
    ClientLoader m_clientLoader;
    QList<Passenger> m_clientFriends;
    ClientFriendsLoader m_clientFriendsLoader;
};

inline bool operator==(const Client &lhs, const Client &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_client == rhs.m_client && lhs.m_clientFriends == rhs.m_clientFriends;
}

inline uint qHash(const Client &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_client, seed);
    hash ^= ::qHash(entity.m_clientFriends, seed);

    return hash;
}

/// Schema for Client entity
inline Qleany::Domain::EntitySchema Client::schema = {
    Simple::Domain::Entities::EntityEnum::Client,
    "Client",

    // relationships:
    {{Simple::Domain::Entities::EntityEnum::Client, "Client", Simple::Domain::Entities::EntityEnum::Passenger,
      "Passenger", "client", RelationshipType::OneToOne, RelationshipStrength::Weak, RelationshipCardinality::One,
      RelationshipDirection::Forward},
     {Simple::Domain::Entities::EntityEnum::Client, "Client", Simple::Domain::Entities::EntityEnum::Passenger,
      "Passenger", "clientFriends", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyUnordered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"client", FieldType::Entity, false, true},
     {"clientFriends", FieldType::Entity, false, true}}};

} // namespace Simple::Domain
Q_DECLARE_METATYPE(Simple::Domain::Client)
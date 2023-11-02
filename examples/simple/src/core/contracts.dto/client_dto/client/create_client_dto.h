// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include <QDateTime>
#include <QObject>
#include <QUuid>

using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Contracts::DTO::Client
{

class CreateClientDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(PassengerDTO client READ client WRITE setClient)
    Q_PROPERTY(QList<PassengerDTO> clientFriends READ clientFriends WRITE setClientFriends)

  public:
    struct MetaData
    {
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool clientSet = false;
        bool clientFriendsSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "uuid")
            {
                return uuidSet;
            }
            if (fieldName == "creationDate")
            {
                return creationDateSet;
            }
            if (fieldName == "updateDate")
            {
                return updateDateSet;
            }
            if (fieldName == "client")
            {
                return clientSet;
            }
            if (fieldName == "clientFriends")
            {
                return clientFriendsSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (clientSet)
                return true;

            if (clientFriendsSet)
                return true;

            return false;
        }
    };

    CreateClientDTO() : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime())
    {
    }

    ~CreateClientDTO()
    {
    }

    CreateClientDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                    const PassengerDTO &client, const QList<PassengerDTO> &clientFriends)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_client(client),
          m_clientFriends(clientFriends)
    {
    }

    CreateClientDTO(const CreateClientDTO &other)
        : m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_client(other.m_client), m_clientFriends(other.m_clientFriends)
    {
    }

    CreateClientDTO &operator=(const CreateClientDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_client = other.m_client;
            m_clientFriends = other.m_clientFriends;
        }
        return *this;
    }

    friend bool operator==(const CreateClientDTO &lhs, const CreateClientDTO &rhs);

    friend uint qHash(const CreateClientDTO &dto, uint seed) noexcept;

    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
        m_metaData.uuidSet = true;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
        m_metaData.creationDateSet = true;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
        m_metaData.updateDateSet = true;
    }

    // ------ client : -----

    PassengerDTO client() const
    {
        return m_client;
    }

    void setClient(const PassengerDTO &client)
    {
        m_client = client;
        m_metaData.clientSet = true;
    }

    // ------ clientFriends : -----

    QList<PassengerDTO> clientFriends() const
    {
        return m_clientFriends;
    }

    void setClientFriends(const QList<PassengerDTO> &clientFriends)
    {
        m_clientFriends = clientFriends;
        m_metaData.clientFriendsSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    PassengerDTO m_client;
    QList<PassengerDTO> m_clientFriends;
};

inline bool operator==(const CreateClientDTO &lhs, const CreateClientDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_client == rhs.m_client &&
           lhs.m_clientFriends == rhs.m_clientFriends;
}

inline uint qHash(const CreateClientDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_client, seed);
    hash ^= ::qHash(dto.m_clientFriends, seed);

    return hash;
}

} // namespace Simple::Contracts::DTO::Client
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Client::CreateClientDTO)
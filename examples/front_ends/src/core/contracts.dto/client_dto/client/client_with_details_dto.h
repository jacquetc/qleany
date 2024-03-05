// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include <QDateTime>
#include <QObject>
#include <QUuid>

using namespace FrontEnds::Contracts::DTO::Passenger;
using namespace FrontEnds::Contracts::DTO::Passenger;

namespace FrontEnds::Contracts::DTO::Client
{

class ClientWithDetailsDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(PassengerDTO client READ client WRITE setClient)
    Q_PROPERTY(QList<PassengerDTO> clientFriends READ clientFriends WRITE setClientFriends)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool clientSet = false;
        bool clientFriendsSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "id")
            {
                return idSet;
            }
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

    ClientWithDetailsDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime())
    {
    }

    ~ClientWithDetailsDTO()
    {
    }

    ClientWithDetailsDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                         const PassengerDTO &client, const QList<PassengerDTO> &clientFriends)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_client(client),
          m_clientFriends(clientFriends)
    {
    }

    ClientWithDetailsDTO(const ClientWithDetailsDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_client(other.m_client), m_clientFriends(other.m_clientFriends)
    {
    }

    Q_INVOKABLE bool isValid() const
    {
        return m_id > 0;
    }

    Q_INVOKABLE bool isNull() const
    {
        return !isValid();
    }

    Q_INVOKABLE bool isInvalid() const
    {
        return !isValid();
    }

    ClientWithDetailsDTO &operator=(const ClientWithDetailsDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_client = other.m_client;
            m_clientFriends = other.m_clientFriends;
        }
        return *this;
    }

    ClientWithDetailsDTO &operator=(const ClientWithDetailsDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_client = other.m_client;
            m_clientFriends = other.m_clientFriends;
        }
        return *this;
    }

    ClientWithDetailsDTO &mergeWith(const ClientWithDetailsDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.idSet)
            {
                m_id = other.m_id;
                m_metaData.idSet = true;
            }
            if (other.m_metaData.uuidSet)
            {
                m_uuid = other.m_uuid;
                m_metaData.uuidSet = true;
            }
            if (other.m_metaData.creationDateSet)
            {
                m_creationDate = other.m_creationDate;
                m_metaData.creationDateSet = true;
            }
            if (other.m_metaData.updateDateSet)
            {
                m_updateDate = other.m_updateDate;
                m_metaData.updateDateSet = true;
            }
            if (other.m_metaData.clientSet)
            {
                m_client = other.m_client;
                m_metaData.clientSet = true;
            }
            if (other.m_metaData.clientFriendsSet)
            {
                m_clientFriends = other.m_clientFriends;
                m_metaData.clientFriendsSet = true;
            }
        }
        return *this;
    }

    // import operator
    ClientWithDetailsDTO &operator<<(const ClientWithDetailsDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const ClientWithDetailsDTO &lhs, const ClientWithDetailsDTO &rhs);

    friend uint qHash(const ClientWithDetailsDTO &dto, uint seed) noexcept;

    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
        m_metaData.idSet = true;
    }

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

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    PassengerDTO m_client;
    QList<PassengerDTO> m_clientFriends;
};

inline bool operator==(const ClientWithDetailsDTO &lhs, const ClientWithDetailsDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_client == rhs.m_client &&
           lhs.m_clientFriends == rhs.m_clientFriends;
}

inline uint qHash(const ClientWithDetailsDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_client, seed);
    hash ^= ::qHash(dto.m_clientFriends, seed);

    return hash;
}

} // namespace FrontEnds::Contracts::DTO::Client
Q_DECLARE_METATYPE(FrontEnds::Contracts::DTO::Client::ClientWithDetailsDTO)
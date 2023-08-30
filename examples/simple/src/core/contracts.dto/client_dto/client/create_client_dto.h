// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include "passenger/passenger_dto.h"
#include <QDateTime>
#include <QUuid>


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

  public:
    CreateClientDTO() : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime())
    {
    }

    ~CreateClientDTO()
    {
    }

    CreateClientDTO( const QUuid &uuid,   const QDateTime &creationDate,   const QDateTime &updateDate,   const PassengerDTO &client ) 
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_client(client)
    {
    }

    CreateClientDTO(const CreateClientDTO &other) : m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate), m_client(other.m_client)
    {
    }

    CreateClientDTO &operator=(const CreateClientDTO &other)
    {
        if (this != &other)
        {
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_client = other.m_client;
            
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

    void setUuid( const QUuid &uuid)
    {
        m_uuid = uuid;
    }
    

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate( const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }
    

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate( const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }
    

    // ------ client : -----

    PassengerDTO client() const
    {
        return m_client;
    }

    void setClient( const PassengerDTO &client)
    {
        m_client = client;
    }
    


  private:

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    PassengerDTO m_client;
};

inline bool operator==(const CreateClientDTO &lhs, const CreateClientDTO &rhs)
{

    return 
            lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  && lhs.m_updateDate == rhs.m_updateDate  && lhs.m_client == rhs.m_client 
    ;
}

inline uint qHash(const CreateClientDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_uuid, seed);
        hash ^= ::qHash(dto.m_creationDate, seed);
        hash ^= ::qHash(dto.m_updateDate, seed);
        hash ^= ::qHash(dto.m_client, seed);
        

        return hash;
}

} // namespace Simple::Contracts::DTO::Client
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Client::CreateClientDTO)
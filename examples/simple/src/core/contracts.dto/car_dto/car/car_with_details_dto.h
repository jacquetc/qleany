// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include "brand/brand_dto.h"
#include "passenger/passenger_dto.h"
#include <QDateTime>
#include <QString>
#include <QUuid>


using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::DTO::Passenger;


namespace Simple::Contracts::DTO::Car
{

class CarWithDetailsDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)
    Q_PROPERTY(BrandDTO brand READ brand WRITE setBrand)
    Q_PROPERTY(QList<PassengerDTO> passengers READ passengers WRITE setPassengers)

  public:
    CarWithDetailsDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_content(QString())
    {
    }

    ~CarWithDetailsDTO()
    {
    }

    CarWithDetailsDTO( int id,   const QUuid &uuid,   const QDateTime &creationDate,   const QDateTime &updateDate,   const QString &content,   const BrandDTO &brand,   const QList<PassengerDTO> &passengers ) 
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content), m_brand(brand), m_passengers(passengers)
    {
    }

    CarWithDetailsDTO(const CarWithDetailsDTO &other) : m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate), m_content(other.m_content), m_brand(other.m_brand), m_passengers(other.m_passengers)
    {
    }

    CarWithDetailsDTO &operator=(const CarWithDetailsDTO &other)
    {
        if (this != &other)
        {
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            m_brand = other.m_brand;
            m_passengers = other.m_passengers;
            
        }
        return *this;
    }

    friend bool operator==(const CarWithDetailsDTO &lhs, const CarWithDetailsDTO &rhs);


    friend uint qHash(const CarWithDetailsDTO &dto, uint seed) noexcept;



    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId( int id)
    {
        m_id = id;
    }
    

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
    

    // ------ content : -----

    QString content() const
    {
        return m_content;
    }

    void setContent( const QString &content)
    {
        m_content = content;
    }
    

    // ------ brand : -----

    BrandDTO brand() const
    {
        return m_brand;
    }

    void setBrand( const BrandDTO &brand)
    {
        m_brand = brand;
    }
    

    // ------ passengers : -----

    QList<PassengerDTO> passengers() const
    {
        return m_passengers;
    }

    void setPassengers( const QList<PassengerDTO> &passengers)
    {
        m_passengers = passengers;
    }
    


  private:

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
    BrandDTO m_brand;
    QList<PassengerDTO> m_passengers;
};

inline bool operator==(const CarWithDetailsDTO &lhs, const CarWithDetailsDTO &rhs)
{

    return 
            lhs.m_id == rhs.m_id  && lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  && lhs.m_updateDate == rhs.m_updateDate  && lhs.m_content == rhs.m_content  && lhs.m_brand == rhs.m_brand  && lhs.m_passengers == rhs.m_passengers 
    ;
}

inline uint qHash(const CarWithDetailsDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_id, seed);
        hash ^= ::qHash(dto.m_uuid, seed);
        hash ^= ::qHash(dto.m_creationDate, seed);
        hash ^= ::qHash(dto.m_updateDate, seed);
        hash ^= ::qHash(dto.m_content, seed);
        hash ^= ::qHash(dto.m_brand, seed);
        hash ^= ::qHash(dto.m_passengers, seed);
        

        return hash;
}

} // namespace Simple::Contracts::DTO::Car
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Car::CarWithDetailsDTO)